# Development Workflow

> Reference: [vision.md](../vision.md) for architecture, [tasklist.md](./tasklist.md) for phases.

---

## Workflow Cycle

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

---

## Step-by-Step Rules

### 1. PROPOSE

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

---

### 2. AGREE

User responds:
- ✅ **"Approved"** → Proceed to implement
- 🔄 **"Change X"** → Revise proposal
- ❌ **"Different approach"** → Start over

---

### 3. IMPLEMENT

After approval:
- Write code following [conventions.md](../conventions.md)
- Create/modify only agreed files
- Run `cargo fmt` to format code
- Run `cargo build` to verify compilation
- Run relevant tests

---

### 4. CONFIRM

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

---

### 5. UPDATE

After confirmation:

1. Update `doc/tasklist.md`:
   - Mark completed tasks: `- [ ]` → `- [x]`
   - Update progress report table
   - Change status: `⬜` → `✅`

2. Announce:
   ```markdown
   ## 📝 Progress Updated
   
   **Completed:** Phase X.Y
   **Next:** Phase X.Z
   
   Ready to proceed?
   ```

**Note:** Do NOT commit to git. User will review code and commit manually.

---

## Status Icons

| Icon | Meaning |
|------|---------|
| ⬜ | Not started |
| 🔄 | In progress |
| ✅ | Complete |
| ⚠️ | Blocked |
| ❌ | Failed/Rejected |

---

## Rules Summary

| Rule | Description |
|------|-------------|
| **No skipping** | Follow tasklist order strictly |
| **No surprise code** | Always propose first |
| **No auto-commit** | User reviews and commits manually |
| **Always test** | Verify before marking complete |
| **Always wait** | Get confirmation before proceeding |

---

## Quick Reference

```
1. Read current phase from tasklist.md
2. Propose solution with code snippets
3. Wait for "Approved"
4. Implement and test
5. Show results, wait for "Confirmed"
6. Update tasklist (no git commit)
7. Ask "Ready for next phase?"
8. Repeat
```

