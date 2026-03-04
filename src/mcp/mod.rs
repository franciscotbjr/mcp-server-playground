//! MCP protocol layer — JSON-RPC types, MCP types, tools, handler, session, SSE transport, and server.

mod app_state;
mod handler;
mod message_query;
mod protocol;
mod server;
mod session;
mod sse_handler;
mod tool_trait;
mod tool_registry;
mod types;

pub use handler::RequestHandler;
pub use protocol::{JsonRpcError, JsonRpcNotification, JsonRpcRequest, JsonRpcResponse};
pub use server::McpServer;
pub use session::SessionState;
pub use tool_trait::McpTool;
pub use tool_registry::ToolRegistry;
pub use types::{
    CallToolParams, CallToolResult, Content, InitializeResult, InputSchema, ListToolsResult,
    ServerCapabilities, ServerInfo, ToolDefinition, ToolsCapability,
};
