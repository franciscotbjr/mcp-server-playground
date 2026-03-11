//! Streamable HTTP transport handlers — `POST /mcp`, `GET /mcp`, `DELETE /mcp`.
//!
//! Implements MCP spec 2025-03-26 Streamable HTTP transport.
//! All JSON-RPC traffic flows through a single `/mcp` endpoint.

use super::app_state::AppState;
use super::session::{Session, SessionState, SessionStore};
use crate::mcp::protocol::{JsonRpcError, JsonRpcRequest, JsonRpcResponse};

use axum::extract::State;
use axum::http::{HeaderMap, StatusCode};
use axum::response::sse::{Event, KeepAlive, Sse};
use axum::response::IntoResponse;
use std::convert::Infallible;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tokio_stream::StreamExt;
use tracing::{debug, error, info, warn};
use uuid::Uuid;

/// Header name for the MCP session identifier.
const MCP_SESSION_ID: &str = "mcp-session-id";

// ---------------------------------------------------------------------------
// POST /mcp — client sends JSON-RPC, server responds with JSON
// ---------------------------------------------------------------------------

pub(crate) async fn handle_post_mcp(
    State(state): State<AppState>,
    headers: HeaderMap,
    body: String,
) -> impl IntoResponse {
    // --- Header validation ---
    if !header_contains(&headers, "content-type", "application/json") {
        return error_response(
            StatusCode::UNSUPPORTED_MEDIA_TYPE,
            "Content-Type must be application/json",
        );
    }

    if !header_contains(&headers, "accept", "application/json") {
        return error_response(
            StatusCode::NOT_ACCEPTABLE,
            "Accept must include application/json",
        );
    }

    debug!("POST /mcp body={body}");

    // --- Parse body (single object or batch array) ---
    let parsed: serde_json::Value = match serde_json::from_str(&body) {
        Ok(v) => v,
        Err(e) => {
            error!("Failed to parse JSON-RPC: {e}");
            let err = JsonRpcResponse::error(
                serde_json::Value::Null,
                JsonRpcError::parse_error(e.to_string()),
            );
            return json_response(StatusCode::OK, None, &serde_json::to_value(&err).unwrap());
        }
    };

    let is_batch = parsed.is_array();
    let messages: Vec<serde_json::Value> = if is_batch {
        parsed.as_array().unwrap().clone()
    } else {
        vec![parsed]
    };

    // --- Session ID from header ---
    let session_id = headers
        .get(MCP_SESSION_ID)
        .and_then(|v| v.to_str().ok())
        .map(String::from);

    // Does the batch contain an `initialize` request?
    let has_initialize = messages.iter().any(|m| {
        m.get("method").and_then(|v| v.as_str()) == Some("initialize")
    });

    // If not initializing, require a valid session
    if !has_initialize {
        match &session_id {
            None => {
                return error_response(
                    StatusCode::BAD_REQUEST,
                    "Missing Mcp-Session-Id header",
                );
            }
            Some(sid) => {
                let exists = state.sessions.lock().await.contains_key(sid);
                if !exists {
                    warn!("Unknown session_id={sid}");
                    return error_response(StatusCode::NOT_FOUND, "Session not found");
                }
            }
        }
    }

    // --- Process messages ---
    let mut responses: Vec<JsonRpcResponse> = Vec::new();
    let mut new_session_id: Option<String> = None;

    for msg_value in &messages {
        let request: JsonRpcRequest = match serde_json::from_value(msg_value.clone()) {
            Ok(req) => req,
            Err(e) => {
                responses.push(JsonRpcResponse::error(
                    serde_json::Value::Null,
                    JsonRpcError::parse_error(e.to_string()),
                ));
                continue;
            }
        };

        let method = request.method.clone();
        let is_notification = request.id.is_none();

        if method == "initialize" {
            // Create a new session
            let sid = Uuid::new_v4().to_string();
            let (tx, rx) = mpsc::channel::<String>(64);

            {
                let mut sessions = state.sessions.lock().await;
                sessions.insert(
                    sid.clone(),
                    Session {
                        state: SessionState::Initializing,
                        tx,
                        sse_rx: Some(rx),
                        sse_active: false,
                    },
                );
            }

            info!("Session created: {sid}");
            new_session_id = Some(sid);

            let response = state.handler.handle(&request).await;
            if !is_notification {
                responses.push(response);
            }
        } else {
            // Determine which session ID to use
            let sid = session_id.as_deref().or(new_session_id.as_deref());
            let sid = match sid {
                Some(s) => s,
                None => {
                    if !is_notification {
                        responses.push(JsonRpcResponse::error(
                            request.id.unwrap_or(serde_json::Value::Null),
                            JsonRpcError::invalid_request("No session"),
                        ));
                    }
                    continue;
                }
            };

            // Enforce lifecycle
            if let Some(err) = check_lifecycle(&state.sessions, sid, &request).await {
                if !is_notification {
                    responses.push(err);
                }
                continue;
            }

            // Handle state transitions
            if method == "notifications/initialized" {
                let mut sessions = state.sessions.lock().await;
                if let Some(session) = sessions.get_mut(sid) {
                    session.state = SessionState::Ready;
                    info!("Session {sid} → Ready");
                }
                // Notifications produce no response
                continue;
            }

            // Dispatch via handler
            let response = state.handler.handle(&request).await;
            if !is_notification {
                responses.push(response);
            }
        }
    }

    // --- Build HTTP response ---
    let mut resp_headers = HeaderMap::new();
    if let Some(ref sid) = new_session_id {
        resp_headers.insert(MCP_SESSION_ID, sid.parse().unwrap());
    }

    // All notifications → 202
    if responses.is_empty() {
        return (StatusCode::ACCEPTED, resp_headers, String::new());
    }

    // Single vs batch
    let body_value = if is_batch {
        serde_json::to_value(&responses).unwrap()
    } else {
        serde_json::to_value(&responses[0]).unwrap()
    };

    json_response(StatusCode::OK, Some(&resp_headers), &body_value)
}

