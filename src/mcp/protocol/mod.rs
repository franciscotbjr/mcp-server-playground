//! JSON-RPC 2.0 and MCP domain types.

mod jsonrpc;
mod types;

pub use jsonrpc::{JsonRpcError, JsonRpcNotification, JsonRpcRequest, JsonRpcResponse};
pub use types::{
    CallToolParams, CallToolResult, Content, InitializeResult, InputSchema, ListToolsResult,
    ServerCapabilities, ServerInfo, ToolDefinition, ToolsCapability,
};
