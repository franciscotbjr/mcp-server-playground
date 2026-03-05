//! Contacts metadata.

use serde::{Deserialize, Serialize};

/// Metadata about the contacts data file.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContactsMetadata {
    pub total_contacts: u32,
    pub last_sync: String,
    pub version: String,
    pub source: String,
}
