//! Calendar event recurrence rule.

use serde::{Deserialize, Serialize};

/// Recurrence rule for a repeating calendar event.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Recurrence {
    pub frequency: String,
    pub interval: u32,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub until: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub days_of_week: Option<Vec<String>>,
}
