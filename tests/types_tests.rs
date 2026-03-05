//! Tests for MCP domain types.

use mcp_server_playground::{
    CallToolParams, CallToolResult, Content, InitializeResult, InputSchema, ListToolsResult,
    ServerCapabilities, ServerInfo, ToolDefinition, ToolsCapability,
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
fn test_content_text_serialize() {
    let content = Content::text("hello world");
    let json = serde_json::to_value(&content).unwrap();
    assert_eq!(json["type"], "text");
    assert_eq!(json["text"], "hello world");
}

#[test]
fn test_content_text_deserialize() {
    let json = r#"{"type":"text","text":"parsed content"}"#;
    let content: Content = serde_json::from_str(json).unwrap();
    assert_eq!(content, Content::Text { text: "parsed content".to_string() });
}

#[test]
fn test_input_schema_object_defaults() {
    let schema = InputSchema::object();
    let json = serde_json::to_value(&schema).unwrap();
    assert_eq!(json["type"], "object");
    assert!(json.get("properties").is_none());
    assert!(json.get("required").is_none());
}

#[test]
fn test_input_schema_builder() {
    let schema = InputSchema::object()
        .with_properties(serde_json::json!({"name": {"type": "string"}}))
        .with_required(vec!["name".to_string()]);
    let json = serde_json::to_value(&schema).unwrap();
    assert_eq!(json["type"], "object");
    assert!(json["properties"]["name"].is_object());
    assert_eq!(json["required"], serde_json::json!(["name"]));
}

#[test]
fn test_call_tool_params_deserialize() {
    let json = r#"{"name":"my_tool","arguments":{"key":"value"}}"#;
    let params: CallToolParams = serde_json::from_str(json).unwrap();
    assert_eq!(params.name, "my_tool");
    assert_eq!(params.arguments["key"], "value");

    let no_args = r#"{"name":"simple_tool"}"#;
    let params2: CallToolParams = serde_json::from_str(no_args).unwrap();
    assert_eq!(params2.name, "simple_tool");
    assert_eq!(params2.arguments, serde_json::Value::Null);
}

#[test]
fn test_list_tools_result_serialize() {
    let result = ListToolsResult {
        tools: vec![ToolDefinition::new("t1", "desc1", InputSchema::object())],
    };
    let json = serde_json::to_value(&result).unwrap();
    assert_eq!(json["tools"].as_array().unwrap().len(), 1);
    assert_eq!(json["tools"][0]["name"], "t1");
}

#[test]
fn test_initialize_result_serialize() {
    let result = InitializeResult {
        protocol_version: "2024-11-05".to_string(),
        capabilities: ServerCapabilities {
            tools: Some(ToolsCapability { list_changed: Some(false) }),
        },
        server_info: ServerInfo {
            name: "test-server".to_string(),
            version: "1.0.0".to_string(),
        },
    };
    let json = serde_json::to_value(&result).unwrap();
    assert_eq!(json["protocolVersion"], "2024-11-05");
    assert_eq!(json["serverInfo"]["name"], "test-server");
    assert_eq!(json["serverInfo"]["version"], "1.0.0");
    assert_eq!(json["capabilities"]["tools"]["listChanged"], false);
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
