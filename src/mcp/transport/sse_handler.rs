//! SSE transport handlers — `GET /sse` and `POST /message` endpoints.

use super::app_state::AppState;
use super::message_query::MessageQuery;
use super::session::{Session, SessionState, SessionStore};
use crate::mcp::protocol::{JsonRpcError, JsonRpcRequest, JsonRpcResponse};

use axum::extract::{Query, State};
use axum::response::sse::{Event, KeepAlive, Sse};
use axum::response::IntoResponse;
use std::convert::Infallible;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tokio_stream::StreamExt;
use tracing::{debug, error, info, warn};
use uuid::Uuid;

// ---------------------------------------------------------------------------
// GET /sse — open SSE stream, send `endpoint` event
// ---------------------------------------------------------------------------

pub(crate) async fn handle_sse(
    State(state): State<AppState>,
) -> Sse<impl tokio_stream::Stream<Item = std::result::Result<Event, Infallible>>> {
    let session_id = Uuid::new_v4().to_string();
    let (tx, rx) = mpsc::channel::<String>(64);

    // Store session
    {
        let mut sessions = state.sessions.lock().await;
        sessions.insert(
            session_id.clone(),
            Session {
                state: SessionState::Uninitialized,
                tx,
            },
        );
    }

    info!("SSE connection opened, session_id={session_id}");

    // Build the SSE stream: first emit the `endpoint` event, then relay messages
    let endpoint_uri = format!("/message?sessionId={session_id}");
    let endpoint_event =
        std::result::Result::<Event, Infallible>::Ok(Event::default().event("endpoint").data(endpoint_uri));

    let session_id_for_cleanup = session_id.clone();
    let sessions_for_cleanup = state.sessions.clone();

    let message_stream = ReceiverStream::new(rx).map(|msg| {
        std::result::Result::<Event, Infallible>::Ok(Event::default().event("message").data(msg))
    });

    let stream = tokio_stream::once(endpoint_event).chain(message_stream);

    // Spawn a task to clean up the session when the stream ends
    tokio::spawn(async move {
        // This task relies on the receiver being dropped when the SSE connection closes.
        // The cleanup happens when the mpsc sender fails (connection gone).
        // We use a simple delay-then-check approach; actual cleanup also happens
        // in handle_message when sends fail.
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        debug!(
            "Session cleanup task started for session_id={}",
            session_id_for_cleanup
        );
        // The actual removal happens when send fails in handle_message
        let _ = sessions_for_cleanup;
    });

    Sse::new(stream).keep_alive(KeepAlive::default())
}

// ---------------------------------------------------------------------------
// POST /message?sessionId=<uuid> — receive JSON-RPC, dispatch, respond via SSE
// ---------------------------------------------------------------------------

pub(crate) async fn handle_message(
    State(state): State<AppState>,
    Query(query): Query<MessageQuery>,
    body: String,
) -> impl IntoResponse {
    let session_id = &query.session_id;

    debug!("POST /message session_id={session_id}, body={body}");

    // Validate session exists
    let session_exists = {
        let sessions = state.sessions.lock().await;
        sessions.contains_key(session_id)
    };

    if !session_exists {
        warn!("Unknown session_id={session_id}");
        return (
            axum::http::StatusCode::NOT_FOUND,
            "Session not found".to_string(),
        );
    }

    // Parse JSON-RPC request
    let request: JsonRpcRequest = match serde_json::from_str(&body) {
        Ok(req) => req,
        Err(e) => {
            error!("Failed to parse JSON-RPC request: {e}");
            let err_response = JsonRpcResponse::error(
                serde_json::Value::Null,
                JsonRpcError::parse_error(e.to_string()),
            );
            let _ = send_to_session(&state.sessions, session_id, &err_response).await;
            return (axum::http::StatusCode::ACCEPTED, String::new());
        }
    };

    // Enforce lifecycle
    let method = request.method.as_str();
    let enforcement = enforce_lifecycle(&state.sessions, session_id, &request).await;
    if let Some(response_tuple) = enforcement {
        return response_tuple;
    }

    // Transition session state based on method
    if method == "initialize" {
        let mut sessions = state.sessions.lock().await;
        if let Some(session) = sessions.get_mut(session_id) {
            session.state = SessionState::Initializing;
            debug!("Session {session_id} → Initializing");
        }
    } else if method == "notifications/initialized" {
        let mut sessions = state.sessions.lock().await;
        if let Some(session) = sessions.get_mut(session_id) {
            session.state = SessionState::Ready;
            info!("Session {session_id} → Ready");
        }
        // Notifications don't produce a response
        return (axum::http::StatusCode::ACCEPTED, String::new());
    }

    // Dispatch via handler
    let response = state.handler.handle(&request).await;

    // Notifications (no id) don't produce a response
    let is_notification = request.id.is_none();
    if !is_notification {
        let _ = send_to_session(&state.sessions, session_id, &response).await;
    }

    (axum::http::StatusCode::ACCEPTED, String::new())
}

