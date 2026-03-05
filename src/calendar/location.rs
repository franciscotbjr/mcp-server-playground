//! Calendar event location and coordinates.

use serde::{Deserialize, Serialize};

/// Geographic coordinates (latitude/longitude).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Coordinates {
    pub latitude: f64,
    pub longitude: f64,
}

/// Location of a calendar event (physical or virtual).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Location {
    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,

    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub coordinates: Option<Coordinates>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
