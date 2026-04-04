# Ralph Agent Instructions — Kvlar

You are an autonomous coding agent working on the Kvlar SDK, a runtime security layer for AI agents.

## Project Context

- **Language**: Rust (2024 edition)
- **Workspace**: 4 crates — kvlar-core, kvlar-proxy, kvlar-audit, kvlar-cli
- **Architecture**: kvlar-core is pure (no I/O, no async). Proxy handles transport. CLI wires everything together.
- **Tests**: `cargo test --workspace` (must pass before committing)
- **Lint**: `cargo clippy --workspace -- -D warnings` (must pass before committing)
- **Format**: `cargo fmt --all`

## Your Task

1. Read the PRD at `scripts/ralph/prd.json`
2. Read the progress log at `scripts/ralph/progress.txt` (check Codebase Patterns section first)
3. Check you're on the correct branch from PRD `branchName`. If not, create it from main.
4. Pick the **highest priority** user story where `passes: false`
5. Implement that single user story
6. Run quality checks:
   ```bash
   cargo build --workspace
   cargo test --workspace
   cargo clippy --workspace -- -D warnings
   cargo fmt --all -- --check
   ```
7. If checks pass, commit ALL changes with message: `feat: [Story ID] - [Story Title]`
8. Update the PRD to set `passes: true` for the completed story
9. Append your progress to `scripts/ralph/progress.txt`

## Progress Report Format

APPEND to progress.txt (never replace, always append):
```
## [Date/Time] - [Story ID]
- What was implemented
- Files changed
- **Learnings for future iterations:**
  - Patterns discovered
  - Gotchas encountered
  - Useful context
---
```

## Codebase Rules

- **kvlar-core MUST remain pure** — no I/O, no async, no network. Zero external dependencies beyond serde/regex.
- **Fail-closed default** — deny if no rule matches (ADR-001)
- **MCP denials** use tool results with `isError: true` (not JSON-RPC errors)
- **All stderr** for watcher/reload/diagnostic messages (stdio transport safety)
- **Publish order**: kvlar-audit → kvlar-core → kvlar-proxy → kvlar-cli (dependency chain)
- **Binary name**: `kvlar` (set via `[[bin]]` in kvlar-cli/Cargo.toml)

## Quality Requirements

- ALL commits must pass: build, test, clippy, fmt
- Do NOT commit broken code
- Keep changes focused and minimal
- Follow existing code patterns
- Add tests for new functionality

## Stop Condition

After completing a user story, check if ALL stories have `passes: true`.

If ALL stories are complete and passing, reply with:
<promise>COMPLETE</promise>

If there are still stories with `passes: false`, end your response normally (another iteration will pick up the next story).

## Important

- Work on ONE story per iteration
- Commit frequently
- Keep CI green
- Read the Codebase Patterns section in progress.txt before starting
