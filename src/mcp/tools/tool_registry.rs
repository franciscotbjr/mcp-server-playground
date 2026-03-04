//! MCP tool registry — manages the collection of available tools.

use crate::error::{Error, Result};
use crate::mcp::tools::McpTool;
use crate::mcp::protocol::{CallToolResult, ToolDefinition};

/// Registry of all available MCP tools.
pub struct ToolRegistry {
    tools: Vec<Box<dyn McpTool>>,
}

impl ToolRegistry {
    /// Create an empty registry.
    pub fn new() -> Self {
        Self { tools: Vec::new() }
    }

    /// Register a tool.
    pub fn register(&mut self, tool: Box<dyn McpTool>) {
        self.tools.push(tool);
    }

    /// List all registered tool definitions.
    pub fn list_definitions(&self) -> Vec<ToolDefinition> {
        self.tools.iter().map(|t| t.to_definition()).collect()
    }

    /// Find and execute a tool by name.
    pub async fn call_tool(&self, name: &str, arguments: serde_json::Value) -> Result<CallToolResult> {
        let tool = self
            .tools
            .iter()
            .find(|t| t.name() == name)
            .ok_or_else(|| Error::ToolError(format!("Unknown tool: {name}")))?;

        tool.execute(arguments).await
    }
}

impl Default for ToolRegistry {
    fn default() -> Self {
        Self::new()
    }
}
