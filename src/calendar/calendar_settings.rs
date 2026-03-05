//! Calendar settings, working hours, and category configuration.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::Reminder;

/// Working hours configuration.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkingHours {
    pub start: String,
    pub end: String,
    pub working_days: Vec<String>,
}

/// Display configuration for an event category.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CategoryConfig {
    pub color: String,
    pub icon: String,
}

/// Calendar-wide settings.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CalendarSettings {
    pub default_reminders: Vec<Reminder>,
    pub default_duration: u32,
    pub working_hours: WorkingHours,
    pub categories: HashMap<String, CategoryConfig>,
}
