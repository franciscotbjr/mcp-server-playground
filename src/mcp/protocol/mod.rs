//! JSON-RPC 2.0 and MCP domain types.

mod jsonrpc;
mod call_tool_params;
mod call_tool_result;
mod content;
mod initialize_result;
mod input_schema;
mod list_tools_result;
mod server_capabilities;
mod server_info;
mod tool_definition;
mod tools_capability;

pub use jsonrpc::{JsonRpcError, JsonRpcNotification, JsonRpcRequest, JsonRpcResponse};
pub use call_tool_params::CallToolParams;
pub use call_tool_result::CallToolResult;
pub use content::Content;
pub use initialize_result::InitializeResult;
pub use input_schema::InputSchema;
pub use list_tools_result::ListToolsResult;
pub use server_capabilities::ServerCapabilities;
pub use server_info::ServerInfo;
pub use tool_definition::ToolDefinition;
pub use tools_capability::ToolsCapability;
