//! Result of the `tools/list` method.

use serde::{Deserialize, Serialize};

use super::ToolDefinition;

/// Result of `tools/list`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ListToolsResult {
    pub tools: Vec<ToolDefinition>,
}
