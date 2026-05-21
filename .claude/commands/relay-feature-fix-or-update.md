---
name: relay-feature-fix-or-update
description: Workflow command scaffold for relay-feature-fix-or-update in hcom.
allowed_tools: ["Bash", "Read", "Write", "Grep", "Glob"]
---

# /relay-feature-fix-or-update

Use this workflow when working on **relay-feature-fix-or-update** in `hcom`.

## Goal

Bugfixes or improvements to relay-related functionality, such as worker spawn, reconnect backoff, status, or PTY delivery.

## Common Files

- `src/relay/client.rs`
- `src/relay/worker.rs`
- `src/commands/relay.rs`
- `src/terminal.rs`
- `tests/test_pty_delivery.rs`

## Suggested Sequence

1. Understand the current state and failure mode before editing.
2. Make the smallest coherent change that satisfies the workflow goal.
3. Run the most relevant verification for touched files.
4. Summarize what changed and what still needs review.

## Typical Commit Signals

- Edit src/relay/client.rs and/or src/relay/worker.rs to fix or enhance relay logic.
- Edit src/commands/relay.rs or related files for command-level changes.
- Edit src/terminal.rs or tests/test_pty_delivery.rs for terminal/PTY integration or test coverage.

## Notes

- Treat this as a scaffold, not a hard-coded script.
- Update the command if the workflow evolves materially.