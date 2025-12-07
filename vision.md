# Blog Project - Technical Vision

> **Principle:** KISS - Keep It Simple, Stupid. No overengineering. Only essentials.

---

## 1. Technologies

### Core Stack

| Layer | Technology | Version Strategy |
|-------|------------|------------------|
| **Runtime** | Rust (stable) | Latest stable |
| **Async Runtime** | tokio | 1.x |

### Server (blog-server)

| Purpose | Technology | Notes |
|---------|------------|-------|
| HTTP API | actix-web | Required by task |
| gRPC API | tonic | With prost for protobuf |
| Database | SQLite via sqlx | Compile-time checked queries |
| Auth | jsonwebtoken + argon2 | JWT tokens, secure password hashing |
| Serialization | serde | JSON for HTTP, protobuf for gRPC |
| CORS | actix-cors | Strict policy for WASM frontend |

### Client Library (blog-client)

| Purpose | Technology |
|---------|------------|
| HTTP Client | reqwest |
| gRPC Client | tonic |

### CLI (blog-cli)

| Purpose | Technology |
|---------|------------|
| Argument Parsing | clap (derive) |

### WASM Frontend (blog-wasm)

| Purpose | Technology |
|---------|------------|
| Framework | Yew |
| HTTP Requests | gloo-net |
| Build Tool | trunk |

### Shared

| Purpose | Technology |
|---------|------------|
| Error Handling | thiserror |
| Logging | tracing |
| Date/Time | chrono |

---

## 2. Development Principles

### Core Principles

| Principle | Description |
|-----------|-------------|
| **KISS First** | Simplest solution that works. No abstractions until needed twice. |
| **Fail Fast** | Return errors early with `?`. Validate at API boundaries only. |
| **Single Responsibility** | One file = one purpose. Functions do one thing. |
| **No Premature Optimization** | Make it work first. Optimize only with evidence. |

### Testing Strategy

| Level | Scope | Tools |
|-------|-------|-------|
| **Unit Tests** | Domain logic, services | `#[cfg(test)]` modules |
| **Integration Tests** | API endpoints, DB operations | `tests/` folder, test database |
| **E2E Tests** | Full workflows via CLI | CLI commands against test server |

**Testing Rules:**
- Each public function has at least one test
- Happy path + one error case minimum
- Use `#[tokio::test]` for async tests
- SQLite in-memory database for test isolation

### Error Handling

| Approach | Implementation |
|----------|----------------|
| **One enum per crate** | `AppError` in blog-server, `ClientError` in blog-client |
| **thiserror derive** | Auto-generate `Error` trait |
| **Conversion traits** | `From<SqlxError>`, `From<JwtError>`, etc. |

### Documentation

| Type | Where | When |
|------|-------|------|
| **Doc comments** (`///`) | All public APIs | Always |
| **Code comments** (`//`) | Complex logic | When not obvious |
| **No comments** | Self-explanatory code | Prefer clear naming |

---

## 3. Project Structure

```
blog-project/
├── Cargo.toml                    # Workspace config (5 members)
├── .env                          # Local environment (git-ignored)
├── .gitignore
├── README.md
│
├── blog-shared/                  # Crate 1: Shared types
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs                # User, Post, AuthResponse DTOs
│       └── constants.rs          # Shared constants
│
├── blog-server/                  # Crate 2: Web server
│   ├── Cargo.toml
│   ├── build.rs
│   ├── blog.db                   # SQLite (git-ignored)
│   ├── migrations/
│   │   ├── 001_create_users.sql
│   │   └── 002_create_posts.sql
│   ├── proto/
│   │   └── blog.proto
│   └── src/
│       ├── main.rs
│       ├── constants.rs          # Server constants
│       ├── domain/
│       │   ├── mod.rs
│       │   ├── user.rs           # User entity
│       │   ├── post.rs           # Post entity
│       │   └── error.rs          # Domain errors
│       ├── application/
│       │   ├── mod.rs
│       │   ├── auth_service.rs
│       │   └── blog_service.rs
│       ├── data/
│       │   ├── mod.rs
│       │   ├── user_repository.rs
│       │   └── post_repository.rs
│       ├── infrastructure/
│       │   ├── mod.rs
│       │   ├── database.rs
│       │   ├── jwt.rs
│       │   └── config.rs
│       └── presentation/
│           ├── mod.rs
│           ├── http_handlers.rs
│           ├── grpc_service.rs
│           └── middleware.rs
│
├── blog-client/                  # Crate 3: Client library
│   ├── Cargo.toml
│   ├── build.rs
│   ├── proto/
│   │   └── blog.proto
│   └── src/
│       ├── lib.rs
│       ├── http_client.rs
│       ├── grpc_client.rs
│       └── error.rs
│
├── blog-cli/                     # Crate 4: CLI client
│   ├── Cargo.toml
│   └── src/
│       ├── main.rs               # Entry point, CLI parsing
│       ├── commands.rs           # Command execution logic
│       └── constants.rs          # CLI constants
│
└── blog-wasm/                    # Crate 5: WASM frontend
    ├── Cargo.toml
    ├── index.html
    ├── Trunk.toml
    └── src/
        ├── lib.rs                # Yew app entry
        ├── api.rs                # HTTP client
        └── components/
            ├── mod.rs
            ├── header.rs
            ├── login_form.rs
            ├── register_form.rs
            ├── post_list.rs
            ├── post_form.rs
            └── post_card.rs
```

