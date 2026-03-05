//! Parameters for the `tools/call` method.

use serde::{Deserialize, Serialize};

/// Parameters for `tools/call`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CallToolParams {
    pub name: String,

    #[serde(default)]
    pub arguments: serde_json::Value,
}
