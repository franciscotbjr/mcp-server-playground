//! JSON-RPC 2.0 and MCP server protocol types.

mod jsonrpc;
mod initialize_result;
mod server_capabilities;
mod server_info;
mod tools_capability;

pub use jsonrpc::{JsonRpcError, JsonRpcNotification, JsonRpcRequest, JsonRpcResponse};
pub use initialize_result::InitializeResult;
pub use server_capabilities::ServerCapabilities;
pub use server_info::ServerInfo;
pub use tools_capability::ToolsCapability;
