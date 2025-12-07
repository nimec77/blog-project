# Development Task List

## Progress Report

| Phase | Status | Progress | Notes |
|-------|--------|----------|-------|
| 1. Workspace Setup | ✅ Complete | 8/8 | All crates created |
| 2. Shared Types | ✅ Complete | 3/3 | DTOs with tests |
| 3. Server Core | ✅ Complete | 6/6 | Server starts, /health works |
| 4. Auth API | ✅ Complete | 4/4 | Register + Login work |
| 5. Posts API | ✅ Complete | 5/5 | Full CRUD works |
| 6. gRPC API | ✅ Complete | 5/5 | HTTP + gRPC parallel + reflection |
| 7. Client Library | ✅ Complete | 4/4 | HTTP + gRPC clients |
| 8. CLI | ✅ Complete | 4/4 | Full CLI with token persistence |
| 9. WASM Frontend | 🔄 In Progress | 5/6 | CORS enabled |
| 10. Final Polish | ⬜ Not Started | 0/3 | — |

**Legend:** ⬜ Not Started | 🔄 In Progress | ✅ Complete | ⚠️ Blocked

---

## Phase 1: Workspace Setup

**Goal:** Project compiles, basic structure ready.

- [x] 1.1 Create workspace `Cargo.toml` with 5 members
- [x] 1.2 Create `blog-shared` crate (lib)
- [x] 1.3 Create `blog-server` crate (bin)
- [x] 1.4 Create `blog-client` crate (lib)
- [x] 1.5 Create `blog-cli` crate (bin)
- [x] 1.6 Create `blog-wasm` crate (lib, cdylib)
- [x] 1.7 Configure `.gitignore`
- [x] 1.8 Add workspace dependencies

**✓ Test:** `cargo build --workspace` succeeds ✅

---

## Phase 2: Shared Types

**Goal:** Common DTOs available for all crates.

- [x] 2.1 Define `UserDto`, `PostDto`, `AuthResponse`
- [x] 2.2 Define request types (`RegisterRequest`, `LoginRequest`, etc.)
- [x] 2.3 Add serde derives

**✓ Test:** `cargo test -p blog-shared` ✅

---

## Phase 3: Server Core

**Goal:** Server starts, DB ready, no endpoints yet.

- [x] 3.1 Create `infrastructure/config.rs` (env loading)
- [x] 3.2 Create `infrastructure/database.rs` (SQLite pool)
- [x] 3.3 Create migrations (`users`, `posts` tables)
- [x] 3.4 Create domain entities (`User`, `Post`)
- [x] 3.5 Create `domain/error.rs` (`AppError`)
- [x] 3.6 Setup `main.rs` with actix-web server (empty routes)

**✓ Test:** Server starts on port 8080, `GET /health` returns 200 ✅

---

## Phase 4: Auth API

**Goal:** User registration and login work.

- [x] 4.1 Create `data/user_repository.rs`
- [x] 4.2 Create `infrastructure/jwt.rs`
- [x] 4.3 Create `application/auth_service.rs`
- [x] 4.4 Create `presentation/http_handlers.rs` (auth endpoints)

**✓ Test:** 
```bash
curl -X POST http://localhost:8080/api/auth/register \
  -H "Content-Type: application/json" \
  -d '{"username":"test","email":"test@test.com","password":"secret123"}'
# Returns: {"token":"...", "user":{...}}

curl -X POST http://localhost:8080/api/auth/login \
  -H "Content-Type: application/json" \
  -d '{"username":"test","password":"secret123"}'
# Returns: {"token":"...", "user":{...}}
```
✅

---

## Phase 5: Posts API

**Goal:** Full CRUD for posts via HTTP.

- [x] 5.1 Create `data/post_repository.rs`
- [x] 5.2 Create `application/blog_service.rs`
- [x] 5.3 Create `presentation/middleware.rs` (JWT auth)
- [x] 5.4 Add posts endpoints to `http_handlers.rs`
- [x] 5.5 Wire up routes with auth middleware