// ---------------------------------------------------------------------------
// Lifecycle enforcement
// ---------------------------------------------------------------------------

/// Check if the request is allowed given the current session state.
/// Returns `Some((StatusCode, String))` if the request should be rejected,
/// or `None` if it is allowed to proceed.
pub(crate) async fn enforce_lifecycle(
    sessions: &SessionStore,
    session_id: &str,
    request: &JsonRpcRequest,
) -> Option<(axum::http::StatusCode, String)> {
    let method = request.method.as_str();
    let session_state = {
        let sessions = sessions.lock().await;
        sessions.get(session_id).map(|s| s.state)
    };

    match session_state {
        Some(SessionState::Uninitialized) => {
            if method != "initialize" && method != "ping" {
                let id = request.id.clone().unwrap_or(serde_json::Value::Null);
                let err_response = JsonRpcResponse::error(
                    id,
                    JsonRpcError::invalid_request("Server not initialized. Send 'initialize' first."),
                );
                let _ = send_to_session(sessions, session_id, &err_response).await;
                return Some((axum::http::StatusCode::ACCEPTED, String::new()));
            }
        }
        Some(SessionState::Initializing) => {
            if method != "notifications/initialized" && method != "ping" {
                let id = request.id.clone().unwrap_or(serde_json::Value::Null);
                let err_response = JsonRpcResponse::error(
                    id,
                    JsonRpcError::invalid_request(
                        "Initialization in progress. Send 'notifications/initialized' to complete.",
                    ),
                );
                let _ = send_to_session(sessions, session_id, &err_response).await;
                return Some((axum::http::StatusCode::ACCEPTED, String::new()));
            }
        }
        Some(SessionState::Ready) => {
            // All methods allowed
        }
        None => {
            return Some((
                axum::http::StatusCode::NOT_FOUND,
                "Session not found".to_string(),
            ));
        }
    }

    None
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Serialize a response and send it to the session's SSE channel.
pub(crate) async fn send_to_session(
    sessions: &SessionStore,
    session_id: &str,
    response: &JsonRpcResponse,
) -> std::result::Result<(), ()> {
    let json = serde_json::to_string(response).map_err(|e| {
        error!("Failed to serialize response: {e}");
    })?;

    let tx = {
        let sessions = sessions.lock().await;
        sessions.get(session_id).map(|s| s.tx.clone())
    };

    if let Some(tx) = tx {
        if tx.send(json).await.is_err() {
            warn!("SSE channel closed for session_id={session_id}, removing session");
            let mut sessions = sessions.lock().await;
            sessions.remove(session_id);
            return Err(());
        }
        Ok(())
    } else {
        warn!("Session {session_id} not found when sending response");
        Err(())
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use std::sync::Arc;
    use tokio::sync::Mutex;

    /// Helper: create a SessionStore with one session in the given state,
    /// returning (store, session_id, rx).
    async fn setup_session(
        state: SessionState,
    ) -> (SessionStore, String, mpsc::Receiver<String>) {
        let (tx, rx) = mpsc::channel(64);
        let session_id = "test-session-id".to_string();
        let store: SessionStore = Arc::new(Mutex::new(HashMap::new()));
        {
            let mut sessions = store.lock().await;
            sessions.insert(session_id.clone(), Session { state, tx });
        }
        (store, session_id, rx)
    }

    fn make_request(method: &str) -> JsonRpcRequest {
        JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            id: Some(serde_json::json!(1)),
            method: method.to_string(),
            params: None,
        }
    }

    #[test]
    fn test_message_query_deserialize() {
        let json = r#"{"sessionId": "abc-123"}"#;
        let query: MessageQuery = serde_json::from_str(json).unwrap();
        assert_eq!(query.session_id, "abc-123");
    }

    #[test]
    fn test_app_state_is_send_sync() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<AppState>();
    }

    #[tokio::test]
    async fn test_reject_before_initialize() {
        let (store, session_id, mut rx) = setup_session(SessionState::Uninitialized).await;
        let request = make_request("tools/list");

        let result = enforce_lifecycle(&store, &session_id, &request).await;
        assert!(result.is_some(), "Should reject tools/list before initialize");

        // Verify error was sent to SSE channel
        let msg = rx.try_recv().unwrap();
        assert!(msg.contains("Server not initialized"));
    }

    #[tokio::test]
    async fn test_allow_initialize_when_uninitialized() {
        let (store, session_id, _rx) = setup_session(SessionState::Uninitialized).await;
        let request = make_request("initialize");

        let result = enforce_lifecycle(&store, &session_id, &request).await;
        assert!(result.is_none(), "Should allow initialize when Uninitialized");
    }

    #[tokio::test]
    async fn test_reject_during_initializing() {
        let (store, session_id, mut rx) = setup_session(SessionState::Initializing).await;
        let request = make_request("tools/list");

        let result = enforce_lifecycle(&store, &session_id, &request).await;
        assert!(result.is_some(), "Should reject tools/list during Initializing");

        let msg = rx.try_recv().unwrap();
        assert!(msg.contains("Initialization in progress"));
    }

    #[tokio::test]
    async fn test_allow_initialized_notification() {
        let (store, session_id, _rx) = setup_session(SessionState::Initializing).await;
        let request = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            id: None,
            method: "notifications/initialized".to_string(),
            params: None,
        };

        let result = enforce_lifecycle(&store, &session_id, &request).await;
        assert!(result.is_none(), "Should allow notifications/initialized during Initializing");
    }

    #[tokio::test]
    async fn test_ping_always_allowed() {
        for state in [SessionState::Uninitialized, SessionState::Initializing, SessionState::Ready] {
            let (store, session_id, _rx) = setup_session(state).await;
            let request = make_request("ping");

            let result = enforce_lifecycle(&store, &session_id, &request).await;
            assert!(result.is_none(), "ping should be allowed in state {state:?}");
        }
    }

    #[tokio::test]
    async fn test_all_methods_allowed_when_ready() {
        let (store, session_id, _rx) = setup_session(SessionState::Ready).await;

        for method in ["tools/list", "tools/call", "initialize", "anything"] {
            let request = make_request(method);
            let result = enforce_lifecycle(&store, &session_id, &request).await;
            assert!(result.is_none(), "{method} should be allowed when Ready");
        }
    }

    #[tokio::test]
    async fn test_unknown_session_returns_not_found() {
        let store: SessionStore = Arc::new(Mutex::new(HashMap::new()));
        let request = make_request("initialize");

        let result = enforce_lifecycle(&store, "nonexistent", &request).await;
        assert!(result.is_some());
        let (status, body) = result.unwrap();
        assert_eq!(status, axum::http::StatusCode::NOT_FOUND);
        assert!(body.contains("Session not found"));
    }

    #[tokio::test]
    async fn test_send_to_session_delivers_message() {
        let (store, session_id, mut rx) = setup_session(SessionState::Ready).await;
        let response = JsonRpcResponse::success(serde_json::json!(1), serde_json::json!({"ok": true}));

        let result = send_to_session(&store, &session_id, &response).await;
        assert!(result.is_ok());

        let msg = rx.try_recv().unwrap();
        assert!(msg.contains("\"ok\":true") || msg.contains("\"ok\": true"));
    }

    #[tokio::test]
    async fn test_send_to_session_unknown_session() {
        let store: SessionStore = Arc::new(Mutex::new(HashMap::new()));
        let response = JsonRpcResponse::success(serde_json::json!(1), serde_json::json!({}));

        let result = send_to_session(&store, "nonexistent", &response).await;
        assert!(result.is_err());
    }
}
