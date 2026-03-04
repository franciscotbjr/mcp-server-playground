# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Changed

- **Transport: stdio → SSE** — `McpServer` now runs an Axum HTTP server with `GET /sse` and `POST /message` endpoints, replacing the previous stdin/stdout loop
- **`handler.rs` split** — Extracted `McpTool` trait into `tool_trait.rs` and `ToolRegistry` into `tool_registry.rs`; `handler.rs` now contains only `RequestHandler`
- **`mcp/` reorganized into subdirectories** — `protocol/` (jsonrpc.rs, types.rs), `tools/` (tool_trait.rs, tool_registry.rs), `transport/` (server.rs, sse_handler.rs, session.rs, app_state.rs, message_query.rs); `handler.rs` stays at `mcp/` root as central dispatcher
- **`protocol.rs` renamed to `jsonrpc.rs`** — Avoids clippy `module_inception` lint inside `protocol/` subdirectory
- **Public type tests moved to `tests/`** — Following design-source convention, tests for public types are now in separate integration test files (`tests/error_tests.rs`, `tests/protocol_tests.rs`, `tests/types_tests.rs`, `tests/tool_registry_tests.rs`, `tests/handler_tests.rs`); `pub(crate)` tests remain inline
- **`main.rs`** — Updated to bootstrap Axum HTTP server with `McpServer::new(handler, addr)`

### Added

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
