//! MCP client example — exercises every contacts tool action via Streamable HTTP.
//!
//! **Prerequisites:** server must be running on `http://127.0.0.1:3000`.
//! Start it in another terminal with: `cargo run`
//!
//! Run with: `cargo run --example contacts_tool`

const BASE: &str = "http://127.0.0.1:3000";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let mcp_url = format!("{BASE}/mcp");

    // --- Step 1: Initialize lifecycle ---
    println!("--- Step 1: POST /mcp — initialize ---");
    let resp = post_json(&client, &mcp_url, None, serde_json::json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "initialize",
        "params": {
            "protocolVersion": "2025-03-26",
            "capabilities": {},
            "clientInfo": { "name": "contacts-example", "version": "0.1.0" }
        }
    }))
    .await?;

    let session_id = resp
        .headers()
        .get("mcp-session-id")
        .and_then(|v| v.to_str().ok())
        .map(String::from)
        .expect("Server must return Mcp-Session-Id");
    println!("  Mcp-Session-Id: {session_id}");

    let body: serde_json::Value = resp.json().await?;
    println!("  Response: {}", serde_json::to_string_pretty(&body)?);

    println!("\n--- Step 1b: POST /mcp — notifications/initialized ---");
    let resp = post_json(&client, &mcp_url, Some(&session_id), serde_json::json!({
        "jsonrpc": "2.0",
        "method": "notifications/initialized"
    }))
    .await?;
    println!("  Status: {}", resp.status());

    // --- Step 2: tools/list ---
    println!("\n--- Step 2: POST /mcp — tools/list ---");
    let resp = post_json(&client, &mcp_url, Some(&session_id), serde_json::json!({
        "jsonrpc": "2.0",
        "id": 2,
        "method": "tools/list"
    }))
    .await?;
    let body: serde_json::Value = resp.json().await?;
    println!("  Response: {}", serde_json::to_string_pretty(&body)?);

    // --- Step 3: Contacts tool actions ---
    let actions: Vec<(&str, serde_json::Value)> = vec![
        (
            "list_contacts (all)",
            serde_json::json!({"action": "list_contacts"}),
        ),
        (
            "list_contacts (tag=family)",
            serde_json::json!({"action": "list_contacts", "tag": "family"}),
        ),
        (
            "list_contacts (favorite=true)",
            serde_json::json!({"action": "list_contacts", "favorite": true}),
        ),
        (
            "get_contact (c001)",
            serde_json::json!({"action": "get_contact", "contact_id": "c001"}),
        ),
        (
            "search_contacts (tech)",
            serde_json::json!({"action": "search_contacts", "query": "tech"}),
        ),
        (
            "contacts_by_tag (doctor)",
            serde_json::json!({"action": "contacts_by_tag", "tag": "doctor"}),
        ),
        (
            "favorite_contacts",
            serde_json::json!({"action": "favorite_contacts"}),
        ),
    ];

    for (i, (label, args)) in actions.iter().enumerate() {
        let id = i as u64 + 10;
        println!("\n--- Step 3.{}: tools/call — {label} ---", i + 1);
        let resp = post_json(&client, &mcp_url, Some(&session_id), serde_json::json!({
            "jsonrpc": "2.0",
            "id": id,
            "method": "tools/call",
            "params": {
                "name": "contacts",
                "arguments": args
            }
        }))
        .await?;
        let body: serde_json::Value = resp.json().await?;
        println!("  Response: {}", serde_json::to_string_pretty(&body)?);
    }

    // --- Done ---
    println!("\n--- All contacts actions exercised! ---");
    Ok(())
}

async fn post_json(
    client: &reqwest::Client,
    url: &str,
    session_id: Option<&str>,
    body: serde_json::Value,
) -> Result<reqwest::Response, Box<dyn std::error::Error>> {
    let mut req = client
        .post(url)
        .header("Content-Type", "application/json")
        .header("Accept", "application/json, text/event-stream");

    if let Some(sid) = session_id {
        req = req.header("Mcp-Session-Id", sid);
    }

    let resp = req.body(serde_json::to_string(&body)?).send().await?;
    println!("  POST status: {}", resp.status());
    Ok(resp)
}
