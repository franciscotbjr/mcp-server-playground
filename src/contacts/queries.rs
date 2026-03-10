//! Pure query functions over contacts data.

use super::{Contact, ContactsData};

/// List contacts with optional filters for tag, favorite, and company.
pub fn list_contacts<'a>(
    data: &'a ContactsData,
    tag: Option<&str>,
    favorite: Option<bool>,
    company: Option<&str>,
) -> Vec<&'a Contact> {
    data.contacts
        .iter()
        .filter(|c| tag.is_none_or(|t| c.tags.iter().any(|ct| ct.eq_ignore_ascii_case(t))))
        .filter(|c| favorite.is_none_or(|f| c.favorite == f))
        .filter(|c| {
            company.is_none_or(|co| {
                c.company
                    .as_deref()
                    .is_some_and(|comp| comp.eq_ignore_ascii_case(co))
            })
        })
        .collect()
}

/// Find a single contact by its ID.
pub fn get_contact<'a>(data: &'a ContactsData, contact_id: &str) -> Option<&'a Contact> {
    data.contacts.iter().find(|c| c.id == contact_id)
}

/// Search contacts by keyword (case-insensitive match on displayName, company, and notes).
pub fn search_contacts<'a>(data: &'a ContactsData, query: &str) -> Vec<&'a Contact> {
    let q = query.to_lowercase();
    data.contacts
        .iter()
        .filter(|c| {
            c.display_name.to_lowercase().contains(&q)
                || c.company
                    .as_deref()
                    .is_some_and(|co| co.to_lowercase().contains(&q))
                || c.notes
                    .as_deref()
                    .is_some_and(|n| n.to_lowercase().contains(&q))
        })
        .collect()
}

/// Filter contacts by exact tag match (case-insensitive).
pub fn contacts_by_tag<'a>(data: &'a ContactsData, tag: &str) -> Vec<&'a Contact> {
    data.contacts
        .iter()
        .filter(|c| c.tags.iter().any(|t| t.eq_ignore_ascii_case(tag)))
        .collect()
}

/// Return all favorite contacts.
pub fn favorite_contacts(data: &ContactsData) -> Vec<&Contact> {
    data.contacts.iter().filter(|c| c.favorite).collect()
}
