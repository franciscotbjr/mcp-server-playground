//! Minimal example demonstrating the MCP `initialize` lifecycle over SSE.
//!
//! 1. Starts the MCP server on an ephemeral port
//! 2. Connects to `GET /sse` and reads the `endpoint` event
//! 3. Sends `initialize` request via `POST /message`
//! 4. Reads the `initialize` response from the SSE stream
//! 5. Sends `notifications/initialized` to complete the handshake
//!
//! Run with: `cargo run --example initialize`

use futures_util::StreamExt;
use mcp_server_playground::{McpServer, RequestHandler, ToolRegistry};
use tokio::io::AsyncBufReadExt;

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

    // --- Step 1: Connect to GET /sse (streaming) ---
    println!("\n--- Step 1: Connecting to GET /sse ---");
    let sse_url = format!("http://{addr}/sse");
    let sse_response = client.get(&sse_url).send().await?;
    println!("SSE connection status: {}", sse_response.status());

    // Read SSE stream as lines until we find the `endpoint` event
    let byte_stream = sse_response.bytes_stream();
    let stream_reader = tokio_util::io::StreamReader::new(
        byte_stream.map(|result| {
            result.map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
        }),
    );
    let mut lines = tokio::io::BufReader::new(stream_reader).lines();

    let mut message_path = String::new();
    while let Some(line) = lines.next_line().await? {
        println!("  SSE: {line}");
        if let Some(path) = line.strip_prefix("data: /message") {
            message_path = format!("/message{path}");
            break;
        }
    }

    if message_path.is_empty() {
        eprintln!("ERROR: Did not receive endpoint event");
        server_handle.abort();
        return Ok(());
    }

    let message_url = format!("http://{addr}{message_path}");
    println!("Message endpoint: {message_url}");

    // Keep the SSE stream alive in background so responses can arrive
    let sse_reader_handle = tokio::spawn(async move {
        while let Ok(Some(line)) = lines.next_line().await {
            if !line.is_empty() {
                println!("  SSE event: {line}");
            }
        }
    });

    // --- Step 2: Send initialize request ---
    println!("\n--- Step 2: Sending initialize request ---");
    let initialize_request = serde_json::json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "initialize",
        "params": {
            "protocolVersion": "2024-11-05",
            "capabilities": {},
            "clientInfo": {
                "name": "example-client",
                "version": "0.1.0"
            }
        }
    });

    let resp = client
        .post(&message_url)
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&initialize_request)?)
        .send()
        .await?;
    println!("POST /message status: {}", resp.status());

    // Give the server a moment to process and send the SSE response
    tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;

    // --- Step 3: Send notifications/initialized ---
    println!("\n--- Step 3: Sending notifications/initialized ---");
    let initialized_notification = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "notifications/initialized"
    });

    let resp = client
        .post(&message_url)
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&initialized_notification)?)
        .send()
        .await?;
    println!("POST /message status: {}", resp.status());

    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    println!("\n--- Initialize lifecycle complete! ---");
    println!("Session is now in Ready state.");

    // Clean up
    sse_reader_handle.abort();
    server_handle.abort();
    Ok(())
}
