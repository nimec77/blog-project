# Blog Project - Rust Training Project Idea

## Project Overview

A comprehensive Rust training project implementing a full-stack blog platform with five crates in a single Cargo workspace. The project demonstrates real-world development patterns including clean architecture, JWT authentication, database integration, and multi-transport API support (HTTP + gRPC).

**Goal:** Build a production-ready personal blog system with user registration, JWT protection, and secure database storage that can be deployed to a custom domain.

---

## Architecture

### Workspace Structure

```
blog_project/
├── Cargo.toml                    # Workspace configuration
├── README.md                     # Project description, startup instructions
├── blog-shared/                  # Crate 1: Shared types and DTOs
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs                # DTOs (User, Post, AuthResponse, etc.)
│       └── constants.rs          # Shared constants
├── blog-server/                  # Crate 2: Blog web server
│   ├── Cargo.toml
│   ├── build.rs
│   ├── migrations/
│   │   ├── 20251205151238_create_users.sql
│   │   └── 20251205151239_create_posts.sql
│   ├── proto/
│   │   └── blog.proto
│   └── src/
│       ├── main.rs
│       ├── domain/
│       │   ├── mod.rs
│       │   ├── post.rs           # Post Domain Model
│       │   ├── user.rs           # User Domain Model
│       │   └── error.rs          # Domain Errors
│       ├── application/
│       │   ├── mod.rs
│       │   ├── auth_service.rs   # Authentication Service
│       │   └── blog_service.rs   # Blog Business Logic
│       ├── data/
│       │   ├── mod.rs
│       │   ├── user_repository.rs
│       │   └── post_repository.rs
│       ├── infrastructure/
│       │   ├── mod.rs
│       │   ├── database.rs       # Database connection
│       │   ├── jwt.rs            # JWT token handling
│       │   └── logging.rs        # Tracing setup
│       └── presentation/
│           ├── mod.rs
│           ├── middleware.rs     # JWT middleware for actix-web
│           ├── http_handlers.rs  # HTTP handlers for actix-web
│           └── grpc_service.rs   # gRPC service for tonic
├── blog-client/                  # Crate 3: Client Library
│   ├── Cargo.toml
│   ├── build.rs
│   ├── proto/
│   │   └── blog.proto            # Copy from blog-server
│   └── src/
│       ├── lib.rs
│       ├── http_client.rs
│       ├── grpc_client.rs
│       └── error.rs
├── blog-cli/                     # Crate 4: CLI Client
│   ├── Cargo.toml
│   └── src/
│       └── main.rs
└── blog-wasm/                    # Crate 5: WASM Frontend
    ├── Cargo.toml
    └── src/
        └── lib.rs
```

---

## Crate Specifications

### 1. blog-shared (Library Crate)

**Purpose:** Shared types and DTOs used across all crates.

**Exports:**
- `UserDto` - User information (no password hash)
- `PostDto` - Post with author information
- `AuthResponse` - JWT token + user
- `RegisterRequest`, `LoginRequest` - Auth payloads
- `CreatePostRequest`, `UpdatePostRequest` - Post payloads
- `PostListResponse` - Paginated posts
- Constants (environment variables, default ports)

**Dependencies:**
- serde (JSON serialization)
- chrono (DateTime types)

---

### 2. blog-server (Binary Crate)

**Purpose:** Backend server hosting both HTTP and gRPC APIs.

**Technology Stack:**
- **HTTP Server:** actix-web (port 8080)
- **gRPC Server:** tonic (port 50051)
- **Database:** SQLite via sqlx
- **Authentication:** JWT (jsonwebtoken) + Argon2 password hashing
- **Logging:** tracing + tracing-subscriber
- **Error Handling:** thiserror

**Architecture:** Clean Architecture
- `domain/` - Core business entities (User, Post, Error types)
- `application/` - Business logic services (AuthService, BlogService)
- `data/` - Repository implementations (UserRepository, PostRepository)
- `infrastructure/` - Technical concerns (database, JWT, logging)
- `presentation/` - API layer (HTTP handlers, gRPC service, middleware)

**HTTP API Endpoints:**
| Method | Endpoint | Auth | Description |
|--------|----------|------|-------------|
| POST | `/api/auth/register` | Public | Register new user |
| POST | `/api/auth/login` | Public | User login |
| POST | `/api/posts` | Required | Create post |
| GET | `/api/posts/{id}` | Public | Get post by ID |
| PUT | `/api/posts/{id}` | Required | Update post (author only) |
| DELETE | `/api/posts/{id}` | Required | Delete post (author only) |
| GET | `/api/posts` | Public | List posts (paginated) |

**gRPC Service Methods:**
- `Register`, `Login` - Authentication
- `CreatePost`, `GetPost`, `UpdatePost`, `DeletePost`, `ListPosts` - CRUD operations

**Database Schema:**
- `users` table: id (INTEGER PRIMARY KEY AUTOINCREMENT), username, email, password_hash, created_at
- `posts` table: id (INTEGER PRIMARY KEY AUTOINCREMENT), title, content, author_id (FK), created_at, updated_at

**Security Features:**
- Argon2 password hashing
- JWT token generation/validation (24h expiry)
- Parameterized SQL queries (no SQL injection)
- CORS configuration for WASM frontend

---

### 3. blog-client (Library Crate)

**Purpose:** Unified client library for HTTP and gRPC communication with the server.

**Features:**
- Dual transport support: `Transport::Http` and `Transport::Grpc`
- Single interface regardless of transport
- Automatic JWT token management
- Error handling with custom `BlogClientError` type

