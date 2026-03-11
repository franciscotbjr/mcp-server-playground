//! Minimal example demonstrating the MCP `initialize` lifecycle over Streamable HTTP.
//!
//! 1. Starts the MCP server on an ephemeral port
//! 2. Sends `initialize` request via `POST /mcp` — receives `Mcp-Session-Id`
//! 3. Sends `notifications/initialized` via `POST /mcp` with session header
//!
//! Run with: `cargo run --example initialize`

use mcp_server_playground::{McpServer, RequestHandler, ToolRegistry};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // --- Start server on port 0 (OS picks a free port) ---
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await?;
    let addr = listener.local_addr()?;
    println!("Server listening on {addr}");

    let registry = ToolRegistry::new();
    let handler = RequestHandler::new(registry);
    let server = McpServer::new(handler, addr);

    // Spawn server in background — drop the listener first so McpServer can bind.
    drop(listener);
    let server_handle = tokio::spawn(async move {
        if let Err(e) = server.run().await {
            eprintln!("Server error: {e}");
        }
    });

    // Give the server a moment to bind
    tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;

    let client = reqwest::Client::new();
    let mcp_url = format!("http://{addr}/mcp");

    // --- Step 1: Send initialize request ---
    println!("\n--- Step 1: POST /mcp — initialize ---");
    let resp = client
        .post(&mcp_url)
        .header("Content-Type", "application/json")
        .header("Accept", "application/json, text/event-stream")
        .body(serde_json::to_string(&serde_json::json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "initialize",
            "params": {
                "protocolVersion": "2025-03-26",
                "capabilities": {},
                "clientInfo": {
                    "name": "example-client",
                    "version": "0.1.0"
                }
            }
        }))?)
        .send()
        .await?;

    println!("  Status: {}", resp.status());

    let session_id = resp
        .headers()
        .get("mcp-session-id")
        .and_then(|v| v.to_str().ok())
        .map(String::from)
        .expect("Server must return Mcp-Session-Id header");
    println!("  Mcp-Session-Id: {session_id}");

    let body: serde_json::Value = resp.json().await?;
    println!("  Response: {}", serde_json::to_string_pretty(&body)?);

    // --- Step 2: Send notifications/initialized ---
    println!("\n--- Step 2: POST /mcp — notifications/initialized ---");
    let resp = client
        .post(&mcp_url)
        .header("Content-Type", "application/json")
        .header("Accept", "application/json, text/event-stream")
        .header("Mcp-Session-Id", &session_id)
        .body(serde_json::to_string(&serde_json::json!({
            "jsonrpc": "2.0",
            "method": "notifications/initialized"
        }))?)
        .send()
        .await?;
    println!("  Status: {} (202 = accepted notification)", resp.status());

    println!("\n--- Initialize lifecycle complete! ---");
    println!("Session is now in Ready state.");

    // Clean up
    server_handle.abort();
    Ok(())
}
