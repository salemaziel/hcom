---
name: dependency-version-bump
description: Workflow command scaffold for dependency-version-bump in hcom.
allowed_tools: ["Bash", "Read", "Write", "Grep", "Glob"]
---

# /dependency-version-bump

Use this workflow when working on **dependency-version-bump** in `hcom`.

## Goal

Bumping Rust crate or Python package versions and updating related metadata.

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

- Update Cargo.toml and Cargo.lock for Rust dependencies.
- Update pyproject.toml for Python dependencies.
- Optionally update README.md.

## Notes

- Treat this as a scaffold, not a hard-coded script.
- Update the command if the workflow evolves materially.