//! Tests for the MCP tool registry.

use mcp_server_playground::ToolRegistry;

#[test]
fn test_tool_registry_empty() {
    let registry = ToolRegistry::new();
    assert!(registry.list_definitions().is_empty());
}

#[test]
fn test_tool_registry_is_send_sync() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<ToolRegistry>();
}
