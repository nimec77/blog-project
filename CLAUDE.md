# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

A Rust-based blog platform with multiple interfaces: HTTP API (actix-web), gRPC API (tonic), CLI client, and WASM frontend (Yew). Built as a Cargo workspace with 5 crates following clean architecture principles.

**Core Philosophy**: KISS (Keep It Simple, Stupid) - no abstractions until needed twice, fail fast with `?`, validate only at boundaries.

### Development Principles

| Principle | Description |
|-----------|-------------|
| **KISS First** | Simplest solution that works. No abstractions until needed twice. |
| **Fail Fast** | Return errors early with `?`. Validate at API boundaries only. |
| **Single Responsibility** | One file = one purpose. Functions do one thing. |
| **No Premature Optimization** | Make it work first. Optimize only with evidence. |

**Examples of KISS in Practice**:
- No token refresh mechanism - just re-login after 24h
- Simple pagination - just `posts + total`, no cursor-based complexity
- SQLite instead of PostgreSQL - no external DB server needed
- Flat structures - minimal nesting in code organization

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

### Layer Responsibilities

| Layer | Responsibility | Depends On |
|-------|----------------|------------|
| **Domain** | Entities, validation, domain errors | Nothing |
| **Data** | Repository traits + SQLite implementations | Domain |
| **Application** | Thin services, orchestrate repositories | Domain, Data |
| **Infrastructure** | DB connection, JWT, config | External crates |
| **Presentation** | HTTP/gRPC handlers, request mapping | Application |

**Dependency Rule**: Layers can only depend on layers below them. Domain is at the core and has no dependencies.

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

### Test Levels

| Level | Scope | Tools |
|-------|-------|-------|
| **Unit Tests** | Domain logic, services | `#[cfg(test)]` modules |
| **Integration Tests** | API endpoints, DB operations | `tests/` folder, test database |
| **E2E Tests** | Full workflows via CLI | CLI commands against test server |

### Testing Rules

- **Each public function has at least one test**
- **Minimum coverage**: Happy path + one error case
- Use `#[tokio::test]` for async tests
- Test database: Use in-memory SQLite (`:memory:`) or separate test DB file
- SQLite in-memory database for test isolation

### Test Naming Convention

Pattern: `test_<function>_<scenario>`

```rust
#[test]
fn test_post_new_creates_with_current_timestamp() { }

#[test]
fn test_post_new_fails_with_empty_title() { }

#[tokio::test]
async fn test_create_post_returns_post_with_id() { }
```

### Test File Location

```rust
// Unit tests: same file
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_post_new() {
        let post = Post::new(1, "Title".into(), "Content".into());
        assert_eq!(post.title, "Title");
    }
}

// Integration tests: tests/ folder
// tests/api_tests.rs
```

## What NOT To Do

| Don't | Do Instead |
|-------|------------|
| `unwrap()`/`expect()` in runtime code | `?` or explicit error handling |
| `println!` for logging | `tracing::info!`, `tracing::error!` |
| Nested `if-else` chains | Early returns with `?` |
| Magic numbers | Named constants in `constants.rs` |
| Comments explaining *what* | Self-documenting code |
| Over-abstracting | Abstractions only when needed twice |
| `clone()` everywhere | Borrow when possible |
| Panics in libraries | Return `Result` |
| Panic in config/DB functions | Return `Result`, panic only in `main()` |
| `mod.rs` files | New style modules (Rust 2018+) |

## Commit Message Format

```
<type>: <short description>

Types: feat, fix, refactor, test, docs, chore
```

**Examples**:
- `feat: add user registration endpoint`
- `fix: handle empty post title validation`
- `refactor: extract JWT logic to separate module`
- `test: add integration tests for post CRUD`
- `docs: update API endpoint documentation`

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

This project follows a structured workflow documented in [doc/workflow.md](doc/workflow.md):

### Workflow Cycle

