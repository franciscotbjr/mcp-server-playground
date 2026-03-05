# mcp-server-playground

A Rust-based MCP (Model Context Protocol) server that exposes two tools for AI agents: **calendar** and **contacts**, with simulated data backed by local JSON files.

## Overview

This project implements an MCP server over **SSE (Server-Sent Events)** transport, following the [MCP specification (2024-11-05)](https://modelcontextprotocol.io/). It serves as a learning/playground project for MCP server development in Rust.

The server exposes two HTTP endpoints:

- **`GET /sse`** — Opens an SSE stream; the server sends an `endpoint` event with the message URI
- **`POST /message?sessionId=<uuid>`** — Receives JSON-RPC requests from the client

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

Connect to the SSE endpoint (requires the server to be running):

```bash
curl -N http://127.0.0.1:3000/sse
```

Expected output — the server assigns a session and sends the message endpoint URI:

```
event: endpoint
data: /message?sessionId=<uuid>
```

## Examples

Run the `initialize` example to see the full MCP lifecycle handshake (SSE connect → initialize → initialized):

```bash
cargo run --example initialize
```

## Testing

```bash
cargo test
```

79 tests total: 14 inline unit tests (`pub(crate)` internals) + 65 integration tests in `tests/`.

## Project Structure

See [ARCHITECTURE.md](ARCHITECTURE.md) for the full module structure and design decisions.

## Methodology

This project follows the [design-source](https://github.com/franciscotbjr/design-source) methodology for structured Rust development with AI assistance.

## License

MIT
