//! Declares that the server supports tools.

use serde::{Deserialize, Serialize};

/// Declares that the server supports tools.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ToolsCapability {
    #[serde(rename = "listChanged", skip_serializing_if = "Option::is_none")]
    pub list_changed: Option<bool>,
}
