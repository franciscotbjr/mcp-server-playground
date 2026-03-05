//! Contact record.

use serde::{Deserialize, Serialize};

use super::{Address, ContactEmail, PhoneNumber, SocialProfile};

/// A contact record with personal and professional information.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Contact {
    pub id: String,
    pub first_name: String,
    pub last_name: String,
    pub display_name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub nickname: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub company: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_title: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub department: Option<String>,

    pub phone_numbers: Vec<PhoneNumber>,
    pub emails: Vec<ContactEmail>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addresses: Option<Vec<Address>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub social_profiles: Option<Vec<SocialProfile>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub birthday: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo: Option<String>,

    pub tags: Vec<String>,
    pub favorite: bool,
    pub created_at: String,
    pub updated_at: String,
}
