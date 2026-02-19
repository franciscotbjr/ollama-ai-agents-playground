# Cache Project Context

I'll save the current session context for the ollama-oxide project by reading critical project files and creating a cache for future session continuity.

## Session Context (CRITICAL)

### ATTENTION
Before running the cache script, you MUST execute `/compact` command.

### `--task` argument
- Must contain the current/last task being developed in the project
- Example: "Rename feature 'create' to 'model' - refactoring complete, pending git commit"

### `--summary` argument
- **MUST have content equivalent to the result of `/compact` command execution**
The content of `--summary` is the result obtained from `/compact` previously executed.

### Session History
- Each save appends a new `SessionEntry` with `datetime`, `task`, and `summary` to the session history array
- The cache keeps the **last 10 sessions** ordered by date/time
- Older sessions are automatically rotated out

## Cache Files

The cache system uses two files in `~/.claude/ollama-oxide/`:
- **project.cache** - Single unified cache file for the project
- **project.cache.bkp** - Automatic backup created before each save (corruption safety)

## What Gets Cached

The cache script will automatically gather information from:

### Critical Project Files
- **spec/definition.md** - Complete project definition and implementation strategy
- **spec/api-analysis.md** - Detailed analysis of all 12 API endpoints
- **spec/apis/*.yaml** - Individual API endpoint specifications (12 files)
- **impl/*.md** - Implementation plan files
- **DEV_NOTES.md** - Development notes and architectural decisions
- **CHANGELOG.md** - Version history and changes
- **README.md** - Project overview and quick start
- **CONTRIBUTING.md** - Development guidelines and standards
- **ARCHITECTURE.md** - Architectural principles and module organization
- **DECISIONS.md** - Architectural decisions log
- **BLOCKERS.md** - Active blockers tracking
- **Cargo.toml** - Project configuration

### Extracted Information
From these files, the script extracts:
- Project name, version, repository, license (from Cargo.toml)
- Build system and tooling information
- List of all critical documentation files
- List of all API specification files
- List of all implementation plan files
- Build status (basic validation)
- Session tracking (count, timestamps, history of last 10 sessions)

### NOT Duplicated
The following information is **NOT** cached but read from files when needed:
- Implementation strategy and phases (read from definition.md)
- Current phase status and progress (read from definition.md)
- API endpoint details (read from api-analysis.md)
- Development notes and decisions (read from DEV_NOTES.md)
- Recent decisions (read from DECISIONS.md)
- Active blockers (read from BLOCKERS.md)
- Task lists and TODOs (read from relevant files)

This ensures the cache stays lean and source files remain the single source of truth.

## Usage

<function_calls>
<invoke name="Bash">
<parameter name="command">rust-script .claude/scripts/save_cache.rs</parameter>
<parameter name="description">Save session context to cache</parameter>
</invoke>
