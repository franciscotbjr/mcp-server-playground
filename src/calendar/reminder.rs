//! Calendar event reminder.

use serde::{Deserialize, Serialize};

/// A reminder for a calendar event.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Reminder {
    pub method: String,
    pub minutes: u32,
}
