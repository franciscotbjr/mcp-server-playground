# API Analysis: MCP Tools

## Tool Complexity Assessment

| Tool | Complexity | Reason |
|------|-----------|--------|
| Calendar | Medium | Multiple query actions, date parsing, nested types (location, attendees, recurrence) |
| Contacts | Medium | Multiple query actions, nested types (phones, emails, addresses, social profiles) |

## Calendar Tool — Data Shape

```
CalendarRoot                          # Top-level JSON wrapper
└── calendar: CalendarData
    ├── name: String
    ├── timeZone: String
    ├── owner: String
    ├── events: Vec<Event>
    │   ├── id: String
    │   ├── title: String
    │   ├── description: String
    │   ├── startDateTime: String (ISO 8601)
    │   ├── endDateTime: String (ISO 8601)
    │   ├── location: Option<Location>
    │   │   ├── name: String
    │   │   ├── address: Option<String>
    │   │   ├── type: Option<String>
    │   │   ├── coordinates: Option<Coordinates>
    │   │   │   ├── latitude: f64
    │   │   │   └── longitude: f64
    │   │   └── url: Option<String>
    │   ├── attendees: Option<Vec<Attendee>>
    │   │   ├── contactId: String
    │   │   ├── name: String
    │   │   ├── email: Option<String>
    │   │   ├── status: String
    │   │   └── type: String
    │   ├── category: String
    │   ├── priority: String
    │   ├── status: String
    │   ├── allDay: bool
    │   ├── recurrence: Option<Recurrence>
    │   │   ├── frequency: String
    │   │   ├── interval: u32
    │   │   ├── until: Option<String>
    │   │   └── daysOfWeek: Option<Vec<String>>
    │   ├── reminders: Vec<Reminder>
    │   │   ├── method: String
    │   │   └── minutes: u32
    │   ├── attachments: Option<Vec<Attachment>>
    │   │   ├── name: String
    │   │   ├── type: String
    │   │   ├── url: Option<String>
    │   │   └── size: Option<String>
    │   ├── cost: Option<Cost>
    │   │   ├── amount: f64
    │   │   └── currency: String
    │   ├── notes: Option<String>
    │   ├── color: Option<String>
    │   ├── createdAt: String
    │   └── updatedAt: String
    ├── settings: CalendarSettings
    │   ├── defaultReminders: Vec<Reminder>
    │   ├── defaultDuration: u32
    │   ├── workingHours: WorkingHours
    │   │   ├── start: String
    │   │   ├── end: String
    │   │   └── workingDays: Vec<String>
    │   └── categories: HashMap<String, CategoryConfig>
    │       ├── color: String
    │       └── icon: String
    └── metadata: CalendarMetadata
        ├── totalEvents: u32
        ├── lastSync: String
        ├── version: String
        ├── platform: String
        └── syncEnabled: bool
```

## Contacts Tool — Data Shape

```
ContactsData                          # Top-level JSON (no wrapper key)
├── contacts: Vec<Contact>
│   ├── id: String
│   ├── firstName: String
│   ├── lastName: String
│   ├── displayName: String
│   ├── nickname: Option<String>
│   ├── company: Option<String>
│   ├── jobTitle: Option<String>
│   ├── department: Option<String>
│   ├── phoneNumbers: Vec<PhoneNumber>
│   │   ├── type: String
│   │   ├── number: String
│   │   └── primary: Option<bool>
│   ├── emails: Vec<ContactEmail>
│   │   ├── type: String
│   │   ├── address: String
│   │   └── primary: Option<bool>
│   ├── addresses: Option<Vec<Address>>
│   │   ├── type: String
│   │   ├── street: String
│   │   ├── city: String
│   │   ├── state: String
│   │   ├── postalCode: String
│   │   ├── country: String
│   │   └── primary: Option<bool>
│   ├── socialProfiles: Option<Vec<SocialProfile>>
│   │   ├── platform: String
│   │   ├── url: String
│   │   └── username: String
│   ├── birthday: Option<String>
│   ├── notes: Option<String>
│   ├── photo: Option<String>
│   ├── tags: Vec<String>
│   ├── favorite: bool
│   ├── createdAt: String
│   └── updatedAt: String
└── metadata: ContactsMetadata
    ├── totalContacts: u32
    ├── lastSync: String
    ├── version: String
    └── source: String
```

## Dependencies Between Tools

- Event `attendees[].contactId` references `contacts[].id`
- This cross-reference is informational only; tools operate independently

## Implementation Order

1. Calendar types (more complex due to date handling)
2. Contacts types
3. Calendar tool + queries
4. Contacts tool + queries
