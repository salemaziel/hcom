---
name: dependency-version-bump-release
description: Workflow command scaffold for dependency-version-bump-release in hcom.
allowed_tools: ["Bash", "Read", "Write", "Grep", "Glob"]
---

# /dependency-version-bump-release

Use this workflow when working on **dependency-version-bump-release** in `hcom`.

## Goal

Updates dependency versions and bumps project version for a new release.

## Common Files

- `Cargo.toml`
- `Cargo.lock`
- `pyproject.toml`
- `README.md`

## Suggested Sequence

1. Understand the current state and failure mode before editing.
2. Make the smallest coherent change that satisfies the workflow goal.
3. Run the most relevant verification for touched files.
4. Summarize what changed and what still needs review.

## Typical Commit Signals

- Update Cargo.toml and Cargo.lock for Rust dependency changes.
- Update pyproject.toml for Python dependency changes.
- Update README.md if relevant.
- Commit with a release or version bump message.

## Notes

- Treat this as a scaffold, not a hard-coded script.
- Update the command if the workflow evolves materially.