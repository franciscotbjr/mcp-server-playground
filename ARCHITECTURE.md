# Architecture

## Overview

`mcp-server-playground` is a Rust-based MCP (Model Context Protocol) server that exposes two tools — **calendar** and **contacts** — with simulated data backed by local JSON files.

The server communicates over **SSE (Server-Sent Events)** transport via HTTP, using the JSON-RPC 2.0 protocol as defined by the MCP specification (2024-11-05).

Two HTTP endpoints are exposed:

- **`GET /sse`** — Opens an SSE stream. The server sends an `endpoint` event with the URI for posting messages, then delivers all responses as `message` events.
- **`POST /message?sessionId=<uuid>`** — Receives JSON-RPC requests from the client.

## Module Structure

```
src/
├── main.rs               # Entry point — Axum HTTP server bootstrap
├── lib.rs                # Crate root — module declarations and re-exports
├── error.rs              # Centralized Error enum with {Type}Error suffix convention
└── mcp/                  # MCP protocol layer
    ├── mod.rs            # Facade: mod + pub use (re-exports from subdirectories)
    ├── handler.rs        # RequestHandler (JSON-RPC method dispatcher)
    ├── protocol/         # JSON-RPC 2.0 wire format
    │   ├── mod.rs        # Facade: mod + pub use
    │   ├── request.rs    # JsonRpcRequest
    │   ├── response.rs   # JsonRpcResponse
    │   ├── error.rs      # JsonRpcError
    │   └── notification.rs # JsonRpcNotification
    ├── tools/            # Tool abstraction, types, and registry
    │   ├── mod.rs        # Facade: mod + pub use
    │   ├── tool_trait.rs # McpTool trait (interface for MCP tools)
    │   ├── tool_registry.rs # ToolRegistry (collection of registered tools)
    │   ├── tool_definition.rs # ToolDefinition (tool metadata)
    │   ├── input_schema.rs # InputSchema (JSON Schema for tool input)
    │   ├── call_tool_result.rs # CallToolResult (tool invocation result)
    │   ├── call_tool_params.rs # CallToolParams (tools/call parameters)
    │   ├── list_tools_result.rs # ListToolsResult (tools/list result)
    │   └── content.rs    # Content (content block in tool results)
    └── transport/        # SSE transport, HTTP server, server identity
        ├── mod.rs        # Facade: mod + pub use
        ├── server.rs     # McpServer — HTTP bootstrap + graceful shutdown
        ├── sse_handler.rs # SSE endpoint handlers + lifecycle enforcement
        ├── session.rs    # SessionState, Session, SessionStore (per-client lifecycle)
        ├── app_state.rs  # AppState (shared state for Axum handlers)
        ├── message_query.rs # MessageQuery (POST /message query params)
        ├── initialize_result.rs # InitializeResult
        ├── server_capabilities.rs # ServerCapabilities
        ├── server_info.rs # ServerInfo
        └── tools_capability.rs # ToolsCapability

├── calendar/             # Calendar domain types
│   ├── mod.rs            # Facade: mod + pub use
│   ├── calendar_data.rs  # CalendarRoot, CalendarData (top-level wrappers)
│   ├── event.rs          # Event (main calendar event struct)
│   ├── location.rs       # Location, Coordinates
│   ├── attendee.rs       # Attendee
│   ├── recurrence.rs     # Recurrence
│   ├── reminder.rs       # Reminder
│   ├── attachment.rs     # Attachment
│   ├── cost.rs           # Cost
│   ├── calendar_settings.rs # CalendarSettings, WorkingHours, CategoryConfig
│   └── calendar_metadata.rs # CalendarMetadata

├── contacts/             # Contacts domain types
│   ├── mod.rs            # Facade: mod + pub use
│   ├── contacts_data.rs  # ContactsData (top-level)
│   ├── contact.rs        # Contact (main contact struct)
│   ├── phone_number.rs   # PhoneNumber
│   ├── contact_email.rs  # ContactEmail
│   ├── address.rs        # Address
│   ├── social_profile.rs # SocialProfile
│   └── contacts_metadata.rs # ContactsMetadata

tests/                    # Integration tests for public types
├── error_tests.rs        # Error enum tests (8 tests)
├── protocol_tests.rs     # JSON-RPC type tests (10 tests)
├── types_tests.rs        # MCP domain type tests (11 tests)
├── tool_registry_tests.rs # ToolRegistry tests (5 tests)
├── handler_tests.rs      # RequestHandler dispatch tests (8 tests)
├── calendar_types_tests.rs # Calendar domain type tests (12 tests)
└── contacts_types_tests.rs # Contacts domain type tests (11 tests)

examples/
├── initialize.rs         # Minimal MCP initialize lifecycle demo
├── calendar_data.rs      # Load and query calendar.json demo
└── contacts_data.rs      # Load and query contacts.json demo
```

## Key Design Decisions

- **Single crate, no feature flags** — project scope is small enough that feature gating adds unnecessary complexity.
- **SSE transport** — replaces stdio; follows MCP spec 2024-11-05 with `GET /sse` + `POST /message`.
- **Session management** — each SSE connection gets a UUID-identified session with lifecycle state tracking (`Uninitialized → Initializing → Ready`).
- **One type per file** — following the design-source methodology for clarity and maintainability.
- **Facade pattern** — each module's `mod.rs` contains only `mod` and `pub use` declarations.
- **Public type tests in `tests/`** — following design-source convention; `pub(crate)` tests remain inline.
- **With-method chain** — preferred over builder pattern for optional fields.
- **`{Type}Error` suffix** — all error variants use this naming convention.
- **Manual `From` implementations** — avoids exposing external types in the public API.

## Data Flow

```
Client                          Server
  │                               │
  ├── GET /sse ──────────────────►│ creates Session (UUID, mpsc channel)
  │◄── SSE event: endpoint ──────┤ sends message URI
  │                               │
  ├── POST /message ─────────────►│ parse JSON-RPC
  │   {initialize}                │ → enforce_lifecycle()
  │                               │ → RequestHandler::handle()
  │◄── SSE event: message ───────┤ sends response via mpsc → SSE stream
  │                               │
  ├── POST /message ─────────────►│ {notifications/initialized}
  │   session → Ready             │ → session state = Ready
  │                               │
  ├── POST /message ─────────────►│ {tools/list, tools/call, ...}
  │                               │ → ToolRegistry → McpTool::execute()
  │◄── SSE event: message ───────┤ → CallToolResult → JsonRpcResponse
```

## Dependencies

| Crate | Purpose |
|-------|---------|
| `tokio` | Async runtime (HTTP server, signal handling) |
| `axum` | HTTP framework (SSE endpoints, routing) |
| `tower-http` | HTTP middleware (CORS) |
| `tokio-stream` | Async stream utilities for SSE |
| `uuid` | Session ID generation (v4) |
| `serde` / `serde_json` | JSON serialization/deserialization |
| `async-trait` | Async trait support for `McpTool` |
| `chrono` | Date/time parsing for calendar events |
| `thiserror` | Ergonomic error type derivation |
| `tracing` / `tracing-subscriber` | Structured logging |

### Dev Dependencies

| Crate | Purpose |
|-------|---------|
| `reqwest` | HTTP client for examples (with `stream` feature) |
| `tokio-util` | Stream-to-AsyncRead adapter for SSE parsing |
| `futures-util` | `StreamExt` for byte stream processing |
| `tempfile` | Temporary files for tests |
