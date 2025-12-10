# Blog Project

A full-stack Rust blog platform with HTTP and gRPC APIs, command-line client, and WebAssembly frontend. Built as a Cargo workspace demonstrating clean architecture, JWT authentication, and multi-transport API support.

## Features

- **Dual API Support**: HTTP (actix-web) and gRPC (tonic) servers running in parallel
- **JWT Authentication**: Secure user registration and login with Argon2 password hashing
- **Full CRUD Operations**: Create, read, update, and delete blog posts
- **Multiple Clients**:
  - CLI client with HTTP and gRPC transport options
  - WASM frontend built with Yew framework
- **Clean Architecture**: Layered design with domain, data, application, and presentation layers
- **SQLite Database**: Simple setup with compile-time checked queries via sqlx

## Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                        blog-project                              │
├─────────────────────────────────────────────────────────────────┤
│  blog-shared     │  Shared DTOs (User, Post, AuthResponse)      │
│  blog-server     │  HTTP + gRPC server (main binary)            │
│  blog-client     │  Client library for HTTP/gRPC communication  │
│  blog-cli        │  Command-line interface                      │
│  blog-wasm       │  Yew WASM frontend                           │
└─────────────────────────────────────────────────────────────────┘
```

### Server Layer Architecture

```
presentation/     HTTP handlers + gRPC service
       ↓
application/      AuthService, BlogService
       ↓
data/             UserRepository, PostRepository
       ↓
domain/           User, Post entities + errors
       ↓
infrastructure/   Database, JWT, Config
```

## Prerequisites

- **Rust** (stable, latest version recommended)
- **SQLite** (included with most systems)
- **Trunk** (for WASM frontend): `cargo install trunk`
- **grpcurl** (optional, for gRPC testing): `brew install grpcurl` or equivalent

## Quick Start

### 1. Clone and Setup

```bash
git clone <repo-url>
cd blog-project

# Copy environment config
cp .env.example .env

# Edit .env and set a secure JWT_SECRET (min 32 characters)
```

### 2. Initialize Database

The project uses sqlx compile-time checked queries, which require the database to exist before building.

```bash
# Install sqlx-cli (one-time)
cargo install sqlx-cli --no-default-features --features sqlite

# Create database and run migrations
sqlx database create --database-url sqlite:blog.db
sqlx migrate run --source blog-server/migrations --database-url sqlite:blog.db
```

### 3. Build and Run Server

```bash
# Build all crates
cargo build --workspace

# Run the server (starts HTTP on 8080, gRPC on 50051)
cargo run -p blog-server
```

### 4. Test the API

```bash
# Health check
curl http://localhost:8080/health

# Register a user
curl -X POST http://localhost:8080/api/auth/register \
  -H "Content-Type: application/json" \
  -d '{"username":"alice","email":"alice@example.com","password":"secret123"}'
```

## Configuration

Create a `.env` file in the project root (copy from `.env.example`):

```env
# Required
DATABASE_URL=sqlite:blog.db
JWT_SECRET=your-super-secret-key-at-least-32-characters-long

# Optional (with defaults)
HTTP_PORT=8080
GRPC_PORT=50051
RUST_LOG=blog_server=debug,info
```

| Variable | Required | Default | Description |
|----------|----------|---------|-------------|
| `DATABASE_URL` | Yes | - | SQLite database path |
| `JWT_SECRET` | Yes | - | JWT signing key (32+ chars) |
| `HTTP_PORT` | No | 8080 | HTTP server port |
| `GRPC_PORT` | No | 50051 | gRPC server port |
| `RUST_LOG` | No | info | Log level |

## Running the Server

```bash
# Development mode
cargo run -p blog-server

# Release mode
cargo run -p blog-server --release

# With debug logging
RUST_LOG=debug cargo run -p blog-server
```

The server starts both HTTP and gRPC services:
- **HTTP API**: http://localhost:8080
- **gRPC API**: http://localhost:50051

## CLI Usage

The CLI client supports both HTTP and gRPC transports.

### Global Flags

| Flag | Description |
|------|-------------|
| `--grpc` | Use gRPC transport instead of HTTP |
| `--server <URL>` | Custom server URL |

### Authentication Commands

```bash
# Register a new user
cargo run -p blog-cli -- register \
  --username alice \
  --email alice@example.com \
  --password secret123

# Login (saves token to ~/.blog_token)
cargo run -p blog-cli -- login \
  --username alice \
  --password secret123