```
┌─────────────────────────────────────────────────────────────┐
│                                                             │
│   1. PROPOSE  →  2. AGREE  →  3. IMPLEMENT  →  4. CONFIRM   │
│        ▲                                            │       │
│        │                                            ▼       │
│        └──────────────  5. UPDATE  ◄────────────────┘       │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

### Step-by-Step Rules

#### 1. PROPOSE

Before coding, present:

```markdown
## Phase X.Y: <Task Name>

**Goal:** <What we're building>

**Files to create/modify:**
- `path/to/file.rs` — description

**Key code snippets:**
```rust
// Show the main approach
```

**Test:** <How to verify it works>
```

⏳ **Wait for approval before implementing.**

#### 2. AGREE

User responds:
- ✅ **"Approved"** → Proceed to implement
- 🔄 **"Change X"** → Revise proposal
- ❌ **"Different approach"** → Start over

#### 3. IMPLEMENT

After approval:
- Write code following [conventions.md](conventions.md)
- Create/modify only agreed files
- Run `cargo fmt` to format code
- Run `cargo build` to verify compilation
- Run relevant tests

#### 4. CONFIRM

Present results:

```markdown
## ✅ Phase X.Y Complete

**Created:**
- `file1.rs` — description
- `file2.rs` — description

**Test result:**
```bash
$ <test command>
<output>
```

**Ready for next phase?**
```

⏳ **Wait for user confirmation.**

#### 5. UPDATE

After confirmation:

1. Update [doc/tasklist.md](doc/tasklist.md):
   - Mark completed tasks: `- [ ]` → `- [x]`
   - Update progress report table
   - Change status: `⬜` → `✅`

2. Announce next phase

**Note:** Do NOT commit to git. User will review code and commit manually.

### Status Icons

| Icon | Meaning |
|---------|---------|
| ⬜ | Not started |
| 🔄 | In progress |
| ✅ | Complete |
| ⚠️ | Blocked |
| ❌ | Failed/Rejected |

### Workflow Rules

| Rule | Description |
|------|-------------|
| **No skipping** | Follow tasklist order strictly |
| **No surprise code** | Always propose first |
| **No auto-commit** | User reviews and commits manually |
| **Always test** | Verify before marking complete |
| **Always wait** | Get confirmation before proceeding |

## Current Project Status

See [doc/tasklist.md](doc/tasklist.md) for the complete development task list and current progress.

### Progress Summary (as of latest update)

| Phase | Status |
|-------|--------|
| 1. Workspace Setup | ✅ Complete (8/8) |
| 2. Shared Types | ✅ Complete (3/3) |
| 3. Server Core | ✅ Complete (6/6) |
| 4. Auth API | ✅ Complete (4/4) |
| 5. Posts API | ✅ Complete (5/5) |
| 6. gRPC API | ✅ Complete (5/5) |
| 7. Client Library | ✅ Complete (4/4) |
| 8. CLI | ✅ Complete (4/4) |
| 9. WASM Frontend | ✅ Complete (6/6) |
| 10. Final Polish | ⬜ Not Started (0/3) |

**Current Phase**: Phase 10 - Final Polish (integration tests, README, code review)

**Note**: This status may be outdated. Always check [doc/tasklist.md](doc/tasklist.md) for the most current information.

## Additional Documentation

### Core Documentation

- [README.md](README.md) - Quick start guide and project setup
- [idea.md](idea.md) - Original project idea and specifications
- [vision.md](vision.md) - Complete technical vision and architecture decisions
- [conventions.md](conventions.md) - Detailed code conventions and patterns

### Development Guides

- [doc/workflow.md](doc/workflow.md) - Development workflow (PROPOSE → AGREE → IMPLEMENT → CONFIRM → UPDATE)
- [doc/tasklist.md](doc/tasklist.md) - Phase-by-phase task breakdown and progress tracking

### Quick Reference

For any development work:
1. Check [doc/tasklist.md](doc/tasklist.md) for current phase
2. Follow workflow in [doc/workflow.md](doc/workflow.md)
3. Apply conventions from [conventions.md](conventions.md)
4. Reference architecture in [vision.md](vision.md)
