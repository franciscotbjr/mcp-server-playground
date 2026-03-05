//! Calendar event.

use serde::{Deserialize, Serialize};

use super::{Attachment, Attendee, Cost, Location, Recurrence, Reminder};

/// A calendar event with all associated metadata.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Event {
    pub id: String,
    pub title: String,
    pub description: String,
    pub start_date_time: String,
    pub end_date_time: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attendees: Option<Vec<Attendee>>,

    pub category: String,
    pub priority: String,
    pub status: String,
    pub all_day: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurrence: Option<Recurrence>,

    pub reminders: Vec<Reminder>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<Attachment>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost: Option<Cost>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,

    pub created_at: String,
    pub updated_at: String,
}
