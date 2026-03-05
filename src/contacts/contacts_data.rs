//! Root contacts data structure for deserializing `contacts.json`.

use serde::{Deserialize, Serialize};

use super::{Contact, ContactsMetadata};

/// Top-level structure matching the `contacts.json` layout.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContactsData {
    pub contacts: Vec<Contact>,
    pub metadata: ContactsMetadata,
}
