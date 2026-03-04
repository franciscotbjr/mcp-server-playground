# Implementation Plan: Phase 1 — Foundation

## Overview

Establish the project foundation: dependencies, error handling, MCP protocol types, SSE transport, session management, initialize lifecycle, server bootstrap, tests, example, and documentation.

## Status: COMPLETE

## Steps Completed

### v0.1.0-alpha (initial — stdio transport)

1. **Cargo.toml** — Initial dependencies, package metadata
2. **error.rs** — `Error` enum with `{Type}Error` suffix, `Result<T>` alias, manual `From` impls
3. **mcp/protocol.rs** — JSON-RPC 2.0 types: `JsonRpcRequest`, `JsonRpcResponse`, `JsonRpcError`, `JsonRpcNotification`
4. **mcp/types.rs** — MCP types: `ToolDefinition`, `InputSchema`, `CallToolResult`, `Content`, `InitializeResult`, `ServerCapabilities`, `ServerInfo`, `CallToolParams`, `ListToolsResult`
5. **mcp/handler.rs** — `McpTool` trait, `ToolRegistry`, `RequestHandler`
6. **mcp/server.rs** — `McpServer` with stdio read/write loop
7. **Documentation** — README, ARCHITECTURE, CHANGELOG, DECISIONS, spec/definition, spec/api-analysis, spec/apis/*.yaml

### v0.1.0 (current — SSE transport + refactor)

8. **Cargo.toml** — Added `axum`, `tokio-stream`, `uuid`, `tower-http`; replaced tokio `io-util`/`io-std` with `signal`+`io-util`; added dev-deps `reqwest` (stream), `tokio-util`, `futures-util`
9. **mcp/server.rs** — Rewritten: SSE transport via Axum HTTP server, graceful shutdown
10. **mcp/session.rs** — New: `SessionState` enum, `Session` struct, `SessionStore` type alias
11. **mcp/sse_handler.rs** — New: `handle_sse`, `handle_message`, `enforce_lifecycle`, `send_to_session`
11b. **mcp/app_state.rs** — New: `AppState` shared state struct (extracted from `sse_handler.rs`)
11c. **mcp/message_query.rs** — New: `MessageQuery` query params struct (extracted from `sse_handler.rs`)
12. **mcp/tool_trait.rs** — New: `McpTool` trait (extracted from handler.rs)
13. **mcp/tool_registry.rs** — New: `ToolRegistry` struct (extracted from handler.rs)
14. **mcp/handler.rs** — Reduced: only `RequestHandler` (dispatch logic)
15. **mcp/mod.rs** — Updated facade: 10 submodules, public re-exports
16. **lib.rs** — Updated re-exports including `SessionState`, `JsonRpcNotification`
17. **main.rs** — Updated: `McpServer::new(handler, addr)` bootstrap
18. **tests/** — Public type tests moved to integration test files:
    - `tests/error_tests.rs` (4 tests)
    - `tests/protocol_tests.rs` (4 tests)
    - `tests/types_tests.rs` (4 tests)
    - `tests/tool_registry_tests.rs` (2 tests)
    - `tests/handler_tests.rs` (8 tests)
19. **examples/initialize.rs** — Minimal example demonstrating SSE connect → initialize → initialized lifecycle
20. **Documentation** — All docs updated to reflect SSE architecture

## Module Structure (current)

```
src/
├── main.rs               # Entry point: Axum HTTP server bootstrap
├── lib.rs                # Crate root: module declarations + re-exports
├── error.rs              # Centralized Error enum
└── mcp/
    ├── mod.rs            # Facade: mod + pub use
    ├── protocol.rs       # JSON-RPC 2.0 types
    ├── types.rs          # MCP domain types
    ├── tool_trait.rs     # McpTool trait
    ├── tool_registry.rs  # ToolRegistry
    ├── handler.rs        # RequestHandler (dispatch)
    ├── session.rs        # SessionState, Session, SessionStore
    ├── app_state.rs      # AppState (shared state for Axum handlers)
    ├── message_query.rs  # MessageQuery (POST /message query params)
    ├── sse_handler.rs    # SSE endpoint handlers + lifecycle enforcement
    └── server.rs         # McpServer (HTTP bootstrap + graceful shutdown)

tests/
├── error_tests.rs
├── protocol_tests.rs
├── types_tests.rs
├── tool_registry_tests.rs
└── handler_tests.rs

examples/
└── initialize.rs
```

## Quality Checks

- [x] `cargo build` — compiles without errors
- [x] `cargo test` — 36 tests pass (14 inline + 22 integration)
- [x] `cargo clippy -- -D warnings` — clean
- [x] `cargo run --example initialize` — completes successfully
- [x] All types are `Send + Sync`
