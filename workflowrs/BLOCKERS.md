# workflowrs — Blockers & Open Decisions

## WF-001 — Workflow API Design

**Status**: Open
**Description**: The public API for defining and executing workflows is not yet designed.
**Options**:
- A) Builder pattern: `Workflow::builder().step(agent).step(agent).build()`
- B) Declarative DSL via TOML/JSON configuration
- C) Trait-based: `impl Workflow for MyFlow { fn steps(...) }`

**Impact**: Blocks all feature work in this crate.

---

## WF-002 — Step Execution Model

**Status**: Open
**Description**: How steps communicate results between them is undefined.
**Options**:
- A) Each step receives the previous step's output (chain of responsibility)
- B) Shared context object passed through all steps
- C) Event-driven: each step emits events consumed by the next

**Impact**: Blocks `Step` type definition.

---

## WF-003 — Integration with ollama-ai-agents-playground

**Status**: Open
**Description**: `workflowrs` needs to depend on the parent crate's agent types, but the parent crate currently depends on `workflowrs`. This circular dependency must be resolved.
**Options**:
- A) Extract shared types (AgentParam, AgentResult, Agent trait) into a third `agents-core` crate
- B) Invert the dependency: `ollama-ai-agents-playground` depends on `workflowrs`, not the other way around
- C) Use trait objects / dynamic dispatch to avoid direct type dependencies

**Impact**: Blocks any real implementation.

---

## WF-004 — Async Execution

**Status**: Open
**Description**: Whether workflows execute steps sequentially, in parallel, or both.
**Options**:
- A) Sequential only (simplest)
- B) Parallel steps with `tokio::join!` for independent branches
- C) DAG-based execution graph with explicit dependencies

**Impact**: Affects the step execution model (WF-002).
