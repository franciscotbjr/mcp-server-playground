//! Demonstrates loading and querying contacts data from `contacts.json`.
//!
//! 1. Loads and deserializes the contacts JSON file
//! 2. Prints a summary of contacts and metadata
//! 3. Shows filtering by tag, finding by ID, and listing favorites
//!
//! Run with: `cargo run --example contacts_data`

use mcp_server_playground::contacts::ContactsData;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // --- Step 1: Load contacts.json ---
    println!("--- Step 1: Loading contacts.json ---");
    let json = std::fs::read_to_string("contacts.json")?;
    let data: ContactsData = serde_json::from_str(&json)?;

    println!("Total contacts: {}", data.contacts.len());
    println!("Version: {}", data.metadata.version);
    println!("Source: {}", data.metadata.source);

    // --- Step 2: List all contacts ---
    println!("\n--- Step 2: All contacts ---");
    for c in &data.contacts {
        let company = c.company.as_deref().unwrap_or("(personal)");
        let fav = if c.favorite { "★" } else { " " };
        println!("  {} {} | {} | {}", fav, c.id, c.display_name, company);
    }

    // --- Step 3: Filter by tag ---
    println!("\n--- Step 3: Contacts tagged 'family' ---");
    let family: Vec<_> = data
        .contacts
        .iter()
        .filter(|c| c.tags.contains(&"family".to_string()))
        .collect();
    println!("Found {} family contacts:", family.len());
    for c in &family {
        println!("  - {} ({})", c.display_name, c.notes.as_deref().unwrap_or(""));
    }

    // --- Step 4: Find contact by ID ---
    println!("\n--- Step 4: Find contact c001 ---");
    if let Some(c) = data.contacts.iter().find(|c| c.id == "c001") {
        println!("  Name: {}", c.display_name);
        println!("  Company: {}", c.company.as_deref().unwrap_or("N/A"));
        println!("  Phones: {}", c.phone_numbers.len());
        for p in &c.phone_numbers {
            let primary = if p.primary == Some(true) { " (primary)" } else { "" };
            println!("    - {} {}{}", p.phone_type, p.number, primary);
        }
        println!("  Emails: {}", c.emails.len());
        for e in &c.emails {
            println!("    - {} {}", e.email_type, e.address);
        }
        if let Some(profiles) = &c.social_profiles {
            println!("  Social: {}", profiles.len());
            for s in profiles {
                println!("    - {} @{}", s.platform, s.username);
            }
        }
    }

    // --- Step 5: Favorites ---
    println!("\n--- Step 5: Favorite contacts ---");
    let favorites: Vec<_> = data.contacts.iter().filter(|c| c.favorite).collect();
    println!("Found {} favorites:", favorites.len());
    for c in &favorites {
        println!("  ★ {}", c.display_name);
    }

    Ok(())
}
