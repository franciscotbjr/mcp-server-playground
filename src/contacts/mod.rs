//! Contacts domain types for deserializing `contacts.json`.

mod address;
mod contact;
mod contact_email;
mod contacts_data;
mod contacts_metadata;
mod contacts_tool;
mod phone_number;
mod queries;
mod social_profile;

pub use contacts_tool::ContactsTool;
pub use address::Address;
pub use contact::Contact;
pub use contact_email::ContactEmail;
pub use contacts_data::ContactsData;
pub use contacts_metadata::ContactsMetadata;
pub use phone_number::PhoneNumber;
pub use social_profile::SocialProfile;
