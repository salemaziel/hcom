---
name: relay-feature-fix-or-enhancement
description: Workflow command scaffold for relay-feature-fix-or-enhancement in hcom.
allowed_tools: ["Bash", "Read", "Write", "Grep", "Glob"]
---

# /relay-feature-fix-or-enhancement

Use this workflow when working on **relay-feature-fix-or-enhancement** in `hcom`.

## Goal

Implements fixes or enhancements to relay functionality, including status, worker spawning, and reconnect logic.

## Common Files

- `src/relay/client.rs`
- `src/relay/worker.rs`
- `src/commands/relay.rs`
- `tests/test_pty_delivery.rs`

## Suggested Sequence

1. Understand the current state and failure mode before editing.
2. Make the smallest coherent change that satisfies the workflow goal.
3. Run the most relevant verification for touched files.
4. Summarize what changed and what still needs review.

## Typical Commit Signals

- Update src/relay/client.rs or src/relay/worker.rs for relay logic changes.
- Modify src/commands/relay.rs for CLI relay commands.
- Update or add tests in tests/ (e.g., test_pty_delivery.rs) to cover relay changes.

## Notes

- Treat this as a scaffold, not a hard-coded script.
- Update the command if the workflow evolves materially.