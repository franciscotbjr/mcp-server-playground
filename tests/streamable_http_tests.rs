//! Integration tests for the Streamable HTTP transport (`POST /mcp`, `GET /mcp`, `DELETE /mcp`).
//!
//! Each test spins up an ephemeral MCP server and exercises the Streamable HTTP
//! endpoints directly via `reqwest`.

use mcp_server_playground::{McpServer, RequestHandler, ToolRegistry};
use reqwest::Client;
use serde_json::{json, Value};

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Spin up an MCP server on a random port and return (base_url, server_handle).
async fn start_server() -> (String, tokio::task::JoinHandle<()>) {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    drop(listener);

    let registry = ToolRegistry::new();
    let handler = RequestHandler::new(registry);
    let server = McpServer::new(handler, addr);

    let handle = tokio::spawn(async move {
        let _ = server.run().await;
    });

    tokio::time::sleep(tokio::time::Duration::from_millis(150)).await;

    (format!("http://{addr}"), handle)
}

/// POST /mcp with the correct headers, returning the raw response.
async fn post_mcp(
    client: &Client,
    base: &str,
    session_id: Option<&str>,
    body: Value,
) -> reqwest::Response {
    let mut req = client
        .post(format!("{base}/mcp"))
        .header("Content-Type", "application/json")
        .header("Accept", "application/json, text/event-stream");

    if let Some(sid) = session_id {
        req = req.header("Mcp-Session-Id", sid);
    }

    req.body(serde_json::to_string(&body).unwrap())
        .send()
        .await
        .unwrap()
}

/// Run the full initialize + notifications/initialized handshake.
/// Returns the `Mcp-Session-Id`.
async fn initialize(client: &Client, base: &str) -> String {
    let resp = post_mcp(client, base, None, json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "initialize",
        "params": {
            "protocolVersion": "2025-03-26",
            "capabilities": {},
            "clientInfo": { "name": "test-client", "version": "0.1.0" }
        }
    }))
    .await;

    assert_eq!(resp.status(), 200);
    let session_id = resp
        .headers()
        .get("mcp-session-id")
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();

    let resp = post_mcp(client, base, Some(&session_id), json!({
        "jsonrpc": "2.0",
        "method": "notifications/initialized"
    }))
    .await;
    assert_eq!(resp.status(), 202);

    session_id
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_post_initialize_returns_session_id() {
    let (base, handle) = start_server().await;
    let client = Client::new();

    let resp = post_mcp(&client, &base, None, json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "initialize",
        "params": {
            "protocolVersion": "2025-03-26",
            "capabilities": {},
            "clientInfo": { "name": "test", "version": "0.1.0" }
        }
    }))
    .await;

    assert_eq!(resp.status(), 200);
    assert!(resp.headers().get("mcp-session-id").is_some());

    let body: Value = resp.json().await.unwrap();
    assert_eq!(body["result"]["protocolVersion"], "2025-03-26");
    assert_eq!(body["result"]["serverInfo"]["name"], "mcp-server-playground");

    handle.abort();
}

#[tokio::test]
async fn test_post_without_content_type_returns_415() {
    let (base, handle) = start_server().await;
    let client = Client::new();

    let resp = client
        .post(format!("{base}/mcp"))
        .header("Accept", "application/json, text/event-stream")
        .body(r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{}}"#)
        .send()
        .await
        .unwrap();

    assert_eq!(resp.status(), 415);
    handle.abort();
}

#[tokio::test]
async fn test_post_without_accept_returns_406() {
    let (base, handle) = start_server().await;
    let client = Client::new();

    let resp = client
        .post(format!("{base}/mcp"))
        .header("Content-Type", "application/json")
        .body(r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{}}"#)
        .send()
        .await
        .unwrap();

    assert_eq!(resp.status(), 406);
    handle.abort();
}

#[tokio::test]
async fn test_post_without_session_id_after_init_returns_400() {
    let (base, handle) = start_server().await;
    let client = Client::new();

    // Initialize first
    let _sid = initialize(&client, &base).await;

    // Now try tools/list without session header
    let resp = post_mcp(&client, &base, None, json!({
        "jsonrpc": "2.0",
        "id": 2,
        "method": "tools/list"
    }))
    .await;

    assert_eq!(resp.status(), 400);
    handle.abort();
}

#[tokio::test]
async fn test_post_with_bad_session_id_returns_404() {
    let (base, handle) = start_server().await;
    let client = Client::new();

    let resp = post_mcp(&client, &base, Some("nonexistent"), json!({
        "jsonrpc": "2.0",
        "id": 2,
        "method": "tools/list"
    }))
    .await;

    assert_eq!(resp.status(), 404);
    handle.abort();
}

#[tokio::test]
async fn test_post_notification_returns_202() {
    let (base, handle) = start_server().await;
    let client = Client::new();
    let sid = initialize(&client, &base).await;

    // Send a generic notification (no id → 202)
    let resp = post_mcp(&client, &base, Some(&sid), json!({
        "jsonrpc": "2.0",
        "method": "notifications/initialized"
    }))
    .await;

    assert_eq!(resp.status(), 202);
    handle.abort();
}

