//! Calendar event attendee.

use serde::{Deserialize, Serialize};

/// An attendee of a calendar event, linked to a contact by `contact_id`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Attendee {
    pub contact_id: String,
    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    pub status: String,

    #[serde(rename = "type")]
    pub attendee_type: String,
}
