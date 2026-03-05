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
fn test_jsonrpc_error_parse_error() {
    let err = JsonRpcError::parse_error("unexpected token");
    assert_eq!(err.code, -32700);
    assert_eq!(err.message, "Parse error");
    assert_eq!(err.data, Some(serde_json::json!("unexpected token")));
}

#[test]
fn test_jsonrpc_error_invalid_request() {
    let err = JsonRpcError::invalid_request("missing jsonrpc field");
    assert_eq!(err.code, -32600);
    assert_eq!(err.message, "Invalid Request");
    assert_eq!(err.data, Some(serde_json::json!("missing jsonrpc field")));
}

#[test]
fn test_jsonrpc_error_invalid_params() {
    let err = JsonRpcError::invalid_params("expected object");
    assert_eq!(err.code, -32602);
    assert_eq!(err.message, "Invalid params");
    assert_eq!(err.data, Some(serde_json::json!("expected object")));
}

#[test]
fn test_jsonrpc_error_internal_error() {
    let err = JsonRpcError::internal_error("database unavailable");
    assert_eq!(err.code, -32603);
    assert_eq!(err.message, "Internal error");
    assert_eq!(err.data, Some(serde_json::json!("database unavailable")));
}

#[test]
fn test_serialize_notification() {
    let notif = JsonRpcNotification {
        jsonrpc: "2.0".to_string(),
        method: "notifications/initialized".to_string(),
        params: None,
    };
    let json = serde_json::to_value(&notif).unwrap();
    assert_eq!(json["jsonrpc"], "2.0");
    assert_eq!(json["method"], "notifications/initialized");
    assert!(json.get("params").is_none());

    let with_params = JsonRpcNotification {
        jsonrpc: "2.0".to_string(),
        method: "test".to_string(),
        params: Some(serde_json::json!({"key": "value"})),
    };
    let json2 = serde_json::to_value(&with_params).unwrap();
    assert_eq!(json2["params"]["key"], "value");
}

#[test]
fn test_deserialize_response() {
    let json = r#"{"jsonrpc":"2.0","id":1,"result":{"ok":true}}"#;
    let resp: JsonRpcResponse = serde_json::from_str(json).unwrap();
    assert_eq!(resp.id, serde_json::json!(1));
    assert!(resp.result.is_some());
    assert!(resp.error.is_none());

    let err_json = r#"{"jsonrpc":"2.0","id":2,"error":{"code":-32601,"message":"Method not found"}}"#;
    let err_resp: JsonRpcResponse = serde_json::from_str(err_json).unwrap();
    assert_eq!(err_resp.id, serde_json::json!(2));
    assert!(err_resp.result.is_none());
    assert_eq!(err_resp.error.unwrap().code, -32601);
}

#[test]
fn test_types_are_send_sync() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<JsonRpcRequest>();
    assert_send_sync::<JsonRpcResponse>();
    assert_send_sync::<JsonRpcError>();
    assert_send_sync::<JsonRpcNotification>();
}
