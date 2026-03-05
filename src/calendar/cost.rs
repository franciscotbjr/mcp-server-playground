//! Calendar event cost.

use serde::{Deserialize, Serialize};

/// Monetary cost associated with a calendar event.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Cost {
    pub amount: f64,
    pub currency: String,
}
