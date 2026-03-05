//! MCP server capabilities declared during initialization.

use serde::{Deserialize, Serialize};

use super::tools_capability::ToolsCapability;

/// MCP server capabilities declared during initialization.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ServerCapabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<ToolsCapability>,
}
