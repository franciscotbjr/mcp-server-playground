//! Contact social media profile.

use serde::{Deserialize, Serialize};

/// A social media profile associated with a contact.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SocialProfile {
    pub platform: String,
    pub url: String,
    pub username: String,
}
