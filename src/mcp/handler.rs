//! MCP request dispatcher — routes JSON-RPC methods to the appropriate handler.

use crate::mcp::protocol::{
    InitializeResult, JsonRpcRequest, JsonRpcResponse,
    ServerCapabilities, ServerInfo, ToolsCapability,
};
use crate::mcp::tools::{
    CallToolParams, CallToolResult, ListToolsResult, ToolRegistry,
};

/// Handles incoming JSON-RPC requests and dispatches them.
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

        match request.method.as_str() {
            "initialize" => self.handle_initialize(id),
            "notifications/initialized" => {
                // Client acknowledgment — no response needed for notifications,
                // but if there's an id we still respond.
                JsonRpcResponse::success(id, serde_json::json!({}))
            }
            "tools/list" => self.handle_tools_list(id),
            "tools/call" => self.handle_tools_call(id, &request.params).await,
            method => JsonRpcResponse::error(
                id,
                crate::mcp::protocol::JsonRpcError::method_not_found(method),
            ),
        }
    }

    fn handle_initialize(&self, id: serde_json::Value) -> JsonRpcResponse {
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
                let error_result = CallToolResult::error(e.to_string());
                JsonRpcResponse::success(id, serde_json::to_value(error_result).unwrap())
            }
        }
    }
}
