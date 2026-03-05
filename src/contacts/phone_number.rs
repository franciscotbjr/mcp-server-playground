//! Contact phone number.

use serde::{Deserialize, Serialize};

/// A phone number associated with a contact.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PhoneNumber {
    #[serde(rename = "type")]
    pub phone_type: String,

    pub number: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary: Option<bool>,
}
