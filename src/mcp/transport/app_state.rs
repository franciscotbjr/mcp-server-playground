//! Shared application state for the MCP server.

use crate::mcp::handler::RequestHandler;
use super::session::SessionStore;
use std::sync::Arc;

#[derive(Clone)]
pub(crate) struct AppState {
    pub(crate) sessions: SessionStore,
    pub(crate) handler: Arc<RequestHandler>,
}
