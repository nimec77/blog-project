# Blog Project

A Rust-based blog platform with HTTP and gRPC APIs, CLI client, and WASM frontend.

## Project Structure

| Crate | Type | Description |
|-------|------|-------------|
| `blog-shared` | lib | Common DTOs (User, Post, AuthResponse) |
| `blog-server` | bin | HTTP + gRPC server |
| `blog-client` | lib | Client library for HTTP/gRPC |
| `blog-cli` | bin | Command-line interface |
| `blog-wasm` | lib (cdylib) | Yew frontend |

## Quick Start

### Prerequisites

- Rust (stable)
- SQLite

### Setup

```bash
# Clone the repository
git clone <repo-url>
cd blog-project

# Copy environment config
cp .env.example .env

# Build all crates
cargo build --workspace

# Run the server
cargo run -p blog-server

# Run the CLI
cargo run -p blog-cli -- --help
```

## Development

```bash
# Build all
cargo build --workspace

# Run tests
cargo test --workspace

# Run server
cargo run -p blog-server

# Run CLI
cargo run -p blog-cli -- <command>
```

## License

MIT
