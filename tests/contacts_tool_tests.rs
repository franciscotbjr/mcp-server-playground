//! Tests for the ContactsTool MCP tool implementation.

use mcp_server_playground::{ContactsTool, McpTool};

fn tool() -> ContactsTool {
    ContactsTool::new("contacts.json").expect("Failed to load contacts.json")
}

#[tokio::test]
async fn test_list_contacts_all() {
    let t = tool();
    let result = t
        .execute(serde_json::json!({"action": "list_contacts"}))
        .await
        .unwrap();
    let mcp_server_playground::Content::Text { text } = &result.content[0];
    let contacts: Vec<serde_json::Value> = serde_json::from_str(text).unwrap();
    assert_eq!(contacts.len(), 17);
}

#[tokio::test]
async fn test_list_contacts_filter_tag() {
    let t = tool();
    let result = t
        .execute(serde_json::json!({"action": "list_contacts", "tag": "family"}))
        .await
        .unwrap();
    let mcp_server_playground::Content::Text { text } = &result.content[0];
    let contacts: Vec<serde_json::Value> = serde_json::from_str(text).unwrap();
    assert!(contacts.len() >= 3, "Expected at least 3 contacts with 'family' tag");
    for c in &contacts {
        let tags = c["tags"].as_array().unwrap();
        assert!(tags.iter().any(|t| t.as_str() == Some("family")));
    }
}

#[tokio::test]
async fn test_list_contacts_filter_favorite() {
    let t = tool();
    let result = t
        .execute(serde_json::json!({"action": "list_contacts", "favorite": true}))
        .await
        .unwrap();
    let mcp_server_playground::Content::Text { text } = &result.content[0];
    let contacts: Vec<serde_json::Value> = serde_json::from_str(text).unwrap();
    assert!(contacts.len() >= 2, "Expected at least 2 favorite contacts");
    for c in &contacts {
        assert_eq!(c["favorite"], true);
    }
}

#[tokio::test]
async fn test_list_contacts_filter_company() {
    let t = tool();
    let result = t
        .execute(serde_json::json!({"action": "list_contacts", "company": "Tech Solutions Inc."}))
        .await
        .unwrap();
    let mcp_server_playground::Content::Text { text } = &result.content[0];
    let contacts: Vec<serde_json::Value> = serde_json::from_str(text).unwrap();
    assert_eq!(contacts.len(), 1);
    assert_eq!(contacts[0]["id"], "c001");
}

#[tokio::test]
async fn test_get_contact_found() {
    let t = tool();
    let result = t
        .execute(serde_json::json!({"action": "get_contact", "contact_id": "c001"}))
        .await
        .unwrap();
    assert!(result.is_error.is_none());
    let mcp_server_playground::Content::Text { text } = &result.content[0];
    let contact: serde_json::Value = serde_json::from_str(text).unwrap();
    assert_eq!(contact["id"], "c001");
    assert_eq!(contact["displayName"], "Tiger Brilliant");
}

#[tokio::test]
async fn test_get_contact_not_found() {
    let t = tool();
    let result = t
        .execute(serde_json::json!({"action": "get_contact", "contact_id": "c999"}))
        .await
        .unwrap();
    assert_eq!(result.is_error, Some(true));
    let mcp_server_playground::Content::Text { text } = &result.content[0];
    assert!(text.contains("Contact not found"));
}

#[tokio::test]
async fn test_search_contacts() {
    let t = tool();
    let result = t
        .execute(serde_json::json!({"action": "search_contacts", "query": "tech"}))
        .await
        .unwrap();
    let mcp_server_playground::Content::Text { text } = &result.content[0];
    let contacts: Vec<serde_json::Value> = serde_json::from_str(text).unwrap();
    assert!(!contacts.is_empty(), "Expected at least one contact matching 'tech'");
}

#[tokio::test]
async fn test_contacts_by_tag() {
    let t = tool();
    let result = t
        .execute(serde_json::json!({"action": "contacts_by_tag", "tag": "doctor"}))
        .await
        .unwrap();
    let mcp_server_playground::Content::Text { text } = &result.content[0];
    let contacts: Vec<serde_json::Value> = serde_json::from_str(text).unwrap();
    assert!(!contacts.is_empty(), "Expected at least one contact with 'doctor' tag");
    for c in &contacts {
        let tags = c["tags"].as_array().unwrap();
        assert!(tags.iter().any(|t| t.as_str() == Some("doctor")));
    }
}

#[tokio::test]
async fn test_favorite_contacts() {
    let t = tool();
    let result = t
        .execute(serde_json::json!({"action": "favorite_contacts"}))
        .await
        .unwrap();
    let mcp_server_playground::Content::Text { text } = &result.content[0];
    let contacts: Vec<serde_json::Value> = serde_json::from_str(text).unwrap();
    assert!(contacts.len() >= 2, "Expected at least 2 favorite contacts");
    for c in &contacts {
        assert_eq!(c["favorite"], true);
    }
}

#[tokio::test]
async fn test_invalid_action() {
    let t = tool();
    let result = t
        .execute(serde_json::json!({"action": "delete_contact"}))
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
fn test_contacts_tool_is_send_sync() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<ContactsTool>();
}

#[test]
fn test_tool_definition() {
    let t = tool();
    let def = t.to_definition();
    assert_eq!(def.name, "contacts");
    assert!(!def.description.is_empty());
    assert_eq!(def.input_schema.schema_type, "object");
    assert!(def.input_schema.properties.is_some());
    let props = def.input_schema.properties.as_ref().unwrap();
    assert!(props.get("action").is_some());
}
