# Project Definition: mcp-server-playground

## Overview

A Rust-based MCP (Model Context Protocol) server that exposes two tools for AI agents:

1. **Calendar Tool** — Query and search calendar events from simulated data
2. **Contacts Tool** — Query and search contacts from simulated data

Data is simulated via local JSON files (`calendar.json`, `contacts.json`) rather than real service integrations.

The server communicates over **SSE (Server-Sent Events)** transport via HTTP, following the MCP specification (2024-11-05). Two endpoints are exposed: `GET /sse` (SSE stream) and `POST /message` (client→server JSON-RPC).

## Goals

- Implement a fully functional MCP server over SSE transport
- Implement the full MCP `initialize` lifecycle (3-step handshake)
- Expose two tools following the MCP specification (2024-11-05)
- Provide rich query capabilities (list, get by ID, search, filter)
- Follow the design-source methodology (spec-first, one-type-per-file, with-method chain, etc.)
- Serve as a learning/playground project for MCP server development in Rust

## Non-Goals

- Real calendar/contacts service integration (Google Calendar, Outlook, etc.)
- Stdio transport (replaced by SSE)
- Persistent state or write operations (read-only tools)
- Authentication or authorization

## Phases

| Phase | Version | Focus |
|-------|---------|-------|
| Foundation | v0.1.0 | Project structure, error handling, MCP protocol types, SSE transport, session management, initialize lifecycle, server bootstrap, tests, example, documentation |
| Domain Types | v0.2.0 | Calendar and contacts data types (one per file), deserialization, with-method chain |
| Tool Implementation | v0.3.0 | McpTool implementations, query logic, tool registry wiring, integration tests |
| Polish | v0.4.0 | Robustness, examples, documentation polish, release checklist |

## Tools Specification

### Calendar Tool (`calendar`)

Single tool with `action` parameter to select the operation:

| Action | Description | Required Args | Optional Args |
|--------|-------------|---------------|---------------|
| `list_events` | List all events | — | `category`, `priority`, `status` |
| `get_event` | Get event by ID | `event_id` | — |
| `search_events` | Full-text search | `query` | — |
| `events_by_date` | Events on a date | `date` | — |
| `events_by_category` | Filter by category | `category` | — |
| `upcoming_events` | Next N events | — | `count` (default: 5) |

### Contacts Tool (`contacts`)

Single tool with `action` parameter to select the operation:

| Action | Description | Required Args | Optional Args |
|--------|-------------|---------------|---------------|
| `list_contacts` | List all contacts | — | `tag`, `company` |
| `get_contact` | Get contact by ID | `contact_id` | — |
| `search_contacts` | Full-text search | `query` | — |
| `contacts_by_tag` | Filter by tag | `tag` | — |
| `favorite_contacts` | List favorites | — | — |

## Data Sources

- `calendar.json` — 12 events with attendees, locations, recurrence, reminders
- `contacts.json` — 17 contacts with phone numbers, emails, addresses, social profiles
- Cross-reference: event attendees have `contactId` linking to contacts
