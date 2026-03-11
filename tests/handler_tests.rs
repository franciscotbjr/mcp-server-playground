//! Tests for the MCP request handler dispatch.

use mcp_server_playground::{JsonRpcRequest, RequestHandler, ToolRegistry};

#[tokio::test]
async fn test_handler_initialize() {
    let registry = ToolRegistry::new();
    let handler = RequestHandler::new(registry);

    let request = JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        id: Some(serde_json::json!(1)),
        method: "initialize".to_string(),
        params: Some(serde_json::json!({})),
    };

    let response = handler.handle(&request).await;
    assert!(response.error.is_none());
    let result = response.result.unwrap();
    assert_eq!(result["protocolVersion"], "2025-03-26");
    assert_eq!(result["serverInfo"]["name"], "mcp-server-playground");
}

#[tokio::test]
async fn test_initialize_returns_server_info_version() {
    let registry = ToolRegistry::new();
    let handler = RequestHandler::new(registry);

    let request = JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        id: Some(serde_json::json!(1)),
        method: "initialize".to_string(),
        params: Some(serde_json::json!({})),
    };

    let response = handler.handle(&request).await;
    let result = response.result.unwrap();
    assert_eq!(result["serverInfo"]["version"], env!("CARGO_PKG_VERSION"));
}

#[tokio::test]
async fn test_initialize_returns_capabilities() {
    let registry = ToolRegistry::new();
    let handler = RequestHandler::new(registry);

    let request = JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        id: Some(serde_json::json!(1)),
        method: "initialize".to_string(),
        params: Some(serde_json::json!({})),
    };

    let response = handler.handle(&request).await;
    let result = response.result.unwrap();
    assert!(result["capabilities"]["tools"].is_object());
    assert_eq!(result["capabilities"]["tools"]["listChanged"], false);
}

#[tokio::test]
async fn test_initialize_preserves_request_id() {
    let registry = ToolRegistry::new();
    let handler = RequestHandler::new(registry);

    let request = JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        id: Some(serde_json::json!("custom-id-42")),
        method: "initialize".to_string(),
        params: Some(serde_json::json!({})),
    };

    let response = handler.handle(&request).await;
    assert_eq!(response.id, serde_json::json!("custom-id-42"));
}

#[tokio::test]
async fn test_handler_tools_list_empty() {
    let registry = ToolRegistry::new();
    let handler = RequestHandler::new(registry);

    let request = JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        id: Some(serde_json::json!(2)),
        method: "tools/list".to_string(),
        params: None,
    };

    let response = handler.handle(&request).await;
    let result = response.result.unwrap();
    assert_eq!(result["tools"], serde_json::json!([]));
}

#[tokio::test]
async fn test_handler_method_not_found() {
    let registry = ToolRegistry::new();
    let handler = RequestHandler::new(registry);

    let request = JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        id: Some(serde_json::json!(3)),
        method: "nonexistent/method".to_string(),
        params: None,
    };

    let response = handler.handle(&request).await;
    assert!(response.error.is_some());
    assert_eq!(response.error.unwrap().code, -32601);
}

#[tokio::test]
async fn test_handler_tools_call_unknown_tool() {
    let registry = ToolRegistry::new();
    let handler = RequestHandler::new(registry);

    let request = JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        id: Some(serde_json::json!(4)),
        method: "tools/call".to_string(),
        params: Some(serde_json::json!({"name": "ghost_tool", "arguments": {}})),
    };

    let response = handler.handle(&request).await;
    // Tool errors are returned as success with isError=true per MCP spec
    let result = response.result.unwrap();
    assert_eq!(result["isError"], true);
}

#[test]
fn test_handler_is_send_sync() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<RequestHandler>();
}
