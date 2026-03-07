//! MCP request dispatcher — routes JSON-RPC methods to the appropriate handler.

use crate::mcp::protocol::{JsonRpcRequest, JsonRpcResponse};
use crate::mcp::tools::{
    CallToolParams, CallToolResult, ListToolsResult, ToolRegistry,
};
use crate::mcp::transport::{
    InitializeResult, ServerCapabilities, ServerInfo, ToolsCapability,
};
use tracing::info;

/// Handles incoming JSON-RPC requests and dispatches them.
#[derive(Debug)]
pub struct RequestHandler {
    registry: ToolRegistry,
}

impl RequestHandler {
    /// Create a new handler with the given tool registry.
    pub fn new(registry: ToolRegistry) -> Self {
        Self { registry }
    }

    /// Dispatch a parsed JSON-RPC request and return a response.
    pub async fn handle(&self, request: &JsonRpcRequest) -> JsonRpcResponse {
        let id = request.id.clone().unwrap_or(serde_json::Value::Null);

        info!(method = %request.method, "Received JSON-RPC request");

        match request.method.as_str() {
            "initialize" => self.handle_initialize(id),
            "notifications/initialized" => {
                info!("Client initialized notification received");
                // Client acknowledgment — no response needed for notifications,
                // but if there's an id we still respond.
                JsonRpcResponse::success(id, serde_json::json!({}))
            }
            "tools/list" => self.handle_tools_list(id),
            "tools/call" => self.handle_tools_call(id, &request.params).await,
            method => {
                info!(method, "Unknown method requested");
                JsonRpcResponse::error(
                    id,
                    crate::mcp::protocol::JsonRpcError::method_not_found(method),
                )
            }
        }
    }

    fn handle_initialize(&self, id: serde_json::Value) -> JsonRpcResponse {
        info!("Handling initialize — protocol 2024-11-05");
        let result = InitializeResult {
            protocol_version: "2024-11-05".to_string(),
            capabilities: ServerCapabilities {
                tools: Some(ToolsCapability {
                    list_changed: Some(false),
                }),
            },
            server_info: ServerInfo {
                name: "mcp-server-playground".to_string(),
                version: env!("CARGO_PKG_VERSION").to_string(),
            },
        };

        JsonRpcResponse::success(id, serde_json::to_value(result).unwrap())
    }

    fn handle_tools_list(&self, id: serde_json::Value) -> JsonRpcResponse {
        info!("Handling tools/list");
        let result = ListToolsResult {
            tools: self.registry.list_definitions(),
        };
        JsonRpcResponse::success(id, serde_json::to_value(result).unwrap())
    }

    async fn handle_tools_call(
        &self,
        id: serde_json::Value,
        params: &Option<serde_json::Value>,
    ) -> JsonRpcResponse {
        let params = match params {
            Some(p) => p,
            None => {
                return JsonRpcResponse::error(
                    id,
                    crate::mcp::protocol::JsonRpcError::invalid_params("Missing params"),
                );
            }
        };

        info!(tool = %params.get("name").and_then(|v| v.as_str()).unwrap_or("unknown"), "Handling tools/call");

        let call_params: CallToolParams = match serde_json::from_value(params.clone()) {
            Ok(p) => p,
            Err(e) => {
                return JsonRpcResponse::error(
                    id,
                    crate::mcp::protocol::JsonRpcError::invalid_params(e.to_string()),
                );
            }
        };

        match self.registry.call_tool(&call_params.name, call_params.arguments).await {
            Ok(result) => {
                JsonRpcResponse::success(id, serde_json::to_value(result).unwrap())
            }
            Err(e) => {
                // Tool execution errors are returned as a success JSON-RPC response
                // with `isError: true` inside the CallToolResult. A JsonRpcResponse::error
                // would indicate a protocol-level failure (e.g. method not found),
                // not a tool-level failure.
                let error_result = CallToolResult::error(e.to_string());
                JsonRpcResponse::success(id, serde_json::to_value(error_result).unwrap())
            }
        }
    }
}
