//! Tests for the MCP tool registry.

use async_trait::async_trait;
use mcp_server_playground::{CallToolResult, McpTool, ToolRegistry};

/// A minimal mock tool for testing the registry.
#[derive(Debug)]
struct MockTool;

#[async_trait]
impl McpTool for MockTool {
    fn name(&self) -> &str {
        "mock_tool"
    }

    fn description(&self) -> &str {
        "A mock tool for tests"
    }

    fn input_schema(&self) -> serde_json::Value {
        serde_json::json!({"type": "object"})
    }

    async fn execute(&self, _arguments: serde_json::Value) -> mcp_server_playground::Result<CallToolResult> {
        Ok(CallToolResult::text("mock result"))
    }
}

#[test]
fn test_tool_registry_empty() {
    let registry = ToolRegistry::new();
    assert!(registry.list_definitions().is_empty());
}

#[test]
fn test_tool_registry_register_and_list() {
    let mut registry = ToolRegistry::new();
    registry.register(Box::new(MockTool));
    let defs = registry.list_definitions();
    assert_eq!(defs.len(), 1);
    assert_eq!(defs[0].name, "mock_tool");
    assert_eq!(defs[0].description, "A mock tool for tests");
}

#[tokio::test]
async fn test_tool_registry_call_unknown() {
    let registry = ToolRegistry::new();
    let result = registry.call_tool("nonexistent", serde_json::json!({})).await;
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Unknown tool"));
}

#[test]
fn test_tool_registry_default() {
    let registry = ToolRegistry::default();
    assert!(registry.list_definitions().is_empty());
}

#[test]
fn test_tool_registry_is_send_sync() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<ToolRegistry>();
}
