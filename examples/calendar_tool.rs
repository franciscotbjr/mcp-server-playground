//! MCP client example — exercises every calendar tool action over SSE.
//!
//! **Prerequisites:** server must be running on `http://127.0.0.1:3000`.
//! Start it in another terminal with: `cargo run`
//!
//! Run with: `cargo run --example calendar_tool`

use futures_util::StreamExt;
use tokio::io::AsyncBufReadExt;

const BASE: &str = "http://127.0.0.1:3000";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    // --- Step 1: Connect to GET /sse ---
    println!("--- Step 1: Connecting to GET {BASE}/sse ---");
    let sse_response = client.get(format!("{BASE}/sse")).send().await?;
    println!("SSE status: {}", sse_response.status());

    let byte_stream = sse_response.bytes_stream();
    let stream_reader = tokio_util::io::StreamReader::new(
        byte_stream.map(|r| r.map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))),
    );
    let mut lines = tokio::io::BufReader::new(stream_reader).lines();

    let mut message_url = String::new();
    while let Some(line) = lines.next_line().await? {
        println!("  SSE: {line}");
        if let Some(path) = line.strip_prefix("data: /message") {
            message_url = format!("{BASE}/message{path}");
            break;
        }
    }

    if message_url.is_empty() {
        eprintln!("ERROR: Did not receive endpoint event");
        return Ok(());
    }
    println!("Message endpoint: {message_url}");

    // Keep SSE stream alive in background to receive responses
    let sse_handle = tokio::spawn(async move {
        while let Ok(Some(line)) = lines.next_line().await {
            if !line.is_empty() {
                println!("  SSE event: {line}");
            }
        }
    });

    // --- Step 2: Initialize lifecycle ---
    println!("\n--- Step 2: MCP initialize ---");
    post_json(
        &client,
        &message_url,
        serde_json::json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "initialize",
            "params": {
                "protocolVersion": "2024-11-05",
                "capabilities": {},
                "clientInfo": { "name": "calendar-example", "version": "0.1.0" }
            }
        }),
    )
    .await?;

    tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;

    post_json(
        &client,
        &message_url,
        serde_json::json!({
            "jsonrpc": "2.0",
            "method": "notifications/initialized"
        }),
    )
    .await?;

    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    // --- Step 3: tools/list ---
    println!("\n--- Step 3: tools/list ---");
    post_json(
        &client,
        &message_url,
        serde_json::json!({
            "jsonrpc": "2.0",
            "id": 2,
            "method": "tools/list"
        }),
    )
    .await?;

    tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;

    // --- Step 4: Calendar tool actions ---
    let actions: Vec<(&str, serde_json::Value)> = vec![
        (
            "list_events (all)",
            serde_json::json!({"action": "list_events"}),
        ),
        (
            "list_events (category=work)",
            serde_json::json!({"action": "list_events", "category": "work"}),
        ),
        (
            "get_event (evt_001)",
            serde_json::json!({"action": "get_event", "event_id": "evt_001"}),
        ),
        (
            "search_events (meeting)",
            serde_json::json!({"action": "search_events", "query": "meeting"}),
        ),
        (
            "events_by_date (2025-08-28)",
            serde_json::json!({"action": "events_by_date", "date": "2025-08-28"}),
        ),
        (
            "upcoming_events (count=3)",
            serde_json::json!({"action": "upcoming_events", "count": 3}),
        ),
    ];

    for (i, (label, args)) in actions.iter().enumerate() {
        let id = i as u64 + 10;
        println!("\n--- Step 4.{}: tools/call — {label} ---", i + 1);
        post_json(
            &client,
            &message_url,
            serde_json::json!({
                "jsonrpc": "2.0",
                "id": id,
                "method": "tools/call",
                "params": {
                    "name": "calendar",
                    "arguments": args
                }
            }),
        )
        .await?;

        tokio::time::sleep(tokio::time::Duration::from_millis(300)).await;
    }

    // --- Done ---
    println!("\n--- All calendar actions exercised! ---");
    sse_handle.abort();
    Ok(())
}

async fn post_json(
    client: &reqwest::Client,
    url: &str,
    body: serde_json::Value,
) -> Result<(), Box<dyn std::error::Error>> {
    let resp = client
        .post(url)
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&body)?)
        .send()
        .await?;
    println!("  POST status: {}", resp.status());
    Ok(())
}