```

### Post Commands

```bash
# Create a post (requires login)
cargo run -p blog-cli -- create \
  --title "My First Post" \
  --content "Hello, world! This is my first blog post."

# List all posts
cargo run -p blog-cli -- list

# List with pagination
cargo run -p blog-cli -- list --limit 5 --offset 10

# Get a specific post
cargo run -p blog-cli -- get --id 1

# Update a post (author only)
cargo run -p blog-cli -- update --id 1 \
  --title "Updated Title" \
  --content "Updated content"

# Delete a post (author only)
cargo run -p blog-cli -- delete --id 1
```

### Using gRPC Transport

```bash
# All commands work with --grpc flag
cargo run -p blog-cli -- --grpc register \
  --username bob \
  --email bob@example.com \
  --password secret123

cargo run -p blog-cli -- --grpc list
```

### Custom Server Address

```bash
# HTTP server on different port
cargo run -p blog-cli -- --server http://localhost:3000 list

# gRPC server on different port
cargo run -p blog-cli -- --grpc --server http://localhost:9000 list
```

## Web Frontend (WASM)

The WASM frontend is built with Yew framework.

### Running the Frontend

```bash
# Install trunk (one-time)
cargo install trunk

# Navigate to frontend directory
cd blog-wasm

# Start development server (opens browser automatically)
trunk serve --open
```

The frontend runs at http://127.0.0.1:8081

### Frontend Features

- **Home Page** (`/`): View all blog posts with pagination
- **Login** (`/login`): User authentication
- **Register** (`/register`): Create new account
- **Create Post** (`/posts/new`): Write new blog post (requires auth)
- **Edit Post** (`/posts/{id}/edit`): Modify existing post (author only)
- **Delete Post**: Remove posts from the post list view (author only)

### Frontend Configuration

The frontend connects to the backend API at `http://localhost:8080`. JWT tokens are stored in localStorage for session persistence.

## HTTP API Reference

### Public Endpoints

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/health` | Health check |
| POST | `/api/auth/register` | Register new user |
| POST | `/api/auth/login` | User login |
| GET | `/api/posts` | List posts (paginated) |
| GET | `/api/posts/{id}` | Get single post |

### Protected Endpoints

Require `Authorization: Bearer <token>` header.

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/api/auth/me` | Get current user |
| POST | `/api/posts` | Create post |
| PUT | `/api/posts/{id}` | Update post (author only) |
| DELETE | `/api/posts/{id}` | Delete post (author only) |

### Request/Response Examples

#### Register User

```bash
curl -X POST http://localhost:8080/api/auth/register \
  -H "Content-Type: application/json" \
  -d '{
    "username": "alice",
    "email": "alice@example.com",
    "password": "secret123"
  }'
```

Response (201 Created):
```json
{
  "token": "eyJhbGciOiJIUzI1NiIs...",
  "user": {
    "id": 1,
    "username": "alice",
    "email": "alice@example.com",
    "created_at": "2025-01-15T10:30:00Z"
  }
}
```

#### Login

```bash
curl -X POST http://localhost:8080/api/auth/login \
  -H "Content-Type: application/json" \
  -d '{
    "username": "alice",
    "password": "secret123"
  }'
```

#### Create Post

```bash
curl -X POST http://localhost:8080/api/posts \
  -H "Authorization: Bearer <token>" \
  -H "Content-Type: application/json" \
  -d '{
    "title": "My Post",
    "content": "Post content here"
  }'
```

Response (201 Created):
```json
{
  "id": 1,
  "title": "My Post",
  "content": "Post content here",
  "author_id": 1,
  "author_username": "alice",
  "created_at": "2025-01-15T10:30:00Z",
  "updated_at": "2025-01-15T10:30:00Z"
}
```

#### List Posts

```bash
curl "http://localhost:8080/api/posts?limit=10&offset=0"
```

Response (200 OK):
```json
{
  "posts": [
    {
      "id": 1,
      "title": "My Post",
      "content": "Post content here",
      "author_id": 1,
      "author_username": "alice",
      "created_at": "2025-01-15T10:30:00Z",
      "updated_at": "2025-01-15T10:30:00Z"
    }
  ],
  "total": 1
}
```

#### Update Post

```bash
curl -X PUT http://localhost:8080/api/posts/1 \
  -H "Authorization: Bearer <token>" \
  -H "Content-Type: application/json" \
  -d '{
    "title": "Updated Title",
    "content": "Updated content"
  }'
```

#### Delete Post

