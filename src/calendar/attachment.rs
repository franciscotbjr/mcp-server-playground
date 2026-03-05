//! Calendar event attachment.

use serde::{Deserialize, Serialize};

/// A file or link attached to a calendar event.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Attachment {
    pub name: String,

    #[serde(rename = "type")]
    pub attachment_type: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
}