**Public API:**
```rust
struct BlogClient {
    fn new(transport: Transport) -> Self;
    fn set_token(token: &str);
    fn get_token() -> Option<String>;
    fn register(username, email, password) -> Result<AuthResponse>;
    fn login(username, password) -> Result<AuthResponse>;
    fn create_post(title, content) -> Result<Post>;
    fn get_post(id) -> Result<Post>;
    fn update_post(id, title, content) -> Result<Post>;
    fn delete_post(id) -> Result<()>;
    fn list_posts(limit, offset) -> Result<Vec<Post>>;
}
```

**Dependencies:**
- HTTP: reqwest with JSON feature
- gRPC: tonic, prost, prost-types
- Utilities: serde, chrono, thiserror

---

### 4. blog-cli (Binary Crate)

**Purpose:** Command-line interface for testing and interacting with the blog server.

**Technology:** clap (derive feature) for argument parsing

**Commands:**
```bash
blog-cli register --username "user" --email "user@example.com" --password "secret"
blog-cli login --username "user" --password "secret"
blog-cli create --title "Title" --content "Content"
blog-cli get --id 1
blog-cli update --id 1 --title "New Title" [--content "New Content"]
blog-cli delete --id 1
blog-cli list [--limit 10] [--offset 0]
```

**Global Flags:**
- `--grpc` - Use gRPC transport instead of HTTP
- `--server <address>` - Specify server address

**Features:**
- Token persistence in `.blog_token` file
- Uses blog-client library (no code duplication)
- User-friendly error messages

---

### 5. blog-wasm (Library Crate - cdylib)

**Purpose:** WebAssembly frontend running in the browser.

**Technology Options:**
- wasm-bindgen + web-sys (basic)
- Yew, Leptos, Dioxus, or egui (framework-based)

**Features:**
- User registration form
- Login form
- JWT token storage in localStorage
- Post list display (public)
- Post creation form (authenticated)
- Post editing (author only)
- Post deletion (author only)
- Authentication status display
- Form validation

**Technical Notes:**
- Uses HTTP transport only (gRPC not supported in browsers)
- Direct HTTP requests via gloo-net or web-sys
- Returns `Result<JsValue, JsValue>` for JavaScript interop

---

## Implementation Steps

### Step 1: Workspace Setup
- [x] Initialize Cargo workspace with five crates
- [ ] Configure shared dependencies in `[workspace.dependencies]`
- [ ] Set up blog-wasm with `crate-type = ["cdylib"]`
- [ ] Create folder structure
- [ ] Configure `.gitignore`

### Step 2: Protobuf Schema
- [ ] Define `blog.proto` with BlogService
- [ ] Define message types (RegisterRequest, LoginRequest, AuthResponse, Post, etc.)
- [ ] Configure `build.rs` in both server and client crates
- [ ] Copy proto file to client crate

### Step 3: Web Server Implementation
- [ ] Domain models (User, Post, Error types)
- [ ] Database migrations (users, posts tables)
- [ ] Repository implementations
- [ ] Authentication service (Argon2 + JWT)
- [ ] Blog service (CRUD operations)
- [ ] HTTP handlers with actix-web
- [ ] gRPC service with tonic
- [ ] JWT middleware
- [ ] CORS configuration
- [ ] Parallel server startup (HTTP + gRPC)

### Step 4: Client Library
- [ ] Transport abstraction (Http/Grpc)
- [ ] HTTP client implementation (reqwest)
- [ ] gRPC client implementation (tonic)
- [ ] Error handling
- [ ] Token management

### Step 5: CLI Client
- [ ] Command parsing with clap
- [ ] Token persistence
- [ ] All CRUD commands
- [ ] Transport switching (--grpc flag)

### Step 6: WASM Frontend
- [ ] BlogApp structure with wasm-bindgen
- [ ] localStorage token management
- [ ] Registration/Login forms
- [ ] Post list display
- [ ] Post CRUD operations
- [ ] UI state management
- [ ] index.html integration

### Step 7: Documentation
- [ ] Comprehensive README.md
- [ ] Installation instructions
- [ ] Usage examples (curl, CLI, browser)

---

## Key Dependencies

### Workspace-Level
```toml
[workspace.dependencies]
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
chrono = { version = "0.4", features = ["serde"] }
anyhow = "1"
thiserror = "1"
tracing = "0.1"
tracing-subscriber = "0.3"
```

### blog-server
- actix-web, actix-cors
- tonic, tonic-build, prost, prost-types
- sqlx (SQLite features)
- jsonwebtoken, argon2
- dotenvy

### blog-client
- reqwest (json feature)
- tonic, prost

### blog-cli
- clap (derive feature)
- blog-client (path dependency)

### blog-wasm
- wasm-bindgen, wasm-bindgen-futures
- web-sys, gloo-net
- serde-wasm-bindgen, js-sys

---

## Environment Variables

```env
DATABASE_URL=sqlite:blog.db
JWT_SECRET=your-secret-key-minimum-32-characters-long
HTTP_PORT=8080
GRPC_PORT=50051
RUST_LOG=blog_server=debug,info
```

---

## Quality Requirements

### Code Quality
- Meaningful variable/function names
- No magic numbers (use constants)
- Comments on complex logic
- Use tracing instead of println!
- No compiler warnings
- Rust idioms (Result, Option, match)

### Security
- Argon2 password hashing (never plaintext)
- Parameterized SQL queries
- JWT validation on protected routes
- CORS properly configured

### Architecture
- Clean architecture separation
- No code duplication between crates
- Common types in client library
- Proper error handling and propagation

---

## Testing Checklist

- [ ] `cargo build --workspace` succeeds
- [ ] HTTP API endpoints respond correctly
- [ ] gRPC service methods work
- [ ] CLI commands execute properly
- [ ] WASM frontend runs in browser
- [ ] Authentication flow works end-to-end
- [ ] Post CRUD operations work
- [ ] Error cases handled gracefully

