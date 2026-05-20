---
name: codex-hook-trust-state-management
description: Workflow command scaffold for codex-hook-trust-state-management in hcom.
allowed_tools: ["Bash", "Read", "Write", "Grep", "Glob"]
---

# /codex-hook-trust-state-management

Use this workflow when working on **codex-hook-trust-state-management** in `hcom`.

## Goal

Manages the trust state and self-healing of Codex hooks, including persisting trust info, bypassing trust, and cleaning up deprecated keys.

## Common Files

- `src/hooks/codex.rs`
- `src/tools/codex_preprocessing.rs`
- `src/commands/hooks.rs`
- `src/commands/status.rs`

## Suggested Sequence

1. Understand the current state and failure mode before editing.
2. Make the smallest coherent change that satisfies the workflow goal.
3. Run the most relevant verification for touched files.
4. Summarize what changed and what still needs review.

## Typical Commit Signals

- Update src/hooks/codex.rs to adjust trust logic or cleanup deprecated keys.
- Modify src/tools/codex_preprocessing.rs to handle trust bypass or preprocessing changes.
- Optionally update src/commands/hooks.rs or src/commands/status.rs to align CLI commands with trust state changes.

## Notes

- Treat this as a scaffold, not a hard-coded script.
- Update the command if the workflow evolves materially.