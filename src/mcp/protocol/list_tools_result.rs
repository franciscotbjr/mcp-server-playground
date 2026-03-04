//! Result of the `tools/list` method.

use serde::{Deserialize, Serialize};

use super::tool_definition::ToolDefinition;

/// Result of `tools/list`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ListToolsResult {
    pub tools: Vec<ToolDefinition>,
}
