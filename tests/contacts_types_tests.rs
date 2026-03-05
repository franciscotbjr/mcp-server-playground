//! Tests for contacts domain types.

use mcp_server_playground::contacts::{
    Address, Contact, ContactEmail, ContactsData, ContactsMetadata, PhoneNumber, SocialProfile,
};

fn load_contacts() -> ContactsData {
    let json = std::fs::read_to_string("contacts.json").expect("contacts.json not found");
    serde_json::from_str(&json).expect("failed to deserialize contacts.json")
}

#[test]
fn test_deserialize_contacts_json() {
    let data = load_contacts();
    assert_eq!(data.contacts.len(), 17);
}

#[test]
fn test_contact_count() {
    let data = load_contacts();
    assert_eq!(data.metadata.total_contacts, 17);
    assert_eq!(data.contacts.len() as u32, data.metadata.total_contacts);
}

#[test]
fn test_contact_with_all_fields() {
    let data = load_contacts();
    let c = data.contacts.iter().find(|c| c.id == "c001").unwrap();
    assert_eq!(c.first_name, "Tiger");
    assert_eq!(c.last_name, "Brilliant");
    assert_eq!(c.display_name, "Tiger Brilliant");
    assert_eq!(c.nickname.as_deref(), Some("Tiggy"));
    assert_eq!(c.company.as_deref(), Some("Tech Solutions Inc."));
    assert_eq!(c.job_title.as_deref(), Some("Software Engineer"));
    assert_eq!(c.department.as_deref(), Some("Engineering"));
    assert_eq!(c.birthday.as_deref(), Some("1985-06-15"));
    assert_eq!(c.photo.as_deref(), Some("https://example.com/photos/tiger_brilliant.jpg"));
    assert!(c.favorite);
}

#[test]
fn test_contact_minimal() {
    let data = load_contacts();
    let c = data.contacts.iter().find(|c| c.id == "c005").unwrap();
    assert_eq!(c.display_name, "Roo Cheerful");
    assert!(c.company.is_none());
    assert!(c.job_title.is_none());
    assert!(c.addresses.is_none());
    assert!(c.social_profiles.is_none());
    assert!(c.photo.is_none());
}

#[test]
fn test_contact_phone_numbers() {
    let data = load_contacts();
    let c = data.contacts.iter().find(|c| c.id == "c001").unwrap();
    assert_eq!(c.phone_numbers.len(), 2);
    assert_eq!(c.phone_numbers[0].phone_type, "mobile");
    assert_eq!(c.phone_numbers[0].number, "+1-555-123-4567");
    assert_eq!(c.phone_numbers[0].primary, Some(true));
    assert_eq!(c.phone_numbers[1].phone_type, "work");
    assert_eq!(c.phone_numbers[1].primary, Some(false));
}

#[test]
fn test_contact_emails() {
    let data = load_contacts();
    let c = data.contacts.iter().find(|c| c.id == "c004").unwrap();
    assert_eq!(c.emails.len(), 2);
    assert_eq!(c.emails[0].email_type, "work");
    assert_eq!(c.emails[0].address, "penguin@swiftrealestate.com");
    assert_eq!(c.emails[0].primary, Some(true));
    assert_eq!(c.emails[1].email_type, "personal");
    assert_eq!(c.emails[1].primary, Some(false));
}

#[test]
fn test_contact_addresses() {
    let data = load_contacts();
    let c = data.contacts.iter().find(|c| c.id == "c001").unwrap();
    let addrs = c.addresses.as_ref().unwrap();
    assert_eq!(addrs.len(), 1);
    assert_eq!(addrs[0].address_type, "home");
    assert_eq!(addrs[0].street, "123 Main Street");
    assert_eq!(addrs[0].city, "New York");
    assert_eq!(addrs[0].state, "NY");
    assert_eq!(addrs[0].postal_code, "10001");
    assert_eq!(addrs[0].country, "USA");
    assert_eq!(addrs[0].primary, Some(true));
}

#[test]
fn test_contact_social_profiles() {
    let data = load_contacts();
    let c = data.contacts.iter().find(|c| c.id == "c001").unwrap();
    let profiles = c.social_profiles.as_ref().unwrap();
    assert_eq!(profiles.len(), 2);
    assert_eq!(profiles[0].platform, "linkedin");
    assert_eq!(profiles[0].url, "https://linkedin.com/in/tigerbrilliant");
    assert_eq!(profiles[0].username, "tigerbrilliant");
}

#[test]
fn test_contact_favorites() {
    let data = load_contacts();
    let favorites: Vec<_> = data.contacts.iter().filter(|c| c.favorite).collect();
    assert_eq!(favorites.len(), 7);
}

#[test]
fn test_metadata_fields() {
    let data = load_contacts();
    assert_eq!(data.metadata.total_contacts, 17);
    assert_eq!(data.metadata.version, "1.1");
    assert_eq!(data.metadata.source, "multi-platform-sync");
    assert!(!data.metadata.last_sync.is_empty());
}

#[test]
fn test_contacts_types_are_send_sync() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<ContactsData>();
    assert_send_sync::<Contact>();
    assert_send_sync::<PhoneNumber>();
    assert_send_sync::<ContactEmail>();
    assert_send_sync::<Address>();
    assert_send_sync::<SocialProfile>();
    assert_send_sync::<ContactsMetadata>();
}
