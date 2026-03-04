//! Content block inside a tool result.

use serde::{Deserialize, Serialize};

/// Content block inside a tool result.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Content {
    #[serde(rename = "text")]
    Text { text: String },
}

impl Content {
    /// Create a text content block.
    pub fn text(text: impl Into<String>) -> Self {
        Content::Text { text: text.into() }
    }
}
