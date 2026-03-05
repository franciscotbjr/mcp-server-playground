//! JSON-RPC 2.0 notification (no id field).

use serde::{Deserialize, Serialize};

/// JSON-RPC 2.0 notification (no id field).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JsonRpcNotification {
    pub jsonrpc: String,
    pub method: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<serde_json::Value>,
}
