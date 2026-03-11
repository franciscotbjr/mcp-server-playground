# mcp-server-playground

A Rust-based MCP (Model Context Protocol) server that exposes two tools for AI agents: **calendar** and **contacts**, with simulated data backed by local JSON files.

## Overview

This project implements an MCP server over **Streamable HTTP** transport, following the [MCP specification (2025-03-26)](https://modelcontextprotocol.io/). It serves as a learning/playground project for MCP server development in Rust.

The server exposes a single `/mcp` endpoint supporting three HTTP methods:

- **`POST /mcp`** — Client sends JSON-RPC requests (single or batch); server responds with JSON. Session ID via `Mcp-Session-Id` header.
- **`GET /mcp`** — Opens a passive SSE stream for server-initiated messages (requires `Mcp-Session-Id` header).
- **`DELETE /mcp`** — Terminates a session (requires `Mcp-Session-Id` header).

### Tools

- **`calendar`** — Query and search calendar events (list, get by ID, search, filter by category/date/priority)
- **`contacts`** — Query and search contacts (list, get by ID, search, filter by tag, favorites)

### Data

All data is simulated via local JSON files:

- `calendar.json` — 12 events with attendees, locations, recurrence, reminders
- `contacts.json` — 17 contacts with phone numbers, emails, addresses, social profiles

## Building

```bash
cargo build
```

## Running

The server starts an HTTP server on `127.0.0.1:3000` by default.

```bash
cargo run
```

To enable debug logging:

```bash
RUST_LOG=debug cargo run
```

## Quick Start with curl

Send an `initialize` request (requires the server to be running):

```bash
curl -X POST http://127.0.0.1:3000/mcp \
  -H "Content-Type: application/json" \
  -H "Accept: application/json, text/event-stream" \
  -d '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2025-03-26","capabilities":{},"clientInfo":{"name":"curl","version":"1.0"}}}'
```

Expected output — JSON-RPC response with server capabilities and a `Mcp-Session-Id` response header:

```json
{"jsonrpc":"2.0","id":1,"result":{"protocolVersion":"2025-03-26","capabilities":{"tools":{"listChanged":false}},"serverInfo":{"name":"mcp-server-playground","version":"0.1.0"}}}
```

## Examples

Run the `initialize` example to see the full MCP lifecycle handshake (`POST /mcp` → initialize → initialized):

```bash
cargo run --example initialize
```

## Testing

```bash
cargo test
```

119 tests total: 14 inline unit tests (`pub(crate)` internals) + 105 integration tests in `tests/`.

## Project Structure

See [ARCHITECTURE.md](ARCHITECTURE.md) for the full module structure and design decisions.

## Methodology

This project follows the [design-source](https://github.com/franciscotbjr/design-source) methodology for structured Rust development with AI assistance.

## License

MIT
