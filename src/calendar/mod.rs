//! Calendar domain types for deserializing `calendar.json`.

mod attachment;
mod attendee;
mod calendar_data;
mod calendar_metadata;
mod calendar_settings;
mod cost;
mod event;
mod location;
mod recurrence;
mod reminder;

pub use attachment::Attachment;
pub use attendee::Attendee;
pub use calendar_data::{CalendarData, CalendarRoot};
pub use calendar_metadata::CalendarMetadata;
pub use calendar_settings::{CalendarSettings, CategoryConfig, WorkingHours};
pub use cost::Cost;
pub use event::Event;
pub use location::{Coordinates, Location};
pub use recurrence::Recurrence;
pub use reminder::Reminder;
