//! Tests for the CalendarTool MCP tool implementation.

use mcp_server_playground::{CalendarTool, McpTool};

fn tool() -> CalendarTool {
    CalendarTool::new("calendar.json").expect("Failed to load calendar.json")
}

#[tokio::test]
async fn test_list_events_all() {
    let t = tool();
    let result = t
        .execute(serde_json::json!({"action": "list_events"}))
        .await
        .unwrap();
    let text = &result.content[0];
    let mcp_server_playground::Content::Text { text } = text;
    let events: Vec<serde_json::Value> = serde_json::from_str(text).unwrap();
    assert_eq!(events.len(), 12);
}

#[tokio::test]
async fn test_list_events_filter_category() {
    let t = tool();
    let result = t
        .execute(serde_json::json!({"action": "list_events", "category": "work"}))
        .await
        .unwrap();
    let mcp_server_playground::Content::Text { text } = &result.content[0];
    let events: Vec<serde_json::Value> = serde_json::from_str(text).unwrap();
    assert!(events.len() >= 2, "Expected at least 2 work events");
    for evt in &events {
        assert_eq!(evt["category"], "work");
    }
}

#[tokio::test]
async fn test_list_events_filter_priority() {
    let t = tool();
    let result = t
        .execute(serde_json::json!({"action": "list_events", "priority": "high"}))
        .await
        .unwrap();
    let mcp_server_playground::Content::Text { text } = &result.content[0];
    let events: Vec<serde_json::Value> = serde_json::from_str(text).unwrap();
    assert!(events.len() >= 2, "Expected at least 2 high-priority events");
    for evt in &events {
        assert_eq!(evt["priority"], "high");
    }
}

#[tokio::test]
async fn test_get_event_found() {
    let t = tool();
    let result = t
        .execute(serde_json::json!({"action": "get_event", "event_id": "evt_001"}))
        .await
        .unwrap();
    assert!(result.is_error.is_none());
    let mcp_server_playground::Content::Text { text } = &result.content[0];
    let event: serde_json::Value = serde_json::from_str(text).unwrap();
    assert_eq!(event["id"], "evt_001");
    assert_eq!(event["category"], "medical");
}

#[tokio::test]
async fn test_get_event_not_found() {
    let t = tool();
    let result = t
        .execute(serde_json::json!({"action": "get_event", "event_id": "evt_999"}))
        .await
        .unwrap();
    assert_eq!(result.is_error, Some(true));
    let mcp_server_playground::Content::Text { text } = &result.content[0];
    assert!(text.contains("Event not found"));
}

#[tokio::test]
async fn test_search_events() {
    let t = tool();
    let result = t
        .execute(serde_json::json!({"action": "search_events", "query": "meeting"}))
        .await
        .unwrap();
    let mcp_server_playground::Content::Text { text } = &result.content[0];
    let events: Vec<serde_json::Value> = serde_json::from_str(text).unwrap();
    assert!(!events.is_empty(), "Expected at least one event matching 'meeting'");
}

#[tokio::test]
async fn test_events_by_date() {
    let t = tool();
    let result = t
        .execute(serde_json::json!({"action": "events_by_date", "date": "2025-08-28"}))
        .await
        .unwrap();
    let mcp_server_playground::Content::Text { text } = &result.content[0];
    let events: Vec<serde_json::Value> = serde_json::from_str(text).unwrap();
    assert!(!events.is_empty(), "Expected at least one event on 2025-08-28");
    for evt in &events {
        let start = evt["startDateTime"].as_str().unwrap();
        assert!(start.starts_with("2025-08-28"));
    }
}

#[tokio::test]
async fn test_events_by_category() {
    let t = tool();
    let result = t
        .execute(serde_json::json!({"action": "events_by_category", "category": "medical"}))
        .await
        .unwrap();
    let mcp_server_playground::Content::Text { text } = &result.content[0];
    let events: Vec<serde_json::Value> = serde_json::from_str(text).unwrap();
    assert!(events.len() >= 2, "Expected at least 2 medical events");
    for evt in &events {
        assert_eq!(evt["category"], "medical");
    }
}

#[tokio::test]
async fn test_upcoming_events_default() {
    let t = tool();
    let result = t
        .execute(serde_json::json!({"action": "upcoming_events"}))
        .await
        .unwrap();
    let mcp_server_playground::Content::Text { text } = &result.content[0];
    let events: Vec<serde_json::Value> = serde_json::from_str(text).unwrap();
    assert_eq!(events.len(), 5);
    // Verify sorted by startDateTime
    for i in 1..events.len() {
        let prev = events[i - 1]["startDateTime"].as_str().unwrap();
        let curr = events[i]["startDateTime"].as_str().unwrap();
        assert!(prev <= curr, "Events should be sorted by startDateTime");
    }
}

#[tokio::test]
async fn test_upcoming_events_custom_count() {
    let t = tool();
    let result = t
        .execute(serde_json::json!({"action": "upcoming_events", "count": 3}))
        .await
        .unwrap();
    let mcp_server_playground::Content::Text { text } = &result.content[0];
    let events: Vec<serde_json::Value> = serde_json::from_str(text).unwrap();
    assert_eq!(events.len(), 3);
}

#[tokio::test]
async fn test_invalid_action() {
    let t = tool();
    let result = t
        .execute(serde_json::json!({"action": "delete_event"}))
        .await
        .unwrap();
    assert_eq!(result.is_error, Some(true));
    let mcp_server_playground::Content::Text { text } = &result.content[0];
    assert!(text.contains("Unknown action"));
}

#[tokio::test]
async fn test_missing_action() {
    let t = tool();
    let result = t.execute(serde_json::json!({})).await;
    assert!(result.is_err(), "Missing action should return Err");
}

#[test]
fn test_calendar_tool_is_send_sync() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<CalendarTool>();
}

#[test]
fn test_tool_definition() {
    let t = tool();
    let def = t.to_definition();
    assert_eq!(def.name, "calendar");
    assert!(!def.description.is_empty());
    assert_eq!(def.input_schema.schema_type, "object");
    assert!(def.input_schema.properties.is_some());
    let props = def.input_schema.properties.as_ref().unwrap();
    assert!(props.get("action").is_some());
}