### Workspace Members

| Crate | Type | Purpose |
|-------|------|---------|
| blog-shared | lib | Common DTOs (User, Post, AuthResponse) |
| blog-server | bin | HTTP + gRPC server |
| blog-client | lib | Client library for HTTP/gRPC |
| blog-cli | bin | Command-line interface |
| blog-wasm | lib (cdylib) | Yew frontend |

### Dependency Graph

```
blog-shared ◄─── blog-server
     ▲             
     │             
     ├──────── blog-client ◄─── blog-cli
     │
     └──────── blog-wasm
```

---

## 4. Project Architecture

### Layer Diagram (blog-server)

```
┌─────────────────────────────────────────────────────────────┐
│                      PRESENTATION                           │
│  ┌─────────────────┐  ┌─────────────────┐  ┌─────────────┐  │
│  │  http_handlers  │  │  grpc_service   │  │  middleware │  │
│  └────────┬────────┘  └────────┬────────┘  └─────────────┘  │
│           │                    │                            │
├───────────┴────────────────────┴────────────────────────────┤
│                       APPLICATION                           │
│  ┌─────────────────────────┐  ┌─────────────────────────┐   │
│  │      AuthService        │  │      BlogService        │   │
│  └────────────┬────────────┘  └────────────┬────────────┘   │
│               │                            │                │
├───────────────┴────────────────────────────┴────────────────┤
│                          DATA                               │
│  ┌─────────────────────────┐  ┌─────────────────────────┐   │
│  │    UserRepository       │  │    PostRepository       │   │
│  └─────────────────────────┘  └─────────────────────────┘   │
│                                                             │
├─────────────────────────────────────────────────────────────┤
│                       DOMAIN                                │
│  ┌──────────┐  ┌──────────┐  ┌──────────────────────────┐   │
│  │   User   │  │   Post   │  │     DomainError          │   │
│  └──────────┘  └──────────┘  └──────────────────────────┘   │
│                                                             │
├─────────────────────────────────────────────────────────────┤
│                    INFRASTRUCTURE                           │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐                   │
│  │ database │  │   jwt    │  │  config  │                   │
│  └──────────┘  └──────────┘  └──────────┘                   │
└─────────────────────────────────────────────────────────────┘
```

### Layer Responsibilities

| Layer | Responsibility | Depends On |
|-------|----------------|------------|
| **Domain** | Entities, validation, domain errors | Nothing |
| **Data** | Repository traits + SQLite implementations | Domain |
| **Application** | Thin services, orchestrate repositories | Domain, Data |
| **Infrastructure** | DB connection, JWT, config | External crates |
| **Presentation** | HTTP/gRPC handlers, request mapping | Application |

### Repository Pattern

```rust
// Trait for testing
#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_by_id(&self, id: i64) -> Result<Option<User>>;
    async fn find_by_username(&self, username: &str) -> Result<Option<User>>;
    async fn save(&self, user: &User) -> Result<User>;
}

// SQLite implementation
pub struct SqliteUserRepository { pool: SqlitePool }
impl UserRepository for SqliteUserRepository { ... }
```

### Dependency Injection Per Crate

| Crate | Pattern | Implementation |
|-------|---------|----------------|
| **blog-server** | AppState | `web::Data<AppState>` with `Arc<dyn Trait>` |
| **blog-client** | Simple struct | Wraps reqwest/tonic clients |
| **blog-cli** | Direct usage | Instantiate `BlogClient` |
| **blog-wasm** | Yew Context | `use_context::<ApiClient>()` |

### Service Layer (Thin)

```rust
// Services coordinate, don't contain complex logic
impl BlogService {
    pub async fn create_post(&self, author_id: i64, req: CreatePostRequest) -> Result<Post> {
        let post = Post::new(author_id, req.title, req.content);
        self.post_repo.save(&post).await
    }
}
```

### Business Rules Location

