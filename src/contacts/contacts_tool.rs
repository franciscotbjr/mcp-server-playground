//! ContactsTool — MCP tool implementation for contacts queries.

use async_trait::async_trait;
use serde_json::Value;

use crate::error::{Error, Result};
use crate::mcp::tools::CallToolResult;
use crate::mcp::tools::McpTool;
use super::contacts_data::ContactsData;
use super::queries;

/// MCP tool that exposes contacts query operations.
///
/// Loads `contacts.json` once at construction and holds the data in memory.
/// A single `action` argument dispatches to one of five query functions.
#[derive(Debug)]
pub struct ContactsTool {
    data: ContactsData,
}

impl ContactsTool {
    /// Create a new `ContactsTool` by loading and parsing the JSON file at `path`.
    pub fn new(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path).map_err(|e| {
            Error::DataNotFoundError(format!("Failed to read {path}: {e}"))
        })?;
        let data: ContactsData = serde_json::from_str(&json)?;
        Ok(Self { data })
    }

    /// Number of contacts loaded.
    pub fn contact_count(&self) -> usize {
        self.data.contacts.len()
    }
}

#[async_trait]
impl McpTool for ContactsTool {
    fn name(&self) -> &str {
        "contacts"
    }

    fn description(&self) -> &str {
        "Query contacts. Actions: list_contacts, get_contact, search_contacts, contacts_by_tag, favorite_contacts"
    }

    fn input_schema(&self) -> Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "action": {
                    "type": "string",
                    "enum": ["list_contacts", "get_contact", "search_contacts", "contacts_by_tag", "favorite_contacts"],
                    "description": "The query action to perform"
                },
                "contact_id": {
                    "type": "string",
                    "description": "Contact ID (required for get_contact)"
                },
                "query": {
                    "type": "string",
                    "description": "Search text (required for search_contacts)"
                },
                "tag": {
                    "type": "string",
                    "description": "Tag filter (required for contacts_by_tag, optional for list_contacts)"
                },
                "company": {
                    "type": "string",
                    "description": "Company filter (optional for list_contacts)"
                },
                "favorite": {
                    "type": "boolean",
                    "description": "Favorite filter (optional for list_contacts)"
                }
            },
            "required": ["action"]
        })
    }

    async fn execute(&self, arguments: Value) -> Result<CallToolResult> {
        let action = arguments
            .get("action")
            .and_then(|v| v.as_str())
            .ok_or_else(|| Error::ToolError("Missing required argument: action".to_string()))?;

        match action {
            "list_contacts" => {
                let tag = arguments.get("tag").and_then(|v| v.as_str());
                let favorite = arguments.get("favorite").and_then(|v| v.as_bool());
                let company = arguments.get("company").and_then(|v| v.as_str());
                let contacts = queries::list_contacts(&self.data, tag, favorite, company);
                let json = serde_json::to_string_pretty(&contacts)?;
                Ok(CallToolResult::text(json))
            }
            "get_contact" => {
                let contact_id = arguments
                    .get("contact_id")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| {
                        Error::ToolError("Missing required argument: contact_id".to_string())
                    })?;
                match queries::get_contact(&self.data, contact_id) {
                    Some(contact) => {
                        let json = serde_json::to_string_pretty(contact)?;
                        Ok(CallToolResult::text(json))
                    }
                    None => Ok(CallToolResult::error(format!(
                        "Contact not found: {contact_id}"
                    ))),
                }
            }
            "search_contacts" => {
                let query = arguments
                    .get("query")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| {
                        Error::ToolError("Missing required argument: query".to_string())
                    })?;
                let contacts = queries::search_contacts(&self.data, query);
                let json = serde_json::to_string_pretty(&contacts)?;
                Ok(CallToolResult::text(json))
            }
            "contacts_by_tag" => {
                let tag = arguments
                    .get("tag")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| {
                        Error::ToolError("Missing required argument: tag".to_string())
                    })?;
                let contacts = queries::contacts_by_tag(&self.data, tag);
                let json = serde_json::to_string_pretty(&contacts)?;
                Ok(CallToolResult::text(json))
            }
            "favorite_contacts" => {
                let contacts = queries::favorite_contacts(&self.data);
                let json = serde_json::to_string_pretty(&contacts)?;
                Ok(CallToolResult::text(json))
            }
            _ => Ok(CallToolResult::error(format!("Unknown action: {action}"))),
        }
    }
}
