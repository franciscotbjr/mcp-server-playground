# Implementation Plan: Phase 2 — Domain Types

## Overview

Define calendar and contacts domain types (one type per file) for deserializing the simulated JSON data files.

## Status: IN PROGRESS

## Calendar Types — COMPLETE

### Files created (10 + facade)

| File | Types | Status |
|------|-------|--------|
| `calendar/mod.rs` | Facade: mod + pub use | ✅ |
| `calendar/reminder.rs` | `Reminder` | ✅ |
| `calendar/attendee.rs` | `Attendee` | ✅ |
| `calendar/recurrence.rs` | `Recurrence` | ✅ |
| `calendar/location.rs` | `Location`, `Coordinates` | ✅ |
| `calendar/attachment.rs` | `Attachment` | ✅ |
| `calendar/cost.rs` | `Cost` | ✅ |
| `calendar/calendar_metadata.rs` | `CalendarMetadata` | ✅ |
| `calendar/calendar_settings.rs` | `CalendarSettings`, `WorkingHours`, `CategoryConfig` | ✅ |
| `calendar/event.rs` | `Event` | ✅ |
| `calendar/calendar_data.rs` | `CalendarRoot`, `CalendarData` | ✅ |

### Tests: `tests/calendar_types_tests.rs` (12 tests) ✅

### Example: `examples/calendar_data.rs` ✅

### Spec fix: `spec/api-analysis.md` updated to match actual JSON ✅

## Contacts Types — COMPLETE

### Files created (7 + facade)

| File | Types | Status |
|------|-------|--------|
| `contacts/mod.rs` | Facade: mod + pub use | ✅ |
| `contacts/phone_number.rs` | `PhoneNumber` | ✅ |
| `contacts/contact_email.rs` | `ContactEmail` | ✅ |
| `contacts/address.rs` | `Address` | ✅ |
| `contacts/social_profile.rs` | `SocialProfile` | ✅ |
| `contacts/contacts_metadata.rs` | `ContactsMetadata` | ✅ |
| `contacts/contact.rs` | `Contact` | ✅ |
| `contacts/contacts_data.rs` | `ContactsData` | ✅ |

### Tests: `tests/contacts_types_tests.rs` (11 tests) ✅

### Example: `examples/contacts_data.rs` ✅

### Spec fix: `spec/api-analysis.md` contacts section updated ✅
