//! MCP protocol layer — JSON-RPC types, MCP types, tools, handler, SSE transport, and server.

pub mod protocol;
pub mod tools;
pub mod transport;
mod handler;

pub use handler::RequestHandler;
pub use protocol::{
    JsonRpcError, JsonRpcNotification, JsonRpcRequest, JsonRpcResponse,
};
pub use tools::{
    CallToolParams, CallToolResult, Content, InputSchema, ListToolsResult,
    McpTool, ToolDefinition, ToolRegistry,
};
pub use transport::{
    InitializeResult, McpServer, ServerCapabilities, ServerInfo, SessionState, ToolsCapability,
};