| Type | Location |
|------|----------|
| Validation | Domain entities (`Post::new()`) |
| Authorization | Presentation layer (middleware, handlers) |
| Persistence | Data layer (repositories) |
| Orchestration | Application layer (services) |

---

## 5. Data Model

### Database: SQLite

#### Table: users

| Column | Type | Constraints |
|--------|------|-------------|
| id | INTEGER | PRIMARY KEY AUTOINCREMENT |
| username | TEXT | NOT NULL, UNIQUE |
| email | TEXT | NOT NULL, UNIQUE |
| password_hash | TEXT | NOT NULL |
| created_at | TEXT | NOT NULL (ISO 8601) |

**Indexes:** `username` (login lookup), `email` (unique constraint)

#### Table: posts

| Column | Type | Constraints |
|--------|------|-------------|
| id | INTEGER | PRIMARY KEY AUTOINCREMENT |
| title | TEXT | NOT NULL |
| content | TEXT | NOT NULL |
| author_id | INTEGER | NOT NULL, FK → users(id) ON DELETE CASCADE |
| created_at | TEXT | NOT NULL (ISO 8601) |
| updated_at | TEXT | NOT NULL (ISO 8601) |

**Indexes:** `author_id` (FK), `created_at` (sorting)

### Domain Entities (blog-server)

```rust
// domain/user.rs
pub struct User {
    pub id: i64,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
}

// domain/post.rs
pub struct Post {
    pub id: i64,
    pub title: String,
    pub content: String,
    pub author_id: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
```

### Shared DTOs (blog-shared)

```rust
// UserDto - no password_hash exposed
pub struct UserDto {
    pub id: i64,
    pub username: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
}

// PostDto - includes author_username to avoid N+1 queries
pub struct PostDto {
    pub id: i64,
    pub title: String,
    pub content: String,
    pub author_id: i64,
    pub author_username: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// AuthResponse - returned after login/register
pub struct AuthResponse {
    pub token: String,
    pub user: UserDto,
}
```

### Request DTOs (blog-shared)

```rust
pub struct RegisterRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

pub struct CreatePostRequest {
    pub title: String,
    pub content: String,
}

pub struct UpdatePostRequest {
    pub title: Option<String>,
    pub content: Option<String>,
}
```

---

## 6. Workflows

### Authentication Flow

```
┌─────────┐                              ┌─────────┐
│ Client  │                              │ Server  │
└────┬────┘                              └────┬────┘
     │                                        │
     │  POST /api/auth/register               │
     │  {username, email, password}           │
     │ ──────────────────────────────────────►│
     │                                        │ Validate
     │                                        │ Hash password (Argon2)
     │                                        │ Save user
     │                                        │ Generate JWT (24h)
     │  201 {token, user}                     │
     │ ◄──────────────────────────────────────│
     │                                        │
     │  Store token in localStorage           │
     │                                        │
```

### Post CRUD Flow

```
┌─────────┐                              ┌─────────┐
│ Client  │                              │ Server  │
└────┬────┘                              └────┬────┘
     │                                        │
     │  GET /api/posts?limit=10&offset=0      │  (Public)
     │ ──────────────────────────────────────►│
     │  200 {posts, total}                    │
     │ ◄──────────────────────────────────────│
     │                                        │
     │  POST /api/posts                       │  (Auth required)
     │  Authorization: Bearer <token>         │
     │  {title, content}                      │
     │ ──────────────────────────────────────►│
     │                                        │ Validate JWT
     │                                        │ Create post
     │  201 {post}                            │
     │ ◄──────────────────────────────────────│
     │                                        │
     │  PUT /api/posts/{id}                   │  (Author only)
     │  Authorization: Bearer <token>         │
     │ ──────────────────────────────────────►│
     │                                        │ Check author_id == user_id
     │  200 {post} or 403 Forbidden           │
     │ ◄──────────────────────────────────────│
```

### Token Management

| Aspect | Decision |
|--------|----------|
| Storage | localStorage (browser) |
| Expiry | 24 hours |
| Refresh | None (re-login required) |
| Header | `Authorization: Bearer <token>` |

### Pagination

```rust
// Response format
pub struct PostListResponse {
    pub posts: Vec<PostDto>,
    pub total: i64,
}

// Request: GET /api/posts?limit=10&offset=0
// Default: limit=10, offset=0
```

---

## 7. Configuration Approach

### Strategy: Environment Variables + `.env`

#### blog-server Config

| Variable | Required | Default | Description |
|----------|----------|---------|-------------|
| `DATABASE_URL` | Yes | - | `sqlite:blog.db` |
| `JWT_SECRET` | Yes | - | Token signing key (32+ chars) |
| `HTTP_PORT` | No | `8080` | HTTP server port |
| `GRPC_PORT` | No | `50051` | gRPC server port |
| `RUST_LOG` | No | `info` | Log level |

