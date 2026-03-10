# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Fixed

- **SSE latency** — Added `NoDelayListener` wrapper that sets `TCP_NODELAY` on each accepted TCP connection, eliminating Nagle's algorithm buffering delay on small SSE events ([axum#2521](https://github.com/tokio-rs/axum/issues/2521))
- **Graceful shutdown** — Single Ctrl-C now cleanly shuts down the server by clearing all sessions (dropping mpsc senders), ending SSE streams, and completing Axum's graceful shutdown; second Ctrl-C forces immediate exit

### Added

- **Tracing logs** — `info!` logs for server initialization steps (`main.rs`) and JSON-RPC request handling (`handler.rs`)
- **SSE disconnect detection** — Client disconnects are detected via `mpsc::Sender::closed()` and logged; sessions are automatically removed from the `SessionStore`
- **Shutdown broadcast** — `watch` channel in `AppState` signals SSE cleanup tasks to drop their sender clones on shutdown, allowing `ReceiverStream` to end naturally

### Changed

- **Transport: stdio → SSE** — `McpServer` now runs an Axum HTTP server with `GET /sse` and `POST /message` endpoints, replacing the previous stdin/stdout loop
- **`handler.rs` split** — Extracted `McpTool` trait into `tool_trait.rs` and `ToolRegistry` into `tool_registry.rs`; `handler.rs` now contains only `RequestHandler`
- **`mcp/` reorganized into subdirectories** — `protocol/`, `tools/`, `transport/`; `handler.rs` stays at `mcp/` root as central dispatcher
- **`protocol.rs` → `jsonrpc.rs` → one-type-per-file** — Split into `request.rs`, `response.rs`, `error.rs`, `notification.rs`
- **`types.rs` split into one-type-per-file** — 10 individual files; 6 tool-related types moved to `tools/` (ToolDefinition, InputSchema, CallToolResult, CallToolParams, ListToolsResult, Content); 4 server identity types moved to `transport/` (InitializeResult, ServerCapabilities, ServerInfo, ToolsCapability)
- **Public type tests moved to `tests/`** — Following design-source convention, tests for public types are now in separate integration test files (`tests/error_tests.rs`, `tests/protocol_tests.rs`, `tests/types_tests.rs`, `tests/tool_registry_tests.rs`, `tests/handler_tests.rs`); `pub(crate)` tests remain inline
- **`main.rs`** — Updated to bootstrap Axum HTTP server with `McpServer::new(handler, addr)`

### Added (Phase 3 — Calendar Tool)

- `src/calendar/calendar_tool.rs` — `CalendarTool` struct implementing `McpTool` trait; loads `calendar.json` at construction and dispatches 6 query actions
- `src/calendar/queries.rs` — Pure query functions: `list_events`, `get_event`, `search_events`, `events_by_date`, `events_by_category`, `upcoming_events`
- `ToolRegistry::len()` and `is_empty()` methods
- `CalendarTool` registered in `main.rs` — server now exposes 1 tool via `tools/list`
- `tests/calendar_tool_tests.rs` — 14 integration tests for all calendar actions
- `examples/calendar_tool.rs` — MCP client example exercising all calendar actions over SSE (requires running server)

### Added (Phase 2 — Domain Types)

- `src/calendar/` module — 10 type files + facade for calendar domain types
- Calendar types: `CalendarRoot`, `CalendarData`, `Event`, `Location`, `Coordinates`, `Attendee`, `Recurrence`, `Reminder`, `Attachment`, `Cost`, `CalendarSettings`, `WorkingHours`, `CategoryConfig`, `CalendarMetadata`
- `src/contacts/` module — 7 type files + facade for contacts domain types
- Contacts types: `ContactsData`, `Contact`, `PhoneNumber`, `ContactEmail`, `Address`, `SocialProfile`, `ContactsMetadata`
- `tests/calendar_types_tests.rs` — 12 integration tests (full JSON deserialization, individual type verification, Send+Sync)
- `tests/contacts_types_tests.rs` — 11 integration tests (full JSON deserialization, individual type verification, Send+Sync)
- `examples/calendar_data.rs` — Demonstrates loading and querying `calendar.json`
- `examples/contacts_data.rs` — Demonstrates loading and querying `contacts.json`
- Updated `spec/api-analysis.md` — Fixed both calendar and contacts data shapes to match actual JSON

### Added (Phase 1 — Foundation)

- `session.rs` — `SessionState` enum (`Uninitialized`, `Initializing`, `Ready`), `Session` struct, `SessionStore` type alias for per-client lifecycle tracking
- `sse_handler.rs` — SSE endpoint handlers (`handle_sse`, `handle_message`), lifecycle enforcement (`enforce_lifecycle`), helper functions (`send_to_session`)
- `app_state.rs` — `AppState` shared state struct (extracted from `sse_handler.rs`)
- `message_query.rs` — `MessageQuery` query params struct (extracted from `sse_handler.rs`)
- `tool_trait.rs` — `McpTool` async trait (extracted from `handler.rs`)
- `tool_registry.rs` — `ToolRegistry` struct with `register()`, `list_definitions()`, `call_tool()` (extracted from `handler.rs`)
- MCP `initialize` lifecycle tests — lifecycle enforcement tests in `sse_handler.rs`, expanded initialize response tests in `handler_tests.rs`
- `examples/initialize.rs` — Minimal example demonstrating the full MCP initialize handshake over SSE (connect → initialize → initialized)
- Dependencies: `axum`, `tower-http` (CORS), `tokio-stream`, `uuid` (v4); dev-deps: `reqwest` (stream), `tokio-util`, `futures-util`
- `JsonRpcNotification` re-export from crate root

### Removed

- Stdio transport (stdin/stdout read/write loop)
- Tokio `io-util` and `io-std` features (replaced by `signal` + `io-util`)

## [0.1.0] - 2025-08-28

### Added

- Project foundation: `Cargo.toml` with all dependencies
- Centralized error handling (`error.rs`) with `{Type}Error` suffix convention
- JSON-RPC 2.0 protocol types (`protocol.rs`)
- MCP-specific types: `ToolDefinition`, `CallToolResult`, `Content`, etc. (`types.rs`)
- `McpTool` trait for implementing MCP tools
- `ToolRegistry` for tool registration and dispatch
- `RequestHandler` for JSON-RPC method routing (`initialize`, `tools/list`, `tools/call`)
- `McpServer` with stdio transport (stdin/stdout)
- Unit tests for all modules (18 tests)
- Documentation: README, ARCHITECTURE, CHANGELOG, DECISIONS
