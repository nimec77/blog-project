# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

A Rust-based blog platform with multiple interfaces: HTTP API (actix-web), gRPC API (tonic), CLI client, and WASM frontend (Yew). Built as a Cargo workspace with 5 crates following clean architecture principles.

**Core Philosophy**: KISS (Keep It Simple, Stupid) - no abstractions until needed twice, fail fast with `?`, validate only at boundaries.

## Workspace Structure

```
blog-project/
├── blog-shared    # Shared DTOs and types (User, Post, AuthResponse)
├── blog-server    # HTTP + gRPC server (main binary)
├── blog-client    # Client library for HTTP/gRPC communication
├── blog-cli       # Command-line interface (uses blog-client)
└── blog-wasm      # Yew WASM frontend (cdylib)
```

**Dependency Flow**: `blog-shared` ← all other crates (star topology)

## Development Commands

### Building

```bash
# Build entire workspace
cargo build --workspace

# Build specific crate
cargo build -p blog-server
cargo build -p blog-wasm

# Release build
cargo build --workspace --release
```

### Running

```bash
# Run server (requires .env file, see .env.example)
cargo run -p blog-server

# Run CLI
cargo run -p blog-cli -- --help
cargo run -p blog-cli -- list
cargo run -p blog-cli -- register --username alice --email alice@example.com --password secret

# Run WASM frontend (requires trunk: cargo install trunk)
cd blog-wasm
trunk serve --open
```

### Testing

```bash
# Run all tests
cargo test --workspace

# Run tests for specific crate
cargo test -p blog-server

# Run specific test
cargo test -p blog-server test_function_name

# Run with logging
RUST_LOG=debug cargo test --workspace
```

### Database

```bash
# Migrations run automatically on server startup
# Database file: blog.db (SQLite)
# Migration files: blog-server/migrations/*.sql

# To reset database, delete the file
rm blog.db
cargo run -p blog-server  # Will recreate and migrate
```

### Formatting & Linting

```bash
# Format code
cargo fmt --all

# Check formatting
cargo fmt --all -- --check

# Clippy
cargo clippy --workspace
```

## Architecture

### blog-server Layer Structure

```
presentation/     # HTTP handlers (actix-web) + gRPC service (tonic)
    ↓
application/      # Thin services (AuthService, BlogService)
    ↓
data/            # Repositories (UserRepository, PostRepository)
    ↓
domain/          # Entities (User, Post) + errors
    ↓
infrastructure/  # Database, JWT, config
```

**Key Pattern**: Services are thin coordinators. Business logic lives in domain entities. Repositories implement `Clone` (wrap SqlitePool). Services use `Arc` for shared dependencies.

### Authentication Flow

1. User registers/logs in → `POST /api/auth/register` or `POST /api/auth/login`
2. Server hashes password (Argon2), generates JWT (24h expiry)
3. Client stores token in localStorage (WASM) or memory (CLI)
4. Protected endpoints require `Authorization: Bearer <token>` header
5. Middleware validates JWT and extracts user_id

**Authorization**: Post operations (update/delete) check `post.author_id == authenticated_user.id`

### Error Handling

- Each crate has one `AppError` enum (using `thiserror`)
- Use `?` operator for propagation
- Convert external errors via `#[from]` attribute
- Validate at API boundaries only (handlers), trust internal code
- Return `Result` from all functions except `main` (which panics on init failures)

## Code Conventions

### Workspace Dependencies

**CRITICAL**: Shared dependencies MUST be in root `Cargo.toml` under `[workspace.dependencies]`. Crates reference them with `.workspace = true`.

```toml
# Root Cargo.toml
[workspace.dependencies]
tokio = { version = "1", features = ["full"] }

# blog-server/Cargo.toml
[dependencies]
tokio.workspace = true  # ✅
```

Only add dependencies directly to crate `Cargo.toml` if used by that crate alone.

### Module System

Use **new style** (Rust 2018+):
- `src/domain.rs` declares submodules
- `src/domain/user.rs` contains implementation
- **Never** use `mod.rs` files

### Constants

All constants go in `constants.rs` files:
- Shared: `blog-shared/src/constants.rs`
- Crate-specific: `<crate>/src/constants.rs`

