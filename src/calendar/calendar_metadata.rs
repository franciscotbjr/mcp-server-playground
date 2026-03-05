//! Calendar metadata.

use serde::{Deserialize, Serialize};

/// Metadata about the calendar data file.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CalendarMetadata {
    pub total_events: u32,
    pub last_sync: String,
    pub version: String,
    pub platform: String,
    pub sync_enabled: bool,
}
