//! MCP protocol layer — JSON-RPC types, MCP types, tools, handler, SSE transport, and server.

pub mod protocol;
pub mod tools;
pub mod transport;
mod handler;

pub use handler::RequestHandler;
pub use protocol::{
    JsonRpcError, JsonRpcNotification, JsonRpcRequest, JsonRpcResponse,
    CallToolParams, CallToolResult, Content, InitializeResult, InputSchema, ListToolsResult,
    ServerCapabilities, ServerInfo, ToolDefinition, ToolsCapability,
};
pub use tools::{McpTool, ToolRegistry};
pub use transport::{McpServer, SessionState};
