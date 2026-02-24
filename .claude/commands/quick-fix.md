# Quick Fix

Execute a small, trivial fix without the full planning workflow.

## When to Use

Use `/quick-fix` for:
- Typo fixes in code or documentation
- Comment updates
- Simple variable/function renames (single file)
- Dependency version bumps
- Formatting changes
- Adding/removing imports
- Small bug fixes with obvious solutions

## When NOT to Use

Do NOT use `/quick-fix` for:
- New features or endpoints
- Architectural changes
- Multi-file refactors
- Changes requiring design decisions
- Anything that should be documented in `DECISIONS.md`

## Instructions

1. Read the user's request
2. Identify the file(s) to modify
3. Make the change directly (no planning phase, no `impl.md`)
4. Run `cargo check` to verify compilation
5. Run `cargo test` if the change touches code logic
6. Summarize what was changed

## Output Format

After completing the fix:

```
Quick Fix Applied:
- File: <path>
- Change: <brief description>
- Verified: cargo check passed
```

Do NOT:
- Update `DECISIONS.md`
- Update `BLOCKERS.md`
- Update `DEV_NOTES.md`
- Update `CHANGELOG.md`
- Create implementation plans