// ---------------------------------------------------------------------------
// GET /mcp — open passive SSE stream for server→client push
// ---------------------------------------------------------------------------

pub(crate) async fn handle_get_mcp(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<
    Sse<impl tokio_stream::Stream<Item = std::result::Result<Event, Infallible>>>,
    (StatusCode, String),
> {
    // Validate Accept
    if !header_contains(&headers, "accept", "text/event-stream") {
        return Err((
            StatusCode::NOT_ACCEPTABLE,
            "Accept must be text/event-stream".to_string(),
        ));
    }

    // Require session
    let session_id = headers
        .get(MCP_SESSION_ID)
        .and_then(|v| v.to_str().ok())
        .map(String::from);

    let session_id = match session_id {
        Some(s) => s,
        None => {
            return Err((
                StatusCode::BAD_REQUEST,
                "Missing Mcp-Session-Id header".to_string(),
            ))
        }
    };

    // Take the receiver from the session
    let rx = {
        let mut sessions = state.sessions.lock().await;
        match sessions.get_mut(&session_id) {
            None => {
                return Err((StatusCode::NOT_FOUND, "Session not found".to_string()));
            }
            Some(session) => {
                if session.sse_active {
                    return Err((
                        StatusCode::CONFLICT,
                        "SSE stream already active for this session".to_string(),
                    ));
                }
                match session.sse_rx.take() {
                    Some(rx) => {
                        session.sse_active = true;
                        rx
                    }
                    None => {
                        return Err((
                            StatusCode::CONFLICT,
                            "SSE stream already claimed".to_string(),
                        ));
                    }
                }
            }
        }
    };

    info!("SSE stream opened, session_id={session_id}");

    // Clone references for cleanup task
    let session_id_for_cleanup = session_id.clone();
    let sessions_for_cleanup = state.sessions.clone();
    let mut shutdown_rx = state.shutdown.clone();

    let tx_clone = {
        let sessions = state.sessions.lock().await;
        sessions.get(&session_id).map(|s| s.tx.clone())
    };

    let message_stream = ReceiverStream::new(rx).map(|msg| {
        std::result::Result::<Event, Infallible>::Ok(Event::default().event("message").data(msg))
    });

    // Detect client disconnect OR server shutdown
    if let Some(tx_for_cleanup) = tx_clone {
        tokio::spawn(async move {
            let is_client_disconnect = tokio::select! {
                _ = tx_for_cleanup.closed() => true,
                _ = shutdown_rx.changed() => false,
            };
            drop(tx_for_cleanup);

            if is_client_disconnect {
                info!("SSE stream closed, session_id={session_id_for_cleanup}");
                let mut sessions = sessions_for_cleanup.lock().await;
                if let Some(session) = sessions.get_mut(&session_id_for_cleanup) {
                    session.sse_active = false;
                }
            } else {
                info!("SSE stream closed (shutdown), session_id={session_id_for_cleanup}");
            }
        });
    }

    Ok(Sse::new(message_stream).keep_alive(KeepAlive::default()))
}

// ---------------------------------------------------------------------------
// DELETE /mcp — terminate session
// ---------------------------------------------------------------------------

pub(crate) async fn handle_delete_mcp(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> impl IntoResponse {
    let session_id = headers
        .get(MCP_SESSION_ID)
        .and_then(|v| v.to_str().ok());

    let session_id = match session_id {
        Some(s) => s,
        None => {
            return (
                StatusCode::BAD_REQUEST,
                "Missing Mcp-Session-Id header".to_string(),
            )
        }
    };

    let mut sessions = state.sessions.lock().await;
    if sessions.remove(session_id).is_some() {
        info!("Session terminated: {session_id}");
        (StatusCode::OK, "Session terminated".to_string())
    } else {
        (StatusCode::NOT_FOUND, "Session not found".to_string())
    }
}

// ---------------------------------------------------------------------------
// Lifecycle enforcement
// ---------------------------------------------------------------------------

/// Check whether the request is allowed given the current session state.
/// Returns `Some(error_response)` if the request should be rejected,
/// or `None` if it is allowed to proceed.
pub(crate) async fn check_lifecycle(
    sessions: &SessionStore,
    session_id: &str,
    request: &JsonRpcRequest,
) -> Option<JsonRpcResponse> {
    let method = request.method.as_str();
    let session_state = {
        let sessions = sessions.lock().await;
        sessions.get(session_id).map(|s| s.state)
    };

    match session_state {
        Some(SessionState::Uninitialized) => {
            if method != "initialize" && method != "ping" {
                return Some(JsonRpcResponse::error(
                    request.id.clone().unwrap_or(serde_json::Value::Null),
                    JsonRpcError::invalid_request(
                        "Server not initialized. Send 'initialize' first.",
                    ),
                ));
            }
        }
        Some(SessionState::Initializing) => {
            if method != "notifications/initialized" && method != "ping" {
                return Some(JsonRpcResponse::error(
                    request.id.clone().unwrap_or(serde_json::Value::Null),
                    JsonRpcError::invalid_request(
                        "Initialization in progress. Send 'notifications/initialized' to complete.",
                    ),
                ));
            }
        }
        Some(SessionState::Ready) => {
            // All methods allowed
        }
        None => {
            return Some(JsonRpcResponse::error(
                request.id.clone().unwrap_or(serde_json::Value::Null),
                JsonRpcError::invalid_request("Session not found"),
            ));
        }
    }

    None
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Serialize a response and send it to the session's passive SSE channel.
#[allow(dead_code)]
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
            warn!("SSE channel closed for session_id={session_id}");
            return Err(());
        }
        Ok(())
    } else {
        warn!("Session {session_id} not found when sending");
        Err(())
    }
}

