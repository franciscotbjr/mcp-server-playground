//! Pure query functions over calendar data.

use super::{CalendarData, Event};

/// List events with optional filters for category, priority, and status.
pub fn list_events<'a>(
    data: &'a CalendarData,
    category: Option<&str>,
    priority: Option<&str>,
    status: Option<&str>,
) -> Vec<&'a Event> {
    data.events
        .iter()
        .filter(|e| category.is_none_or(|c| e.category.eq_ignore_ascii_case(c)))
        .filter(|e| priority.is_none_or(|p| e.priority.eq_ignore_ascii_case(p)))
        .filter(|e| status.is_none_or(|s| e.status.eq_ignore_ascii_case(s)))
        .collect()
}

/// Find a single event by its ID.
pub fn get_event<'a>(data: &'a CalendarData, event_id: &str) -> Option<&'a Event> {
    data.events.iter().find(|e| e.id == event_id)
}

/// Search events by keyword (case-insensitive match on title and description).
pub fn search_events<'a>(data: &'a CalendarData, query: &str) -> Vec<&'a Event> {
    let q = query.to_lowercase();
    data.events
        .iter()
        .filter(|e| {
            e.title.to_lowercase().contains(&q) || e.description.to_lowercase().contains(&q)
        })
        .collect()
}

/// Find events whose `startDateTime` begins with the given date prefix (e.g. "2025-08-28").
pub fn events_by_date<'a>(data: &'a CalendarData, date: &str) -> Vec<&'a Event> {
    data.events
        .iter()
        .filter(|e| e.start_date_time.starts_with(date))
        .collect()
}

/// Filter events by exact category match (case-insensitive).
pub fn events_by_category<'a>(data: &'a CalendarData, category: &str) -> Vec<&'a Event> {
    data.events
        .iter()
        .filter(|e| e.category.eq_ignore_ascii_case(category))
        .collect()
}

/// Return the first `count` events sorted by `startDateTime` ascending.
pub fn upcoming_events(data: &CalendarData, count: usize) -> Vec<&Event> {
    let mut events: Vec<&Event> = data.events.iter().collect();
    events.sort_by(|a, b| a.start_date_time.cmp(&b.start_date_time));
    events.into_iter().take(count).collect()
}
