# Architecture

## Overview

`mcp-server-playground` is a Rust-based MCP (Model Context Protocol) server that exposes two tools — **calendar** and **contacts** — with simulated data backed by local JSON files.

The server communicates over **Streamable HTTP** transport, using the JSON-RPC 2.0 protocol as defined by the MCP specification (2025-03-26).

A single `/mcp` endpoint is exposed supporting three HTTP methods:

- **`POST /mcp`** — Client sends JSON-RPC requests (single or batch). Server responds with JSON directly in the response body. Session ID is passed via `Mcp-Session-Id` header.
- **`GET /mcp`** — Opens a passive SSE stream for server-initiated messages (requires `Mcp-Session-Id` header).
- **`DELETE /mcp`** — Terminates a session (requires `Mcp-Session-Id` header).

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
    └── transport/        # Streamable HTTP transport, HTTP server, server identity
        ├── mod.rs        # Facade: mod + pub use
        ├── server.rs     # McpServer — HTTP bootstrap + graceful shutdown
        ├── streamable_handler.rs # POST/GET/DELETE /mcp handlers + lifecycle enforcement
        ├── session.rs    # SessionState, Session, SessionStore (per-client lifecycle)
        ├── app_state.rs  # AppState (shared state for Axum handlers)
        ├── no_delay_listener.rs # NoDelayListener (TCP_NODELAY wrapper for TcpListener)
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
│   ├── calendar_tool.rs  # CalendarTool (McpTool implementation)
│   ├── queries.rs        # Pure query functions over CalendarData
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
│   ├── contacts_tool.rs  # ContactsTool (McpTool implementation)
│   ├── queries.rs        # Pure query functions over ContactsData
│   └── contacts_metadata.rs # ContactsMetadata

tests/                    # Integration tests for public types
├── error_tests.rs        # Error enum tests (8 tests)
├── protocol_tests.rs     # JSON-RPC type tests (10 tests)
├── types_tests.rs        # MCP domain type tests (11 tests)
├── tool_registry_tests.rs # ToolRegistry tests (5 tests)
├── handler_tests.rs      # RequestHandler dispatch tests (8 tests)
├── calendar_types_tests.rs # Calendar domain type tests (12 tests)
├── calendar_tool_tests.rs # CalendarTool MCP tool tests (14 tests)
├── contacts_tool_tests.rs # ContactsTool MCP tool tests (13 tests)
├── contacts_types_tests.rs # Contacts domain type tests (11 tests)
└── streamable_http_tests.rs # Streamable HTTP transport integration tests (13 tests)

examples/
├── initialize.rs         # Minimal MCP initialize lifecycle demo (Streamable HTTP)
├── calendar_data.rs      # Load and query calendar.json demo
├── calendar_tool.rs      # MCP client exercising all calendar actions via Streamable HTTP
├── contacts_data.rs      # Load and query contacts.json demo
└── contacts_tool.rs      # MCP client exercising all contacts actions via Streamable HTTP
```

## Key Design Decisions

- **Single crate, no feature flags** — project scope is small enough that feature gating adds unnecessary complexity.
- **Streamable HTTP transport** — follows MCP spec 2025-03-26 with a single `POST|GET|DELETE /mcp` endpoint. Supports JSON-RPC batch requests.
- **Session management** — each `initialize` request creates a UUID-identified session (`Mcp-Session-Id` header). Lifecycle state tracking: `Initializing → Ready`.
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
  ├── POST /mcp ─────────────────►│ {initialize}
  │   Accept: application/json    │ → creates Session (UUID)
  │◄── 200 JSON ─────────────────┤ → response + Mcp-Session-Id header
  │                               │
  ├── POST /mcp ─────────────────►│ {notifications/initialized}
  │   Mcp-Session-Id: <uuid>      │ → session state = Ready
  │◄── 202 Accepted ─────────────┤
  │                               │
  ├── POST /mcp ─────────────────►│ {tools/list, tools/call, ...}
  │   Mcp-Session-Id: <uuid>      │ → ToolRegistry → McpTool::execute()
  │◄── 200 JSON ─────────────────┤ → CallToolResult → JsonRpcResponse
  │                               │
  ├── GET /mcp (optional) ───────►│ passive SSE stream for server push
  │◄── SSE events ───────────────┤
  │                               │
  ├── DELETE /mcp ───────────────►│ terminate session
  │◄── 200 OK ───────────────────┤
```

## Dependencies

| Crate | Purpose |
|-------|---------|
| `tokio` | Async runtime (HTTP server, signal handling) |
| `axum` | HTTP framework (Streamable HTTP endpoints, routing) |
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