fn header_contains(headers: &HeaderMap, name: &str, value: &str) -> bool {
    headers
        .get(name)
        .and_then(|v| v.to_str().ok())
        .is_some_and(|v| v.to_lowercase().contains(value))
}

fn error_response(status: StatusCode, message: &str) -> (StatusCode, HeaderMap, String) {
    (status, HeaderMap::new(), message.to_string())
}

fn json_response(
    status: StatusCode,
    extra_headers: Option<&HeaderMap>,
    value: &serde_json::Value,
) -> (StatusCode, HeaderMap, String) {
    let mut headers = HeaderMap::new();
    headers.insert("content-type", "application/json".parse().unwrap());
    if let Some(extra) = extra_headers {
        for (k, v) in extra {
            headers.insert(k.clone(), v.clone());
        }
    }
    (status, headers, serde_json::to_string(value).unwrap())
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
            sessions.insert(
                session_id.clone(),
                Session {
                    state,
                    tx,
                    sse_rx: None,
                    sse_active: false,
                },
            );
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
    fn test_app_state_is_send_sync() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<AppState>();
    }

    #[tokio::test]
    async fn test_reject_before_initialize() {
        let (store, session_id, _rx) = setup_session(SessionState::Uninitialized).await;
        let request = make_request("tools/list");

        let result = check_lifecycle(&store, &session_id, &request).await;
        assert!(result.is_some(), "Should reject tools/list before initialize");

        let resp = result.unwrap();
        let err = resp.error.expect("Should be error response");
        assert_eq!(err.code, -32600);
        let detail = err.data.unwrap().as_str().unwrap().to_string();
        assert!(detail.contains("Server not initialized"));
    }

    #[tokio::test]
    async fn test_allow_initialize_when_uninitialized() {
        let (store, session_id, _rx) = setup_session(SessionState::Uninitialized).await;
        let request = make_request("initialize");

        let result = check_lifecycle(&store, &session_id, &request).await;
        assert!(result.is_none(), "Should allow initialize when Uninitialized");
    }

    #[tokio::test]
    async fn test_reject_during_initializing() {
        let (store, session_id, _rx) = setup_session(SessionState::Initializing).await;
        let request = make_request("tools/list");

        let result = check_lifecycle(&store, &session_id, &request).await;
        assert!(
            result.is_some(),
            "Should reject tools/list during Initializing"
        );

        let resp = result.unwrap();
        let err = resp.error.expect("Should be error response");
        assert_eq!(err.code, -32600);
        let detail = err.data.unwrap().as_str().unwrap().to_string();
        assert!(detail.contains("Initialization in progress"));
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

        let result = check_lifecycle(&store, &session_id, &request).await;
        assert!(
            result.is_none(),
            "Should allow notifications/initialized during Initializing"
        );
    }

    #[tokio::test]
    async fn test_ping_always_allowed() {
        for state in [
            SessionState::Uninitialized,
            SessionState::Initializing,
            SessionState::Ready,
        ] {
            let (store, session_id, _rx) = setup_session(state).await;
            let request = make_request("ping");

            let result = check_lifecycle(&store, &session_id, &request).await;
            assert!(result.is_none(), "ping should be allowed in state {state:?}");
        }
    }

    #[tokio::test]
    async fn test_all_methods_allowed_when_ready() {
        let (store, session_id, _rx) = setup_session(SessionState::Ready).await;

        for method in ["tools/list", "tools/call", "initialize", "anything"] {
            let request = make_request(method);
            let result = check_lifecycle(&store, &session_id, &request).await;
            assert!(result.is_none(), "{method} should be allowed when Ready");
        }
    }

    #[tokio::test]
    async fn test_unknown_session_returns_error() {
        let store: SessionStore = Arc::new(Mutex::new(HashMap::new()));
        let request = make_request("initialize");

        let result = check_lifecycle(&store, "nonexistent", &request).await;
        assert!(result.is_some());
        let resp = result.unwrap();
        let err = resp.error.expect("Should be error response");
        assert_eq!(err.code, -32600);
        let detail = err.data.unwrap().as_str().unwrap().to_string();
        assert!(detail.contains("Session not found"));
    }

    #[tokio::test]
    async fn test_send_to_session_delivers_message() {
        let (store, session_id, mut rx) = setup_session(SessionState::Ready).await;
        let response =
            JsonRpcResponse::success(serde_json::json!(1), serde_json::json!({"ok": true}));

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

    #[test]
    fn test_header_contains() {
        let mut headers = HeaderMap::new();
        headers.insert("content-type", "application/json".parse().unwrap());
        headers.insert(
            "accept",
            "application/json, text/event-stream".parse().unwrap(),
        );

        assert!(header_contains(&headers, "content-type", "application/json"));
        assert!(header_contains(&headers, "accept", "application/json"));
        assert!(header_contains(&headers, "accept", "text/event-stream"));
        assert!(!header_contains(&headers, "accept", "text/html"));
        assert!(!header_contains(&headers, "missing", "value"));
    }
}
