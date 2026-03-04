//! JSON-RPC 2.0 protocol types for MCP communication.

use serde::{Deserialize, Serialize};

/// JSON-RPC 2.0 request.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JsonRpcRequest {
    pub jsonrpc: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<serde_json::Value>,

    pub method: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<serde_json::Value>,
}

/// JSON-RPC 2.0 success response.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JsonRpcResponse {
    pub jsonrpc: String,

    pub id: serde_json::Value,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<serde_json::Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<JsonRpcError>,
}

impl JsonRpcResponse {
    /// Create a success response.
    pub fn success(id: serde_json::Value, result: serde_json::Value) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            id,
            result: Some(result),
            error: None,
        }
    }

    /// Create an error response.
    pub fn error(id: serde_json::Value, error: JsonRpcError) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            id,
            result: None,
            error: Some(error),
        }
    }
}

/// JSON-RPC 2.0 error object.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JsonRpcError {
    pub code: i32,
    pub message: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}

impl JsonRpcError {
    /// Standard JSON-RPC error: Parse error (-32700).
    pub fn parse_error(detail: impl Into<String>) -> Self {
        Self {
            code: -32700,
            message: "Parse error".to_string(),
            data: Some(serde_json::Value::String(detail.into())),
        }
    }

    /// Standard JSON-RPC error: Invalid request (-32600).
    pub fn invalid_request(detail: impl Into<String>) -> Self {
        Self {
            code: -32600,
            message: "Invalid Request".to_string(),
            data: Some(serde_json::Value::String(detail.into())),
        }
    }

    /// Standard JSON-RPC error: Method not found (-32601).
    pub fn method_not_found(method: impl Into<String>) -> Self {
        Self {
            code: -32601,
            message: "Method not found".to_string(),
            data: Some(serde_json::Value::String(method.into())),
        }
    }

    /// Standard JSON-RPC error: Invalid params (-32602).
    pub fn invalid_params(detail: impl Into<String>) -> Self {
        Self {
            code: -32602,
            message: "Invalid params".to_string(),
            data: Some(serde_json::Value::String(detail.into())),
        }
    }

    /// Standard JSON-RPC error: Internal error (-32603).
    pub fn internal_error(detail: impl Into<String>) -> Self {
        Self {
            code: -32603,
            message: "Internal error".to_string(),
            data: Some(serde_json::Value::String(detail.into())),
        }
    }
}

/// JSON-RPC 2.0 notification (no id field).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JsonRpcNotification {
    pub jsonrpc: String,
    pub method: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<serde_json::Value>,
}
