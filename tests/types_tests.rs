//! Tests for MCP domain types.

use mcp_server_playground::{
    CallToolParams, CallToolResult, Content, InitializeResult, InputSchema, ListToolsResult,
    ServerCapabilities, ToolDefinition,
};

#[test]
fn test_tool_definition_serialize() {
    let tool = ToolDefinition::new(
        "test_tool",
        "A test tool",
        InputSchema::object()
            .with_properties(serde_json::json!({
                "query": {"type": "string", "description": "search query"}
            }))
            .with_required(vec!["query".to_string()]),
    );
    let json = serde_json::to_value(&tool).unwrap();
    assert_eq!(json["name"], "test_tool");
    assert_eq!(json["inputSchema"]["type"], "object");
    assert!(json["inputSchema"]["properties"]["query"].is_object());
}

#[test]
fn test_call_tool_result_text() {
    let result = CallToolResult::text("hello");
    let json = serde_json::to_value(&result).unwrap();
    assert_eq!(json["content"][0]["type"], "text");
    assert_eq!(json["content"][0]["text"], "hello");
    assert!(json.get("isError").is_none());
}

#[test]
fn test_call_tool_result_error() {
    let result = CallToolResult::error("something failed");
    let json = serde_json::to_value(&result).unwrap();
    assert_eq!(json["isError"], true);
}

#[test]
fn test_types_are_send_sync() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<ToolDefinition>();
    assert_send_sync::<InputSchema>();
    assert_send_sync::<CallToolResult>();
    assert_send_sync::<Content>();
    assert_send_sync::<ServerCapabilities>();
    assert_send_sync::<InitializeResult>();
    assert_send_sync::<CallToolParams>();
    assert_send_sync::<ListToolsResult>();
}
