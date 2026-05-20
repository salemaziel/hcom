```markdown
# hcom Development Patterns

> Auto-generated skill from repository analysis

## Overview

This skill teaches you the core development patterns and workflows used in the `hcom` Rust codebase. You'll learn the project's coding conventions, how to manage trust state for Codex hooks, enhance relay features, handle dependency releases, and follow the repository's testing approach. This guide is ideal for contributors aiming for consistency and efficiency in the `hcom` project.

## Coding Conventions

- **File Naming:**  
  All files use `snake_case` for names.  
  *Example:*  
  ```
  src/hooks/codex.rs
  tests/test_pty_delivery.rs
  ```

- **Import Style:**  
  Use relative imports within modules.  
  *Example:*  
  ```rust
  use super::trust_state;
  use crate::relay::worker;
  ```

- **Export Style:**  
  Prefer named exports for functions, structs, and modules.  
  *Example:*  
  ```rust
  pub fn update_trust_state(...) { ... }
  pub struct RelayClient { ... }
  ```

- **Commit Patterns:**  
  - Mixed commit types, often prefixed with `fix` or `release`
  - Commit messages are concise (average 47 characters)

## Workflows

### Codex Hook Trust State Management
**Trigger:** When updating or fixing Codex hook trust logic or related launch/cleanup behavior.  
**Command:** `/update-codex-hook-trust`

1. Update `src/hooks/codex.rs` to adjust trust logic or clean up deprecated keys.
2. Modify `src/tools/codex_preprocessing.rs` for trust bypass or preprocessing changes.
3. Optionally update `src/commands/hooks.rs` or `src/commands/status.rs` to align CLI commands with trust state changes.
4. Test changes to ensure trust state is correctly managed and deprecated keys are handled.

*Example:*
```rust
// In src/hooks/codex.rs
pub fn cleanup_deprecated_trust_keys() {
    // Logic to remove old trust keys
}
```

### Relay Feature Fix or Enhancement
**Trigger:** When relay-related bugs are fixed or new relay features are added.  
**Command:** `/fix-relay`

1. Update `src/relay/client.rs` or `src/relay/worker.rs` for relay logic changes (e.g., status, worker spawning, reconnect logic).
2. Modify `src/commands/relay.rs` for CLI relay commands.
3. Update or add tests in `tests/` (e.g., `test_pty_delivery.rs`) to cover relay changes.
4. Verify relay functionality via CLI and automated tests.

*Example:*
```rust
// In src/relay/worker.rs
pub fn spawn_worker() {
    // Worker spawning logic
}
```

### Dependency Version Bump Release
**Trigger:** When releasing a new version or updating Rust/Python dependencies.  
**Command:** `/bump-version`

1. Update `Cargo.toml` and `Cargo.lock` for Rust dependency changes.
2. Update `pyproject.toml` for Python dependency changes (if applicable).
3. Update `README.md` if relevant.
4. Commit with a `release` or version bump message.

*Example:*
```toml
# In Cargo.toml
[dependencies]
serde = "1.0.160"
```

## Testing Patterns

- **Test Framework:** Unknown (standard Rust test framework assumed)
- **Test File Pattern:** Files matching `*.test.*` (e.g., `test_pty_delivery.rs`)
- **Test Example:**
  ```rust
  // In tests/test_pty_delivery.rs
  #[test]
  fn test_relay_delivery() {
      // Test logic here
  }
  ```

## Commands

| Command                     | Purpose                                                        |
|-----------------------------|----------------------------------------------------------------|
| /update-codex-hook-trust    | Update or fix Codex hook trust state management                |
| /fix-relay                  | Implement relay feature fixes or enhancements                  |
| /bump-version               | Bump dependency versions and release a new project version     |
```
