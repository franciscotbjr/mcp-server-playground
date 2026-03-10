//! CalendarTool — MCP tool implementation for calendar queries.

use async_trait::async_trait;
use serde_json::Value;

use crate::error::{Error, Result};
use crate::mcp::tools::CallToolResult;
use crate::mcp::tools::McpTool;
use super::calendar_data::CalendarRoot;
use super::queries;

/// MCP tool that exposes calendar query operations.
///
/// Loads `calendar.json` once at construction and holds the data in memory.
/// A single `action` argument dispatches to one of six query functions.
#[derive(Debug)]
pub struct CalendarTool {
    root: CalendarRoot,
}

impl CalendarTool {
    /// Create a new `CalendarTool` by loading and parsing the JSON file at `path`.
    pub fn new(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path).map_err(|e| {
            Error::DataNotFoundError(format!("Failed to read {path}: {e}"))
        })?;
        let root: CalendarRoot = serde_json::from_str(&json)?;
        Ok(Self { root })
    }

    /// Number of events loaded.
    pub fn event_count(&self) -> usize {
        self.root.calendar.events.len()
    }
}

#[async_trait]
impl McpTool for CalendarTool {
    fn name(&self) -> &str {
        "calendar"
    }

    fn description(&self) -> &str {
        "Query calendar events. Actions: list_events, get_event, search_events, events_by_date, events_by_category, upcoming_events"
    }

    fn input_schema(&self) -> Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "action": {
                    "type": "string",
                    "enum": ["list_events", "get_event", "search_events", "events_by_date", "events_by_category", "upcoming_events"],
                    "description": "The query action to perform"
                },
                "event_id": {
                    "type": "string",
                    "description": "Event ID (required for get_event)"
                },
                "query": {
                    "type": "string",
                    "description": "Search text (required for search_events)"
                },
                "date": {
                    "type": "string",
                    "description": "Date prefix, e.g. '2025-08-28' (required for events_by_date)"
                },
                "category": {
                    "type": "string",
                    "description": "Category filter (required for events_by_category, optional for list_events)"
                },
                "priority": {
                    "type": "string",
                    "description": "Priority filter (optional for list_events)"
                },
                "status": {
                    "type": "string",
                    "description": "Status filter (optional for list_events)"
                },
                "count": {
                    "type": "integer",
                    "description": "Number of events to return (optional for upcoming_events, default: 5)"
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

        let cal = &self.root.calendar;

        match action {
            "list_events" => {
                let category = arguments.get("category").and_then(|v| v.as_str());
                let priority = arguments.get("priority").and_then(|v| v.as_str());
                let status = arguments.get("status").and_then(|v| v.as_str());
                let events = queries::list_events(cal, category, priority, status);
                let json = serde_json::to_string_pretty(&events)?;
                Ok(CallToolResult::text(json))
            }
            "get_event" => {
                let event_id = arguments
                    .get("event_id")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| {
                        Error::ToolError("Missing required argument: event_id".to_string())
                    })?;
                match queries::get_event(cal, event_id) {
                    Some(event) => {
                        let json = serde_json::to_string_pretty(event)?;
                        Ok(CallToolResult::text(json))
                    }
                    None => Ok(CallToolResult::error(format!(
                        "Event not found: {event_id}"
                    ))),
                }
            }
            "search_events" => {
                let query = arguments
                    .get("query")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| {
                        Error::ToolError("Missing required argument: query".to_string())
                    })?;
                let events = queries::search_events(cal, query);
                let json = serde_json::to_string_pretty(&events)?;
                Ok(CallToolResult::text(json))
            }
            "events_by_date" => {
                let date = arguments
                    .get("date")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| {
                        Error::ToolError("Missing required argument: date".to_string())
                    })?;
                let events = queries::events_by_date(cal, date);
                let json = serde_json::to_string_pretty(&events)?;
                Ok(CallToolResult::text(json))
            }
            "events_by_category" => {
                let category = arguments
                    .get("category")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| {
                        Error::ToolError("Missing required argument: category".to_string())
                    })?;
                let events = queries::events_by_category(cal, category);
                let json = serde_json::to_string_pretty(&events)?;
                Ok(CallToolResult::text(json))
            }
            "upcoming_events" => {
                let count = arguments
                    .get("count")
                    .and_then(|v| v.as_u64())
                    .map(|c| c as usize)
                    .unwrap_or(5);
                let events = queries::upcoming_events(cal, count);
                let json = serde_json::to_string_pretty(&events)?;
                Ok(CallToolResult::text(json))
            }
            _ => Ok(CallToolResult::error(format!("Unknown action: {action}"))),
        }
    }
}
