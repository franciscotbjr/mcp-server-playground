//! SSE transport layer — HTTP server, session management, endpoint handlers.

mod app_state;
mod message_query;
mod server;
mod session;
mod sse_handler;

pub use server::McpServer;
pub use session::SessionState;
