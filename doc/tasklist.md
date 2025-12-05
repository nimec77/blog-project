# Development Task List

## Progress Report

| Phase | Status | Progress | Notes |
|-------|--------|----------|-------|
| 1. Workspace Setup | ✅ Complete | 8/8 | All crates created |
| 2. Shared Types | ⬜ Not Started | 0/3 | — |
| 3. Server Core | ⬜ Not Started | 0/6 | — |
| 4. Auth API | ⬜ Not Started | 0/4 | — |
| 5. Posts API | ⬜ Not Started | 0/5 | — |
| 6. gRPC API | ⬜ Not Started | 0/4 | — |
| 7. Client Library | ⬜ Not Started | 0/4 | — |
| 8. CLI | ⬜ Not Started | 0/4 | — |
| 9. WASM Frontend | ⬜ Not Started | 0/5 | — |
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

- [ ] 2.1 Define `UserDto`, `PostDto`, `AuthResponse`
- [ ] 2.2 Define request types (`RegisterRequest`, `LoginRequest`, etc.)
- [ ] 2.3 Add serde derives

**✓ Test:** `cargo test -p blog-shared`

---

## Phase 3: Server Core

**Goal:** Server starts, DB ready, no endpoints yet.

- [ ] 3.1 Create `infrastructure/config.rs` (env loading)
- [ ] 3.2 Create `infrastructure/database.rs` (SQLite pool)
- [ ] 3.3 Create migrations (`users`, `posts` tables)
- [ ] 3.4 Create domain entities (`User`, `Post`)
- [ ] 3.5 Create `domain/error.rs` (`AppError`)
- [ ] 3.6 Setup `main.rs` with actix-web server (empty routes)

**✓ Test:** Server starts on port 8080, `GET /health` returns 200

---

## Phase 4: Auth API

**Goal:** User registration and login work.

- [ ] 4.1 Create `data/user_repository.rs`
- [ ] 4.2 Create `infrastructure/jwt.rs`
- [ ] 4.3 Create `application/auth_service.rs`
- [ ] 4.4 Create `presentation/http_handlers.rs` (auth endpoints)

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

---

## Phase 5: Posts API

**Goal:** Full CRUD for posts via HTTP.

- [ ] 5.1 Create `data/post_repository.rs`
- [ ] 5.2 Create `application/blog_service.rs`
- [ ] 5.3 Create `presentation/middleware.rs` (JWT auth)
- [ ] 5.4 Add posts endpoints to `http_handlers.rs`
- [ ] 5.5 Wire up routes with auth middleware

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

---

## Phase 6: gRPC API

**Goal:** All operations available via gRPC.

- [ ] 6.1 Create `proto/blog.proto`
- [ ] 6.2 Configure `build.rs`
- [ ] 6.3 Create `presentation/grpc_service.rs`
- [ ] 6.4 Add gRPC server to `main.rs` (parallel with HTTP)

**✓ Test:** Use `grpcurl` or blog-cli with `--grpc` flag

---

## Phase 7: Client Library

**Goal:** Unified client for HTTP and gRPC.

- [ ] 7.1 Create `http_client.rs`
- [ ] 7.2 Create `grpc_client.rs`
- [ ] 7.3 Create `error.rs` (`ClientError`)
- [ ] 7.4 Create unified `BlogClient` in `lib.rs`

**✓ Test:** `cargo test -p blog-client` (unit tests with mocks)

---

## Phase 8: CLI

**Goal:** Full CLI for all operations.

- [ ] 8.1 Setup clap with commands and global flags
- [ ] 8.2 Implement auth commands (register, login)
- [ ] 8.3 Implement post commands (create, get, update, delete, list)
- [ ] 8.4 Add token persistence (`.blog_token` file)

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

- [ ] 9.1 Setup Yew app structure
- [ ] 9.2 Create `api.rs` (HTTP client)
- [ ] 9.3 Create auth components (login, register forms)
- [ ] 9.4 Create post components (list, form, card)
- [ ] 9.5 Add localStorage token handling

**✓ Test:**
```bash
cd blog-wasm
trunk serve
# Open http://localhost:8080
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