```bash
curl -X DELETE http://localhost:8080/api/posts/1 \
  -H "Authorization: Bearer <token>"
```

Response: 204 No Content

## gRPC API Reference

### Services

- **AuthService**: Register, Login
- **BlogService**: CreatePost, GetPost, ListPosts, UpdatePost, DeletePost

### Using grpcurl

```bash
# List available services (reflection enabled)
grpcurl -plaintext localhost:50051 list

# Describe a service
grpcurl -plaintext localhost:50051 describe blog.AuthService

# Register user
grpcurl -plaintext \
  -d '{"username":"alice","email":"alice@test.com","password":"secret123"}' \
  localhost:50051 blog.AuthService/Register

# Login
grpcurl -plaintext \
  -d '{"username":"alice","password":"secret123"}' \
  localhost:50051 blog.AuthService/Login

# List posts
grpcurl -plaintext \
  -d '{"limit":10,"offset":0}' \
  localhost:50051 blog.BlogService/ListPosts

# Create post (with auth token)
grpcurl -plaintext \
  -H "authorization: Bearer <token>" \
  -d '{"title":"gRPC Post","content":"Created via gRPC"}' \
  localhost:50051 blog.BlogService/CreatePost
```

## Development

### Build Commands

```bash
# Build entire workspace
cargo build --workspace

# Build specific crate
cargo build -p blog-server
cargo build -p blog-cli

# Release build
cargo build --workspace --release
```

### Testing

```bash
# Run all tests
cargo test --workspace

# Run tests for specific crate
cargo test -p blog-server
cargo test -p blog-shared

# Run specific test
cargo test -p blog-server test_function_name

# Run with logging
RUST_LOG=debug cargo test --workspace

# Run integration tests
cargo test -p blog-server --test '*'
```

### Code Quality

```bash
# Format code
cargo fmt --all

# Check formatting
cargo fmt --all -- --check

# Run clippy linter
cargo clippy --workspace

# Check all targets
cargo clippy --workspace --all-targets
```

### Database Management

```bash
# Database location: blog.db (SQLite file)

# Initialize database (required before first build)
sqlx database create --database-url sqlite:blog.db
sqlx migrate run --source blog-server/migrations --database-url sqlite:blog.db

# Reset database
rm blog.db
sqlx database create --database-url sqlite:blog.db
sqlx migrate run --source blog-server/migrations --database-url sqlite:blog.db

# Or use DATABASE_URL from .env
source .env && sqlx migrate run --source blog-server/migrations
```

## Troubleshooting

### Compilation errors from sqlx (e.g., "error returned from database")
The database must exist with the correct schema before building due to compile-time query checking:
```bash
cargo install sqlx-cli --no-default-features --features sqlite
sqlx database create --database-url sqlite:blog.db
sqlx migrate run --source blog-server/migrations --database-url sqlite:blog.db
```

### "DATABASE_URL not set"
Copy `.env.example` to `.env` and configure values:
```bash
cp .env.example .env
```

### "JWT_SECRET must be set"
Add a secure JWT secret to your `.env` file (minimum 32 characters):
```env
JWT_SECRET=your-super-secret-key-at-least-32-characters-long
```

### Build errors after changing proto files
Delete target and rebuild:
```bash
cargo clean && cargo build --workspace
```

### WASM frontend can't connect to API
1. Ensure server is running on port 8080
2. Check browser console for CORS errors
3. Verify API URL in `blog-wasm/src/constants.rs`

### JWT "invalid token" errors
1. Token may have expired (24h lifetime)
2. Re-login to get a new token
3. Ensure JWT_SECRET matches between requests

### CLI "token not found"
Login first to save token:
```bash
cargo run -p blog-cli -- login --username <user> --password <pass>
```

## Project Status

| Phase | Status |
|-------|--------|
| Workspace Setup | Complete |
| Shared Types | Complete |
| Server Core | Complete |
| Auth API | Complete |
| Posts API | Complete |
| gRPC API | Complete |
| Client Library | Complete |
| CLI | Complete |
| WASM Frontend | Complete |
| Final Polish | In Progress |

See [doc/tasklist.md](doc/tasklist.md) for detailed progress.

## Documentation

- [vision.md](vision.md) - Technical vision and architecture
- [idea.md](idea.md) - Original project specifications
- [doc/tasklist.md](doc/tasklist.md) - Development task list
- [doc/workflow.md](doc/workflow.md) - Development workflow
- [conventions.md](conventions.md) - Code conventions

## License

MIT
