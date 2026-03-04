//! MCP Server Playground — a Rust MCP server with calendar and contacts tools.

pub mod error;
pub mod mcp;

// Re-exports for ergonomic access
pub use error::{Error, Result};
pub use mcp::{
    CallToolParams, CallToolResult, Content, InitializeResult, InputSchema, JsonRpcError,
    JsonRpcNotification, JsonRpcRequest, JsonRpcResponse, ListToolsResult, McpServer, McpTool,
    RequestHandler, ServerCapabilities, ServerInfo, SessionState, ToolDefinition, ToolRegistry,
    ToolsCapability,
};
