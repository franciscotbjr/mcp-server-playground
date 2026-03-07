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

/// A single client session backed by an SSE connection.
#[derive(Debug)]
pub(crate) struct Session {
    pub(crate) state: SessionState,
    pub(crate) tx: mpsc::Sender<String>,
}

/// Thread-safe store of all active sessions.
pub(crate) type SessionStore = Arc<Mutex<HashMap<String, Session>>>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_session_state_default() {
        let (tx, _rx) = mpsc::channel(1);
        let session = Session {
            state: SessionState::Uninitialized,
            tx,
        };
        assert_eq!(session.state, SessionState::Uninitialized);
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
