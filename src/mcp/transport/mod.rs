//! SSE transport layer — HTTP server, session management, endpoint handlers, server identity.

mod app_state;
mod initialize_result;
mod message_query;
mod server;
mod server_capabilities;
mod server_info;
mod session;
mod sse_handler;
mod tools_capability;

pub use initialize_result::InitializeResult;
pub use server::McpServer;
pub use server_capabilities::ServerCapabilities;
pub use server_info::ServerInfo;
pub use session::SessionState;
pub use tools_capability::ToolsCapability;
