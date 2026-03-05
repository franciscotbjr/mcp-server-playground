//! Tests for calendar domain types.

use mcp_server_playground::calendar::{
    Attachment, Attendee, CalendarData, CalendarMetadata, CalendarRoot, CalendarSettings,
    CategoryConfig, Coordinates, Cost, Event, Location, Recurrence, Reminder, WorkingHours,
};

fn load_calendar() -> CalendarRoot {
    let json = std::fs::read_to_string("calendar.json").expect("calendar.json not found");
    serde_json::from_str(&json).expect("failed to deserialize calendar.json")
}

#[test]
fn test_deserialize_calendar_json() {
    let root = load_calendar();
    assert_eq!(root.calendar.name, "Personal Calendar");
    assert_eq!(root.calendar.time_zone, "America/New_York");
    assert_eq!(root.calendar.owner, "user@example.com");
}

#[test]
fn test_event_count() {
    let root = load_calendar();
    assert_eq!(root.calendar.events.len(), 12);
    assert_eq!(root.calendar.metadata.total_events, 12);
}

#[test]
fn test_event_with_location() {
    let root = load_calendar();
    let evt = root.calendar.events.iter().find(|e| e.id == "evt_001").unwrap();
    let loc = evt.location.as_ref().unwrap();
    assert_eq!(loc.name, "City Medical Center");
    assert!(loc.address.is_some());
    let coords = loc.coordinates.as_ref().unwrap();
    assert!((coords.latitude - 41.8781).abs() < 0.001);
    assert!((coords.longitude - (-87.6298)).abs() < 0.001);
}

#[test]
fn test_event_with_attendees() {
    let root = load_calendar();
    let evt = root.calendar.events.iter().find(|e| e.id == "evt_002").unwrap();
    let attendees = evt.attendees.as_ref().unwrap();
    assert_eq!(attendees.len(), 1);
    assert_eq!(attendees[0].contact_id, "c001");
    assert_eq!(attendees[0].name, "Tiger Brilliant");
    assert_eq!(attendees[0].email.as_deref(), Some("t.brilliant@techsolutions.com"));
    assert_eq!(attendees[0].status, "accepted");
    assert_eq!(attendees[0].attendee_type, "required");
}

#[test]
fn test_event_with_recurrence() {
    let root = load_calendar();
    let evt = root.calendar.events.iter().find(|e| e.id == "evt_005").unwrap();
    let rec = evt.recurrence.as_ref().unwrap();
    assert_eq!(rec.frequency, "monthly");
    assert_eq!(rec.interval, 1);
    assert_eq!(rec.days_of_week.as_ref().unwrap(), &["Sunday"]);
}

#[test]
fn test_event_with_attachments() {
    let root = load_calendar();
    let evt = root.calendar.events.iter().find(|e| e.id == "evt_003").unwrap();
    let attachments = evt.attachments.as_ref().unwrap();
    assert_eq!(attachments.len(), 1);
    assert_eq!(attachments[0].name, "Design_Mockups_v2.figma");
    assert_eq!(attachments[0].attachment_type, "link");
    assert!(attachments[0].url.is_some());
}

#[test]
fn test_event_with_cost() {
    let root = load_calendar();
    let evt = root.calendar.events.iter().find(|e| e.id == "evt_011").unwrap();
    let cost = evt.cost.as_ref().unwrap();
    assert!((cost.amount - 89.99).abs() < 0.01);
    assert_eq!(cost.currency, "USD");
}

#[test]
fn test_event_all_day() {
    let root = load_calendar();
    let evt = root.calendar.events.iter().find(|e| e.id == "evt_008").unwrap();
    assert!(evt.all_day);
    assert_eq!(evt.category, "birthday");
}

#[test]
fn test_settings_categories() {
    let root = load_calendar();
    let cats = &root.calendar.settings.categories;
    assert_eq!(cats.len(), 7);
    assert!(cats.contains_key("work"));
    assert!(cats.contains_key("birthday"));
    assert_eq!(cats["work"].icon, "briefcase");
}

#[test]
fn test_settings_working_hours() {
    let root = load_calendar();
    let wh = &root.calendar.settings.working_hours;
    assert_eq!(wh.start, "09:00");
    assert_eq!(wh.end, "18:00");
    assert_eq!(wh.working_days.len(), 5);
}

#[test]
fn test_metadata_fields() {
    let root = load_calendar();
    let meta = &root.calendar.metadata;
    assert_eq!(meta.total_events, 12);
    assert_eq!(meta.version, "2.1");
    assert_eq!(meta.platform, "multi-device");
    assert!(meta.sync_enabled);
}

#[test]
fn test_calendar_types_are_send_sync() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<CalendarRoot>();
    assert_send_sync::<CalendarData>();
    assert_send_sync::<Event>();
    assert_send_sync::<Location>();
    assert_send_sync::<Coordinates>();
    assert_send_sync::<Attendee>();
    assert_send_sync::<Recurrence>();
    assert_send_sync::<Reminder>();
    assert_send_sync::<Attachment>();
    assert_send_sync::<Cost>();
    assert_send_sync::<CalendarSettings>();
    assert_send_sync::<WorkingHours>();
    assert_send_sync::<CategoryConfig>();
    assert_send_sync::<CalendarMetadata>();
}
