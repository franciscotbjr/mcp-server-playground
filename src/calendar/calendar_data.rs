//! Root calendar data structures for deserializing `calendar.json`.

use serde::{Deserialize, Serialize};

use super::{CalendarMetadata, CalendarSettings, Event};

/// The inner calendar object containing events, settings, and metadata.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CalendarData {
    pub name: String,
    pub time_zone: String,
    pub owner: String,
    pub events: Vec<Event>,
    pub settings: CalendarSettings,
    pub metadata: CalendarMetadata,
}

/// Top-level wrapper matching the `{"calendar": ...}` JSON structure.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CalendarRoot {
    pub calendar: CalendarData,
}
