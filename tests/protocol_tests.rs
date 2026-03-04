//! Tests for JSON-RPC 2.0 protocol types.

use mcp_server_playground::{JsonRpcError, JsonRpcNotification, JsonRpcRequest, JsonRpcResponse};

#[test]
fn test_deserialize_request() {
    let json = r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{}}"#;
    let req: JsonRpcRequest = serde_json::from_str(json).unwrap();
    assert_eq!(req.method, "initialize");
    assert_eq!(req.id, Some(serde_json::json!(1)));
}

#[test]
fn test_serialize_success_response() {
    let resp = JsonRpcResponse::success(serde_json::json!(1), serde_json::json!({"ok": true}));
    let json = serde_json::to_string(&resp).unwrap();
    assert!(json.contains("\"result\""));
    assert!(!json.contains("\"error\""));
}

#[test]
fn test_serialize_error_response() {
    let resp = JsonRpcResponse::error(
        serde_json::json!(1),
        JsonRpcError::method_not_found("foo/bar"),
    );
    let json = serde_json::to_string(&resp).unwrap();
    assert!(json.contains("\"error\""));
    assert!(json.contains("-32601"));
}

#[test]
fn test_types_are_send_sync() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<JsonRpcRequest>();
    assert_send_sync::<JsonRpcResponse>();
    assert_send_sync::<JsonRpcError>();
    assert_send_sync::<JsonRpcNotification>();
}
