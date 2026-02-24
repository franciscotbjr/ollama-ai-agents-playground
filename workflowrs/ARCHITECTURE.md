# workflowrs — Architecture

## Overview

`workflowrs` is a Rust library for composing and executing **agent workflows**. It is designed to orchestrate the agents defined in `ollama-ai-agents-playground`, chaining them into multi-step pipelines.

## Status

**Early stage.** Only the error type and module skeleton are defined. See [BLOCKERS.md](BLOCKERS.md) for open decisions.

## Intended Layered Architecture

```
Workflow API (public)
    ↓
Step / Transition (execution graph)
    ↓
Agent invocation (ollama-ai-agents-playground agents)
    ↓
Result aggregation / routing
```

## Module Structure

```
workflowrs/
├── src/
│   ├── lib.rs        — Public API re-exports
│   └── error.rs      — WorkflowError enum
└── Cargo.toml
```

## Design Principles

- **Trait-based API**: workflows and steps will be defined as traits, following the `Agent<P, T>` pattern from the parent crate.
- **One type per file**: each struct, enum, or trait lives in its own `.rs` file.
- **Error enum**: `WorkflowError` is the single error type for the crate.
- **No external I/O**: the library is pure orchestration — I/O is delegated to the agents.

## Dependencies

Currently no dependencies. Future integration with `ollama-ai-agents-playground` will be added when the API is stabilized.
