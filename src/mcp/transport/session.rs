//! MCP session management — tracks the lifecycle state of each client connection.

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{mpsc, Mutex};

/// State of an MCP session during its lifecycle.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SessionState {
    /// SSE connection established, waiting for `initialize` request.
    #[default]
    Uninitialized,
    /// `initialize` request received, waiting for `notifications/initialized`.
    Initializing,
    /// Fully initialized — normal operation.
    Ready,
}

/// A single client session.
#[derive(Debug)]
pub(crate) struct Session {
    pub(crate) state: SessionState,
    /// Channel sender for server→client push via passive SSE stream.
    pub(crate) tx: mpsc::Sender<String>,
    /// Channel receiver — held until `GET /mcp` claims it for the SSE stream.
    pub(crate) sse_rx: Option<mpsc::Receiver<String>>,
    /// Whether a passive SSE stream (`GET /mcp`) is currently open.
    pub(crate) sse_active: bool,
}

/// Thread-safe store of all active sessions.
pub(crate) type SessionStore = Arc<Mutex<HashMap<String, Session>>>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_session_state_default() {
        let (tx, rx) = mpsc::channel(1);
        let session = Session {
            state: SessionState::Uninitialized,
            tx,
            sse_rx: Some(rx),
            sse_active: false,
        };
        assert_eq!(session.state, SessionState::Uninitialized);
        assert!(!session.sse_active);
        assert!(session.sse_rx.is_some());
    }

    #[test]
    fn test_session_state_transitions() {
        assert_ne!(SessionState::Uninitialized, SessionState::Initializing);
        assert_ne!(SessionState::Initializing, SessionState::Ready);
        assert_ne!(SessionState::Uninitialized, SessionState::Ready);
    }

    #[test]
    fn test_session_store_is_send_sync() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<SessionStore>();
    }
}