Never define constants inline in other modules.

### Import Order

```rust
// 1. Standard library
use std::sync::Arc;

// 2. External crates
use actix_web::{web, HttpResponse};
use sqlx::SqlitePool;

// 3. Internal modules
use crate::domain::User;
```

### Naming

- Structs/Enums: `PascalCase` (UserRepository, BlogService)
- Functions: `snake_case` (find_by_id, create_post)
- Constants: `SCREAMING_SNAKE_CASE` (MAX_TITLE_LEN, DEFAULT_LIMIT)

### Error Handling Rules

- **Never** use `unwrap()` or `expect()` in runtime code
- OK to use `expect()` in `main()` for startup initialization
- Use `?` operator for error propagation
- Use `#[instrument(skip(self, password))]` from `tracing` for auto-instrumentation

### Logging

- Use `tracing` macros: `info!`, `debug!`, `warn!`, `error!`
- Never use `println!` for logging
- Use structured fields: `tracing::info!(user_id = %id, "User created")`
- **Never** log sensitive data (passwords, tokens, full email addresses)
- Server logs JSON format, CLI logs plain text

### Validation

Validate **only at boundaries** (HTTP/gRPC handlers). Service and repository layers trust their inputs.

```rust
// ✅ Good: validate in handler
async fn create_post(req: Json<CreatePostRequest>) -> Result<...> {
    if req.title.is_empty() {
        return Err(AppError::ValidationError("Title required"));
    }
    service.create_post(req.into_inner()).await
}

// ❌ Bad: re-validating in service
impl BlogService {
    async fn create_post(&self, req: CreatePostRequest) -> Result<Post> {
        if req.title.is_empty() { ... } // Don't do this
    }
}
```

### File Size Limits

- < 200 lines: ✅ Good
- 200-400 lines: ⚠️ Consider splitting
- \> 400 lines: 🔴 Must split into smaller modules

## gRPC Development

Protocol buffers defined in:
- `blog-server/proto/blog.proto`
- `blog-client/proto/blog.proto` (same file, copied)

Generated code via `build.rs` using `tonic-prost-build`. Auto-generated on `cargo build`.

**gRPC Reflection**: Enabled on server for tools like `grpcurl`:

```bash
# List services
grpcurl -plaintext localhost:50051 list

# Call method
grpcurl -plaintext -d '{"username":"alice","password":"secret"}' \
  localhost:50051 blog.AuthService/Login
```

## WASM Frontend

Built with Yew 0.21 framework. Uses `trunk` for building and serving.

**API Communication**: Uses `gloo-net` for HTTP requests to `http://localhost:8080/api` (configurable via `API_URL` constant).

**State Management**: Uses Yew hooks (`use_state`) and context API for auth state.

**CORS**: Server configured to allow requests from `http://localhost:8080` and `http://127.0.0.1:8080`.

## Environment Variables

Required in `.env` file (copy from `.env.example`):

```env
DATABASE_URL=sqlite:blog.db
JWT_SECRET=your-super-secret-key-at-least-32-chars
HTTP_PORT=8080              # Optional, defaults to 8080
GRPC_PORT=50051             # Optional, defaults to 50051
RUST_LOG=blog_server=debug,info  # Optional, defaults to info
```

## Common Patterns

### Repository Pattern

```rust
#[derive(Clone)]
pub struct UserRepository {
    pool: SqlitePool,  // SqlitePool already implements Clone
}

impl UserRepository {
    pub fn new(pool: SqlitePool) -> Self { Self { pool } }

    pub async fn find_by_id(&self, id: i64) -> Result<Option<User>, AppError> {
        // Use sqlx compile-time checked queries
        sqlx::query_as!(User, "SELECT * FROM users WHERE id = ?", id)
            .fetch_optional(&self.pool)
            .await
            .map_err(AppError::Database)
    }
}
```

### Service Pattern

```rust
pub struct AuthService {
    user_repo: Arc<UserRepository>,
    jwt_secret: String,
}

impl AuthService {
    pub fn new(user_repo: Arc<UserRepository>, jwt_secret: String) -> Self {
        Self { user_repo, jwt_secret }
    }
}
```