#[tokio::test]
async fn test_post_tools_list_returns_200_json() {
    let (base, handle) = start_server().await;
    let client = Client::new();
    let sid = initialize(&client, &base).await;

    let resp = post_mcp(&client, &base, Some(&sid), json!({
        "jsonrpc": "2.0",
        "id": 2,
        "method": "tools/list"
    }))
    .await;

    assert_eq!(resp.status(), 200);
    let ct = resp.headers().get("content-type").unwrap().to_str().unwrap();
    assert!(ct.contains("application/json"));

    let body: Value = resp.json().await.unwrap();
    assert!(body["result"]["tools"].is_array());

    handle.abort();
}

#[tokio::test]
async fn test_post_tools_call_returns_200_json() {
    let (base, handle) = start_server().await;
    let client = Client::new();
    let sid = initialize(&client, &base).await;

    let resp = post_mcp(&client, &base, Some(&sid), json!({
        "jsonrpc": "2.0",
        "id": 3,
        "method": "tools/call",
        "params": { "name": "unknown_tool", "arguments": {} }
    }))
    .await;

    assert_eq!(resp.status(), 200);
    let body: Value = resp.json().await.unwrap();
    // Unknown tool returns a successful JSON-RPC response with isError=true in result
    assert!(body["result"]["isError"] == true || body["error"].is_object());

    handle.abort();
}

#[tokio::test]
async fn test_post_batch_returns_array() {
    let (base, handle) = start_server().await;
    let client = Client::new();
    let sid = initialize(&client, &base).await;

    // Send a batch with two requests
    let resp = post_mcp(&client, &base, Some(&sid), json!([
        { "jsonrpc": "2.0", "id": 10, "method": "tools/list" },
        { "jsonrpc": "2.0", "id": 11, "method": "tools/list" }
    ]))
    .await;

    assert_eq!(resp.status(), 200);
    let body: Value = resp.json().await.unwrap();
    assert!(body.is_array(), "Batch response should be an array");
    assert_eq!(body.as_array().unwrap().len(), 2);

    handle.abort();
}

#[tokio::test]
async fn test_post_batch_notifications_only_returns_202() {
    let (base, handle) = start_server().await;
    let client = Client::new();
    let sid = initialize(&client, &base).await;

    // Send a batch with only notifications (no id)
    let resp = post_mcp(&client, &base, Some(&sid), json!([
        { "jsonrpc": "2.0", "method": "notifications/initialized" },
        { "jsonrpc": "2.0", "method": "notifications/initialized" }
    ]))
    .await;

    assert_eq!(resp.status(), 202);
    handle.abort();
}

#[tokio::test]
async fn test_delete_mcp_terminates_session() {
    let (base, handle) = start_server().await;
    let client = Client::new();
    let sid = initialize(&client, &base).await;

    // DELETE session
    let resp = client
        .delete(format!("{base}/mcp"))
        .header("Mcp-Session-Id", &sid)
        .send()
        .await
        .unwrap();

    assert_eq!(resp.status(), 200);

    // Session should be gone — next request returns 404
    let resp = post_mcp(&client, &base, Some(&sid), json!({
        "jsonrpc": "2.0",
        "id": 5,
        "method": "tools/list"
    }))
    .await;

    assert_eq!(resp.status(), 404);
    handle.abort();
}

#[tokio::test]
async fn test_delete_unknown_session_returns_404() {
    let (base, handle) = start_server().await;
    let client = Client::new();

    let resp = client
        .delete(format!("{base}/mcp"))
        .header("Mcp-Session-Id", "nonexistent")
        .send()
        .await
        .unwrap();

    assert_eq!(resp.status(), 404);
    handle.abort();
}

#[tokio::test]
async fn test_lifecycle_enforce() {
    let (base, handle) = start_server().await;
    let client = Client::new();

    // Step 1: Initialize
    let resp = post_mcp(&client, &base, None, json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "initialize",
        "params": {
            "protocolVersion": "2025-03-26",
            "capabilities": {},
            "clientInfo": { "name": "test", "version": "0.1.0" }
        }
    }))
    .await;
    assert_eq!(resp.status(), 200);
    let sid = resp.headers().get("mcp-session-id").unwrap().to_str().unwrap().to_string();

    // Step 2: Try tools/list before initialized notification — should fail
    let resp = post_mcp(&client, &base, Some(&sid), json!({
        "jsonrpc": "2.0",
        "id": 2,
        "method": "tools/list"
    }))
    .await;
    assert_eq!(resp.status(), 200);
    let body: Value = resp.json().await.unwrap();
    assert!(body["error"].is_object(), "Should be error before initialized");

    // Step 3: Send initialized notification
    let resp = post_mcp(&client, &base, Some(&sid), json!({
        "jsonrpc": "2.0",
        "method": "notifications/initialized"
    }))
    .await;
    assert_eq!(resp.status(), 202);

    // Step 4: Now tools/list should work
    let resp = post_mcp(&client, &base, Some(&sid), json!({
        "jsonrpc": "2.0",
        "id": 3,
        "method": "tools/list"
    }))
    .await;
    assert_eq!(resp.status(), 200);
    let body: Value = resp.json().await.unwrap();
    assert!(body["result"]["tools"].is_array(), "tools/list should succeed after initialized");

    handle.abort();
}
