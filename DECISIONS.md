# Decision Log

Architectural decisions and trade-offs for the `mcp-server-playground` project.

---

**[2025-08-28] Manual MCP protocol implementation (no SDK crate)**
- Context: Existing Rust MCP SDK crates (`mcp-sdk`, `rmcp`) could be used instead of manual implementation.
- Decision: Implement the MCP protocol layer manually.
- Consequences: More educational, full control over behavior, no external MCP dependency to track. More initial code to write.

**[2025-08-28] Stdio transport only** *(superseded — see SSE transport below)*
- Context: MCP supports both stdio and SSE/HTTP transports.
- Decision: Support stdio only for v1.0.
- Consequences: Simpler implementation, compatible with all MCP clients. SSE can be added later if needed.

**[2026-03-04] SSE transport replaces stdio**
- Context: The MCP specification (2024-11-05) defines SSE as a transport option with `GET /sse` and `POST /message` endpoints. SSE enables HTTP-based clients and better lifecycle management.
- Decision: Replace stdio transport with SSE via Axum HTTP server. Each SSE connection creates a session (UUID v4) with lifecycle state tracking (`Uninitialized → Initializing → Ready`).
- Consequences: Requires additional dependencies (`axum`, `tower-http`, `tokio-stream`, `uuid`). Enables full `initialize` lifecycle enforcement. HTTP-based — works with standard HTTP clients and MCP inspectors.

**[2025-08-28] Single tool with action parameter (not multiple sub-tools)**
- Context: Calendar/contacts could be exposed as one tool each (with an `action` argument) or as many fine-grained tools (e.g., `calendar_list_events`, `calendar_get_event`).
- Decision: One tool per domain (`calendar`, `contacts`) with an `action` string argument to select the operation.
- Consequences: Cleaner tool list for LLM consumers, single schema per domain, slightly more complex dispatch inside each tool.

**[2025-08-28] JSON data loaded at startup**
- Context: Simulated data from `calendar.json` and `contacts.json` could be loaded at startup or lazily on first access.
- Decision: Load at startup into memory.
- Consequences: Simple, fast access, no lazy-init complexity. Data files are small (~50KB total).

**[2025-08-28] No feature flags**
- Context: The design-source methodology recommends feature flags for modularity.
- Decision: Skip feature flags — the project scope is small (two tools) and all modules are always needed.
- Consequences: Simpler `Cargo.toml` and no conditional compilation. All code is always compiled.

**[2025-08-28] `{Type}Error` suffix + manual `From` implementations**
- Context: Error variant naming and how to handle external error types.
- Decision: Follow design-source convention: `IoError`, `JsonError`, etc. Manual `From` impls to convert external errors to `String`.
- Consequences: No external types leak into the public API. Clear, consistent error naming.

**[2025-08-28] With-method chain pattern over builder**
- Context: Domain types have many optional fields.
- Decision: Use `with_*` methods on the type itself (no separate builder struct).
- Consequences: Simpler API, no validation step needed, ergonomic method chaining.

**[2025-08-28] Tracing to stderr** *(note: still applies — tracing writes to stderr even with SSE transport)*
- Context: MCP uses stdout for protocol messages, so logs cannot go there.
- Decision: Configure `tracing-subscriber` to write to stderr.
- Consequences: Logs don't interfere with MCP protocol. Visible in terminal when running the server.

**[2026-03-04] Split handler.rs into one-type-per-file**
- Context: `handler.rs` contained three distinct public types: `McpTool` trait, `ToolRegistry`, and `RequestHandler`. The design-source methodology prescribes one type per file.
- Decision: Extract `McpTool` into `tool_trait.rs`, `ToolRegistry` into `tool_registry.rs`, and keep only `RequestHandler` in `handler.rs`.
- Consequences: Better separation of concerns, each file has a single responsibility. Same applies to `server.rs` which was split into `session.rs`, `sse_handler.rs`, and a reduced `server.rs`.

**[2026-03-04] Reorganize mcp/ into subdirectories**
- Context: The flat `mcp/` directory had 10 files spanning three distinct domains: protocol types, tool abstraction, and SSE transport. As the project grows, a flat structure becomes harder to navigate.
- Decision: Group files into `protocol/`, `tools/`, and `transport/`. `handler.rs` stays at `mcp/` root as it bridges protocol and tools. Each subdirectory has its own `mod.rs` facade. `protocol.rs` split into one-type-per-file: `request.rs`, `response.rs`, `error.rs`, `notification.rs`.
- Consequences: Clear domain boundaries. Public API unchanged — `mcp/mod.rs` re-exports everything through subdirectory facades.

**[2026-03-04] Split types.rs and distribute types by domain**
- Context: `types.rs` contained 10 MCP domain types mixing server-level and tool-level concerns. The one-type-per-file principle applies.
- Decision: Split into 10 individual files. Move 6 tool-related types (`ToolDefinition`, `InputSchema`, `CallToolResult`, `CallToolParams`, `ListToolsResult`, `Content`) to `tools/`. Move 4 server identity types (`InitializeResult`, `ServerCapabilities`, `ServerInfo`, `ToolsCapability`) to `transport/` — they describe the server's identity and capabilities, which is a transport/server concern.
- Consequences: Tool types live alongside their trait and registry. Server types live alongside `McpServer`. `handler.rs` imports from all three domains. `protocol/` contains only `jsonrpc.rs`. Public API unchanged.

**[2026-03-04] Public type tests in `tests/`, `pub(crate)` tests inline**
- Context: The design-source methodology defines that unit tests of public types should be in separate files, not in the component source file.
- Decision: Move all public type tests to `tests/{component}_tests.rs`. Keep `pub(crate)` tests (session internals, handler internals) as inline `#[cfg(test)] mod tests` since integration tests cannot access `pub(crate)` items.
- Consequences: Clean source files, clear separation between implementation and testing. 105 integration tests in `tests/`, 14 inline tests for `pub(crate)` internals.

**[2026-03-10] Streamable HTTP transport replaces SSE**
- Context: The MCP specification 2025-03-26 introduces Streamable HTTP as the recommended transport, deprecating the SSE transport (`GET /sse` + `POST /message`). Streamable HTTP uses a single `/mcp` endpoint with `POST`, `GET`, and `DELETE` methods, session management via `Mcp-Session-Id` header, and supports JSON-RPC batch requests.
- Decision: Replace SSE transport with Streamable HTTP. Remove `sse_handler.rs` and `message_query.rs`. Create `streamable_handler.rs` with `handle_post_mcp`, `handle_get_mcp`, `handle_delete_mcp`. Update protocol version to `2025-03-26`.
- Consequences: Responses returned directly in HTTP response body (no longer via SSE channel). Simpler client integration — standard HTTP POST with JSON. Passive SSE stream (`GET /mcp`) still available for server-initiated push. Session termination via `DELETE /mcp`. Batch JSON-RPC support. Compatible with n8n and other MCP clients.
