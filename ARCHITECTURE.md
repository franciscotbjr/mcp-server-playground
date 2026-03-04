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
    ├── protocol/         # JSON-RPC and MCP domain types
    │   ├── mod.rs        # Facade: mod + pub use
    │   ├── jsonrpc.rs    # JSON-RPC 2.0 types (request, response, error, notification)
    │   └── types.rs      # MCP domain types (ToolDefinition, CallToolResult, etc.)
    ├── tools/            # Tool abstraction layer
    │   ├── mod.rs        # Facade: mod + pub use
    │   ├── tool_trait.rs # McpTool trait (interface for MCP tools)
    │   └── tool_registry.rs # ToolRegistry (collection of registered tools)
    └── transport/        # SSE transport + HTTP server
        ├── mod.rs        # Facade: mod + pub use
        ├── server.rs     # McpServer — HTTP bootstrap + graceful shutdown
        ├── sse_handler.rs # SSE endpoint handlers + lifecycle enforcement
        ├── session.rs    # SessionState, Session, SessionStore (per-client lifecycle)
        ├── app_state.rs  # AppState (shared state for Axum handlers)
        └── message_query.rs # MessageQuery (POST /message query params)

tests/                    # Integration tests for public types
├── error_tests.rs        # Error enum tests
├── protocol_tests.rs     # JSON-RPC type tests
├── types_tests.rs        # MCP domain type tests
├── tool_registry_tests.rs # ToolRegistry tests
└── handler_tests.rs      # RequestHandler dispatch tests

examples/
└── initialize.rs         # Minimal MCP initialize lifecycle demo

calendar/                 # Calendar domain (Phase 2+)
│   ├── mod.rs
│   ├── calendar_data.rs, event.rs, location.rs, attendee.rs
│   ├── recurrence.rs, reminder.rs, calendar_settings.rs
│   ├── calendar_tool.rs
│   └── queries.rs

contacts/                 # Contacts domain (Phase 2+)
    ├── mod.rs
    ├── contacts_data.rs, contact.rs, phone_number.rs
    ├── email.rs, address.rs, social_profile.rs
    ├── contacts_tool.rs
    └── queries.rs
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