#### Example `.env`

```env
DATABASE_URL=sqlite:blog.db
JWT_SECRET=your-super-secret-key-at-least-32-chars
HTTP_PORT=8080
GRPC_PORT=50051
RUST_LOG=blog_server=debug,info
```

#### Config Implementation

```rust
// infrastructure/config.rs
pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
    pub http_port: u16,
    pub grpc_port: u16,
}

impl Config {
    pub fn from_env() -> Self {
        dotenvy::dotenv().ok();
        
        Self {
            database_url: env::var("DATABASE_URL")
                .expect("DATABASE_URL is required"),
            jwt_secret: env::var("JWT_SECRET")
                .expect("JWT_SECRET is required"),
            http_port: env::var("HTTP_PORT")
                .unwrap_or_else(|_| "8080".into())
                .parse()
                .expect("HTTP_PORT must be a number"),
            grpc_port: env::var("GRPC_PORT")
                .unwrap_or_else(|_| "50051".into())
                .parse()
                .expect("GRPC_PORT must be a number"),
        }
    }
}
```

#### blog-cli Config

CLI arguments only (no env vars):

```bash
blog-cli --server http://localhost:8080 list
blog-cli --server grpc://localhost:50051 --grpc list
```

Default: `http://localhost:8080`

---

## 8. Logging Approach

### Stack: `tracing` + `tracing-subscriber`

### Log Levels

| Level | Usage |
|-------|-------|
| `ERROR` | Unrecoverable errors (DB connection failed) |
| `WARN` | Recoverable issues (invalid token, not found) |
| `INFO` | Important events (server started, user registered) |
| `DEBUG` | Detailed flow (request received, query executed) |

### Format Per Crate

| Crate | Format | Reason |
|-------|--------|--------|
| blog-server | **JSON** | Structured, production-ready, parseable |
| blog-cli | **Plain text** | Human-readable in terminal |
| blog-wasm | **Browser console** | Via `console_log` / `console_error` |

### Server Log Format (JSON)

```json
{"timestamp":"2025-01-15T10:30:00Z","level":"INFO","target":"blog_server::http","request_id":"abc123","message":"User registered","username":"john"}
```

### CLI Log Format (Plain)

```
INFO  blog_cli: Connected to server url="http://localhost:8080"
```

### Request ID Correlation

```rust
// Middleware adds unique request_id to each request
// All logs within request include this ID

// Request comes in
DEBUG request_id="abc123" method="POST" path="/api/posts"

// Service layer
DEBUG request_id="abc123" "Creating post for user" user_id=42

// Response
INFO  request_id="abc123" status=201 duration_ms=15
```

### What to Log

| Event | Level | Fields |
|-------|-------|--------|
| Server startup | INFO | port, version |
| Request start | DEBUG | request_id, method, path |
| Request end | INFO | request_id, status, duration_ms |
| Auth success | INFO | request_id, username |
| Auth failure | WARN | request_id, reason |
| DB error | ERROR | request_id, error |

### Sensitive Data: NEVER LOG

- Passwords
- JWT tokens
- Email addresses
- Full request bodies with sensitive fields

### Implementation

```rust
// blog-server: JSON format
tracing_subscriber::fmt()
    .json()
    .with_env_filter(EnvFilter::from_default_env())
    .init();

// blog-cli: Plain format
tracing_subscriber::fmt()
    .with_env_filter(EnvFilter::from_default_env())
    .init();
```

---

## Summary

### Key Decisions

| Aspect | Decision |
|--------|----------|
| HTTP Framework | actix-web |
| Database | SQLite + sqlx (compile-time) |
| Frontend | Yew |
| Architecture | Clean Architecture (5 layers) |
| Shared Types | blog-shared crate |
| Repository | Trait + Impl pattern |
| Services | Thin (coordinators) |
| Auth | JWT (24h, no refresh) |
| Passwords | Argon2 |
| Config | Env vars + panic on invalid |
| Logging | tracing (JSON server, plain CLI) |

### KISS Principles Applied

1. **No over-abstraction** - Traits only where needed for testing
2. **Simple pagination** - Just `posts + total`
3. **No token refresh** - Re-login after 24h
4. **SQLite** - No external DB server needed
5. **Flat structures** - Minimal nesting
6. **Fail fast** - Panic on bad config at startup

### Next Steps

1. Create workspace structure
2. Implement blog-shared (DTOs)
3. Implement blog-server (core)
4. Implement blog-client (library)
5. Implement blog-cli
6. Implement blog-wasm (Yew frontend)
7. Write tests
8. Documentation

