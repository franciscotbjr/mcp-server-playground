//! Demonstrates loading and querying calendar data from `calendar.json`.
//!
//! 1. Loads and deserializes the calendar JSON file
//! 2. Prints a summary of events, categories, and settings
//! 3. Shows filtering events by category and finding by ID
//!
//! Run with: `cargo run --example calendar_data`

use mcp_server_playground::calendar::CalendarRoot;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // --- Step 1: Load calendar.json ---
    println!("--- Step 1: Loading calendar.json ---");
    let json = std::fs::read_to_string("calendar.json")?;
    let root: CalendarRoot = serde_json::from_str(&json)?;
    let cal = &root.calendar;

    println!("Calendar: {} ({})", cal.name, cal.time_zone);
    println!("Owner: {}", cal.owner);
    println!("Total events: {}", cal.events.len());

    // --- Step 2: List all events ---
    println!("\n--- Step 2: All events ---");
    for evt in &cal.events {
        let loc = evt
            .location
            .as_ref()
            .map(|l| l.name.as_str())
            .unwrap_or("(no location)");
        println!(
            "  [{:>4}] {} | {} | {} | {}",
            evt.priority, evt.id, evt.category, evt.title, loc
        );
    }

    // --- Step 3: Filter by category ---
    println!("\n--- Step 3: Work events ---");
    let work_events: Vec<_> = cal.events.iter().filter(|e| e.category == "work").collect();
    println!("Found {} work events:", work_events.len());
    for evt in &work_events {
        println!("  - {} ({})", evt.title, evt.start_date_time);
    }

    // --- Step 4: Find event by ID ---
    println!("\n--- Step 4: Find event evt_001 ---");
    if let Some(evt) = cal.events.iter().find(|e| e.id == "evt_001") {
        println!("  Title: {}", evt.title);
        println!("  Category: {}", evt.category);
        println!("  All day: {}", evt.all_day);
        if let Some(loc) = &evt.location {
            println!("  Location: {}", loc.name);
            if let Some(coords) = &loc.coordinates {
                println!("  Coordinates: {}, {}", coords.latitude, coords.longitude);
            }
        }
        if let Some(attendees) = &evt.attendees {
            println!("  Attendees: {}", attendees.len());
            for a in attendees {
                println!("    - {} ({})", a.name, a.status);
            }
        }
    }

    // --- Step 5: Settings summary ---
    println!("\n--- Step 5: Settings ---");
    let wh = &cal.settings.working_hours;
    println!("  Working hours: {} - {} ({} days)", wh.start, wh.end, wh.working_days.len());
    println!("  Default duration: {} min", cal.settings.default_duration);
    println!("  Categories: {:?}", cal.settings.categories.keys().collect::<Vec<_>>());

    // --- Step 6: Metadata ---
    println!("\n--- Step 6: Metadata ---");
    println!("  Version: {}", cal.metadata.version);
    println!("  Platform: {}", cal.metadata.platform);
    println!("  Last sync: {}", cal.metadata.last_sync);

    Ok(())
}
