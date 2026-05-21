```markdown
# hcom Development Patterns

> Auto-generated skill from repository analysis

## Overview
This skill provides a comprehensive guide to the development patterns, coding conventions, and key workflows in the `hcom` Rust codebase. It covers file organization, code style, and the main processes for updating trust logic, fixing relay features, and managing dependency versions. This guide is intended for contributors seeking to maintain consistency and efficiency when working with `hcom`.

## Coding Conventions

### File Naming
- **Style:** camelCase
- **Example:**  
  ```
  src/hooks/codex.rs
  src/tools/codexPreprocessing.rs
  ```

### Import Style
- **Style:** Relative imports
- **Example:**
  ```rust
  use crate::tools::codexPreprocessing;
  use super::codexArgs;
  ```

### Export Style
- **Style:** Named exports (Rust modules and functions are public as needed)
- **Example:**
  ```rust
  pub fn update_trust_state() { ... }
  pub mod relayWorker { ... }
  ```

## Workflows

### codex-hook-trust-update
**Trigger:** When Codex hook trust logic needs to be changed, fixed, or made more robust.  
**Command:** `/update-codex-hook-trust`

1. Edit `src/hooks/codex.rs` to update trust logic or state handling.
2. Edit `src/tools/codex_preprocessing.rs` and/or `src/tools/codex_args.rs` to adjust preprocessing or argument handling for Codex hooks.
3. Optionally update `src/commands/hooks.rs` or related command files to align CLI behavior.

**Example:**
```rust
// src/hooks/codex.rs
pub fn persist_trust_state(new_state: TrustState) {
    // ...implementation...
}
```

---

### relay-feature-fix-or-update
**Trigger:** When relay client/worker logic or related terminal/PTTY delivery needs fixing or improvement.  
**Command:** `/fix-relay`

1. Edit `src/relay/client.rs` and/or `src/relay/worker.rs` to fix or enhance relay logic.
2. Edit `src/commands/relay.rs` or related files for command-level changes.
3. Edit `src/terminal.rs` or `tests/test_pty_delivery.rs` for terminal/PTY integration or test coverage.

**Example:**
```rust
// src/relay/worker.rs
pub fn spawn_worker() {
    // Improved backoff logic
}
```

---

### dependency-version-bump
**Trigger:** When releasing a new version or updating dependency baselines.  
**Command:** `/bump-version`

1. Update `Cargo.toml` and `Cargo.lock` for Rust dependencies.
2. Update `pyproject.toml` for Python dependencies.
3. Optionally update `README.md`.

**Example:**
```toml
# Cargo.toml
[dependencies]
serde = "1.0.160"
```

## Testing Patterns

- **Framework:** Unknown (standard Rust test framework assumed)
- **File Pattern:** `*.test.*` (e.g., `tests/test_pty_delivery.rs`)
- **Example:**
  ```rust
  // tests/test_pty_delivery.rs
  #[test]
  fn test_pty_delivery_success() {
      // ...test code...
  }
  ```

## Commands

| Command                     | Purpose                                                    |
|-----------------------------|------------------------------------------------------------|
| /update-codex-hook-trust    | Update Codex hook trust logic and related CLI behavior     |
| /fix-relay                  | Fix or improve relay client/worker and PTY delivery logic  |
| /bump-version               | Bump Rust/Python dependency versions and update metadata   |
```
