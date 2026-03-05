//! Describes a tool that the MCP server exposes to clients.

use serde::{Deserialize, Serialize};

use super::InputSchema;

/// Describes a tool that the MCP server exposes to clients.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ToolDefinition {
    pub name: String,
    pub description: String,

    #[serde(rename = "inputSchema")]
    pub input_schema: InputSchema,
}

impl ToolDefinition {
    /// Create a new tool definition.
    pub fn new(
        name: impl Into<String>,
        description: impl Into<String>,
        input_schema: InputSchema,
    ) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
            input_schema,
        }
    }
}