Services are `Clone` (via `#[derive(Clone)]`) to be shared across actix-web workers.

### HTTP Handler Pattern

```rust
use actix_web::{post, web, HttpResponse, Responder};

#[post("/auth/register")]
async fn register(
    service: web::Data<AuthService>,
    req: web::Json<RegisterRequest>,
) -> Result<impl Responder, AppError> {
    let response = service.register(req.into_inner()).await?;
    Ok(HttpResponse::Created().json(response))
}

// In routes module
pub fn api_routes() -> Scope {
    web::scope("")
        .service(register)
        .service(login)
}
```

Handlers return `Result<impl Responder, AppError>`. actix-web automatically converts `AppError` to HTTP response via `ResponseError` trait implementation.

### Configuration Pattern

```rust
// infrastructure/config.rs returns Result
impl Config {
    pub fn from_env() -> Result<Self, AppError> {
        dotenvy::dotenv().ok();
        let database_url = std::env::var("DATABASE_URL")
            .map_err(|_| AppError::Config("DATABASE_URL must be set"))?;
        Ok(Self { database_url, ... })
    }
}

// main.rs panics on invalid config
let config = Config::from_env().expect("invalid configuration");
```

## Database Schema

### users table

- `id`: INTEGER PRIMARY KEY AUTOINCREMENT
- `username`: TEXT NOT NULL UNIQUE
- `email`: TEXT NOT NULL UNIQUE
- `password_hash`: TEXT NOT NULL (Argon2)
- `created_at`: TEXT NOT NULL (ISO 8601)

### posts table

- `id`: INTEGER PRIMARY KEY AUTOINCREMENT
- `title`: TEXT NOT NULL
- `content`: TEXT NOT NULL
- `author_id`: INTEGER NOT NULL FK → users(id) ON DELETE CASCADE
- `created_at`: TEXT NOT NULL (ISO 8601)
- `updated_at`: TEXT NOT NULL (ISO 8601)

## API Endpoints

### HTTP (actix-web)

**Public**:
- `POST /api/auth/register` - Register new user
- `POST /api/auth/login` - Login, get JWT
- `GET /api/posts?limit=10&offset=0` - List posts (paginated)
- `GET /api/posts/{id}` - Get single post

**Protected** (requires `Authorization: Bearer <token>`):
- `POST /api/posts` - Create post
- `PUT /api/posts/{id}` - Update post (author only)
- `DELETE /api/posts/{id}` - Delete post (author only)
- `GET /api/auth/me` - Get current user info

### gRPC (tonic)

Services: `AuthService`, `BlogService`

See `blog-server/proto/blog.proto` for full RPC definitions.

## Testing Strategy

- **Unit tests**: In `#[cfg(test)]` modules within source files
- **Integration tests**: In `tests/` directory (if present)
- Use `#[tokio::test]` for async tests
- Test database: Use in-memory SQLite (`:memory:`) or separate test DB file

## Troubleshooting

### "DATABASE_URL not set"
Copy `.env.example` to `.env` and configure values.

### Build errors after changing proto files
Delete `target/` and rebuild: `cargo clean && cargo build`

### WASM frontend can't connect to API
Check CORS configuration in `blog-server/src/constants.rs` includes your origin.

### JWT "invalid token" errors
Check `JWT_SECRET` matches between server and client, and token hasn't expired (24h).

## Project-Specific Workflow

This project has a structured workflow documented in `.cursor/rules/workflow.mdc`:

1. **PROPOSE**: Present implementation plan with file changes and code snippets
2. **AGREE**: Wait for user approval
3. **IMPLEMENT**: Write code, run tests, verify compilation
4. **CONFIRM**: Show results and wait for confirmation
5. **UPDATE**: Update `doc/tasklist.md` (do NOT auto-commit)

**Important**: Never auto-commit to git. User reviews and commits manually.

## Additional Documentation

- `README.md` - Quick start guide
- `vision.md` - Complete technical vision and architecture decisions
- `conventions.md` - Detailed code conventions and patterns
- `.cursor/rules/` - Cursor-specific workflow rules
- `doc/` - Project documentation and task lists