**✓ Test:**
```bash
# Create post (with token)
curl -X POST http://localhost:8080/api/posts \
  -H "Authorization: Bearer <token>" \
  -H "Content-Type: application/json" \
  -d '{"title":"Hello","content":"World"}'

# List posts (public)
curl http://localhost:8080/api/posts

# Get single post
curl http://localhost:8080/api/posts/1

# Update post (author only)
curl -X PUT http://localhost:8080/api/posts/1 \
  -H "Authorization: Bearer <token>" \
  -d '{"title":"Updated"}'

# Delete post (author only)
curl -X DELETE http://localhost:8080/api/posts/1 \
  -H "Authorization: Bearer <token>"
```
✅

---

## Phase 6: gRPC API

**Goal:** All operations available via gRPC.

- [x] 6.1 Create `proto/blog.proto`
- [x] 6.2 Configure `build.rs`
- [x] 6.3 Create `presentation/grpc_service.rs`
- [x] 6.4 Add gRPC server to `main.rs` (parallel with HTTP)
- [x] 6.5 Add gRPC server reflection (`tonic-reflection`)

**✓ Test:** Use `grpcurl` or blog-cli with `--grpc` flag
```bash
# With reflection (no proto file needed)
grpcurl -plaintext localhost:50051 list
# Returns: blog.AuthService, blog.BlogService, grpc.reflection.v1.ServerReflection

grpcurl -plaintext localhost:50051 describe blog.AuthService
# Returns service definition

grpcurl -plaintext \
  -d '{"username":"grpcuser","email":"grpc@test.com","password":"secret123"}' \
  localhost:50051 blog.AuthService/Register
# Returns: {"token":"...", "user":{...}}
```
✅

---

## Phase 7: Client Library

**Goal:** Unified client for HTTP and gRPC.

- [x] 7.1 Create `http_client.rs`
- [x] 7.2 Create `grpc_client.rs`
- [x] 7.3 Create `error.rs` (`ClientError`)
- [x] 7.4 Create unified `BlogClient` in `lib.rs`

**✓ Test:** `cargo test -p blog-client` (unit tests with mocks)

---

## Phase 8: CLI

**Goal:** Full CLI for all operations.

- [x] 8.1 Setup clap with commands and global flags
- [x] 8.2 Implement auth commands (register, login)
- [x] 8.3 Implement post commands (create, get, update, delete, list)
- [x] 8.4 Add token persistence (`.blog_token` file)

**✓ Test:**
```bash
cargo run -p blog-cli -- register --username cli_user --email cli@test.com --password secret
cargo run -p blog-cli -- login --username cli_user --password secret
cargo run -p blog-cli -- create --title "CLI Post" --content "From CLI"
cargo run -p blog-cli -- list
cargo run -p blog-cli -- --grpc list  # Test gRPC transport
```

---

## Phase 9: WASM Frontend

**Goal:** Browser UI for all operations.

- [x] 9.1 Setup Yew app structure
- [x] 9.2 Create `api.rs` (HTTP client)
- [x] 9.3 Create auth components (login, register forms)
- [ ] 9.4 Create post components (list, form, card)
- [x] 9.5 Add localStorage token handling
- [x] 9.6 Add CORS to blog-server for WASM frontend

**✓ Test:**
```bash
cd blog-wasm
trunk serve
# Open http://localhost:8081
# Register, login, create/edit/delete posts in browser
```

---

## Phase 10: Final Polish

**Goal:** Production-ready quality.

- [ ] 10.1 Add integration tests
- [ ] 10.2 Write README.md with setup instructions
- [ ] 10.3 Final code review and cleanup

**✓ Test:** Full E2E flow works via all clients (curl, CLI, browser)

---

## Quick Commands

```bash
# Build all
cargo build --workspace

# Run server
cargo run -p blog-server

# Run CLI
cargo run -p blog-cli -- <command>

# Run WASM dev server
cd blog-wasm && trunk serve

# Run all tests
cargo test --workspace
```

