//! Contact email address.

use serde::{Deserialize, Serialize};

/// An email address associated with a contact.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContactEmail {
    #[serde(rename = "type")]
    pub email_type: String,

    pub address: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary: Option<bool>,
}
