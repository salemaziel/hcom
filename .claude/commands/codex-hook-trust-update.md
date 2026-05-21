---
name: codex-hook-trust-update
description: Workflow command scaffold for codex-hook-trust-update in hcom.
allowed_tools: ["Bash", "Read", "Write", "Grep", "Glob"]
---

# /codex-hook-trust-update

Use this workflow when working on **codex-hook-trust-update** in `hcom`.

## Goal

Updates to Codex hook trust logic, including persisting trust state, bypassing trust, self-healing, and rejecting deprecated keys.

## Common Files

- `src/hooks/codex.rs`
- `src/tools/codex_preprocessing.rs`
- `src/tools/codex_args.rs`
- `src/commands/hooks.rs`

## Suggested Sequence

1. Understand the current state and failure mode before editing.
2. Make the smallest coherent change that satisfies the workflow goal.
3. Run the most relevant verification for touched files.
4. Summarize what changed and what still needs review.

## Typical Commit Signals

- Edit src/hooks/codex.rs to update trust logic or state handling.
- Edit src/tools/codex_preprocessing.rs and/or src/tools/codex_args.rs to adjust preprocessing or argument handling for Codex hooks.
- Optionally update src/commands/hooks.rs or related command files to align CLI behavior.

## Notes

- Treat this as a scaffold, not a hard-coded script.
- Update the command if the workflow evolves materially.