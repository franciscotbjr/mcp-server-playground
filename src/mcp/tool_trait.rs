//! MCP tool trait — defines the interface each tool must implement.

use crate::error::Result;
use crate::mcp::types::{CallToolResult, InputSchema, ToolDefinition};

use async_trait::async_trait;

/// Trait that each MCP tool must implement.
#[async_trait]
pub trait McpTool: Send + Sync {
    /// The tool name as exposed to MCP clients.
    fn name(&self) -> &str;

    /// Human-readable description of what the tool does.
    fn description(&self) -> &str;

    /// JSON Schema describing the tool's expected input.
    fn input_schema(&self) -> serde_json::Value;

    /// Execute the tool with the given arguments.
    async fn execute(&self, arguments: serde_json::Value) -> Result<CallToolResult>;

    /// Build a `ToolDefinition` from this tool's metadata.
    fn to_definition(&self) -> ToolDefinition {
        ToolDefinition {
            name: self.name().to_string(),
            description: self.description().to_string(),
            input_schema: serde_json::from_value(self.input_schema()).unwrap_or_else(|_| {
                InputSchema::object()
            }),
        }
    }
}
