//! Contact postal address.

use serde::{Deserialize, Serialize};

/// A postal address associated with a contact.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Address {
    #[serde(rename = "type")]
    pub address_type: String,

    pub street: String,
    pub city: String,
    pub state: String,
    pub postal_code: String,
    pub country: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary: Option<bool>,
}
