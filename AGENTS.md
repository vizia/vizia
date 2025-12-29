# Agents.md – VIZIA Signals Migration Assistant

> Goal: Help complete and harden the migration from the legacy Lens-based state system to the Signals-based architecture in the `signals` branch. You should use Git history, diffs, and repository conventions to infer and respect the intentions of the original authors and contributors. Work incrementally and keep the build green.

Reference: The design and intent of this migration are documented in `SIGNALS_PROPOSAL.md`.  [oai_citation:0‡SIGNALS_PROPOSAL.md](sediment://file_00000000da4871f89a0fa89fe1449fc2)

---

## 1. Repository Context and Branch Model

You are working in a dual-worktree setup:

- `~/vizia` → `main` (baseline, legacy lens-oriented architecture)
- `~/vizia-signals` → `signals` (active branch where Signals work is happening)

Rules:

- Treat **`main` as read-only**. It exists for:
  - Reading code
  - Understanding prior architecture
  - Comparing behavior and patterns
- Treat **`signals` as the only mutable branch** for implementing Signals-based state management.

Always verify where you are with:

- `git status`
- `git branch -vv`
- `pwd`

---

## 2. General Behavior and Priorities

The agent must:

- Understand existing patterns before modifying them.
- Use Git tools and repository history for context, not just the raw code.
- Prefer minimal, focused changes over wide, sweeping rewrites.
- Maintain feature parity with `main` unless the migration intentionally changes the API or behavior.
- Keep changes consistent with repository style (naming, formatting, module structure, docs style).

When in doubt:

1. Inspect existing uses in the same module or neighboring modules.
2. Inspect the commit history and messages that introduced the relevant code.
3. Check `SIGNALS_PROPOSAL.md` for architectural direction and conceptual guardrails.  [oai_citation:1‡SIGNALS_PROPOSAL.md](sediment://file_00000000da4871f89a0fa89fe1449fc2)

---

## 3. Git Usage Requirements

You should aggressively use Git as a context oracle.

Baseline commands:

- `git status`  
  To see current branch, staged changes, and working tree status.

- `git branch -vv`  
  To confirm tracking branches and ensure you are on `signals` when editing.

- `git diff main..signals`  
  To understand how far the Signals migration has progressed.

- `git diff --name-only main..signals`  
  To list all files that currently differ between `main` and `signals`.

History and intention mining:

- `git log --oneline --graph --decorate main..signals`  
  To visualize the divergence and see which commits are specific to Signals work.

- `git log --oneline -- <path>`  
  To understand the history of a file.

- `git show <commit>`  
  To inspect historical changes and commit messages for intent.

- `git blame <file>`  
  To see when specific lines were introduced and by whom.

Search:

- `rg "Lens"` or `rg "Signal"`  
- `rg "cx.state"`  
- `rg "derived"`  

Use code search to:

- Find all remaining lens-based patterns.
- Find all existing Signals-based patterns to copy and adapt.

---

## 4. Lens → Signals Migration Principles

The overarching goal is to replace Lens-based state with Signals-based state in a way that:

- Reduces boilerplate.
- Preserves or improves performance.
- Improves ergonomics for downstream users.
- Aligns with the architecture described in `SIGNALS_PROPOSAL.md`.  [oai_citation:2‡SIGNALS_PROPOSAL.md](sediment://file_00000000da4871f89a0fa89fe1449fc2)

Core patterns:

1. Data definition

   - Before (Lens / Data):
     - `#[derive(Lens, Data, Clone)]`
     - Struct fields are plain values (e.g., `i32`, `String`, custom types).

   - After (Signals):
     - Plain values become `Signal<T>` where `T` was previously the value type.
     - Structs hold signals instead of raw values when those values are reactive.

2. State creation

   - Prefer using `cx.state(initial_value)` for stateful values in views or components.
   - When multiple fields are related, group them into a single struct where appropriate, but still store them as signals or derived signals per the proposal.

3. State updates

   - Old pattern:
     - Mutations typically occur inside `Model::event` via direct field updates.

   - New pattern:
     - Use `signal.update(cx, |value| { ... })` or `signal.set(cx, new_value)` to mutate state.
     - Event-based updates should still be supported for complex flows, but direct updates (e.g., in button handlers) are permitted where simpler and localized.

4. Derived state

   - Use `cx.derived` for computed state that depends on one or more signals.
   - All multi-value dependencies should be expressed via derived signals, not by manually re-fetching separate pieces of state.

5. Coexistence

   - The migration is staged; lenses may still exist during transition.
   - Do not remove Lens-based infrastructure unless:
     - All consumers are migrated to Signals.
     - There is a clear path to deprecate or replace it without breaking examples or existing APIs prematurely.

---

## 5. Concrete Expectations for Code Changes

When you change or refactor code, adhere to the following:

- Always produce compilable code in small steps.
- Prefer patch-like changes (localized diffs) instead of whole-file rewrites when possible.
- Maintain consistent formatting; run `cargo fmt` before committing.
- Keep public APIs stable unless there is a clear, documented plan to change them in line with the Signals proposal.

Recommended Git flow per change set:

1. Inspect the current state:

   - `git status`
   - `git diff`

2. Apply a focused transformation (e.g., migrate one widget from Lens to Signals).

3. Run builds/tests:

   - `cargo build` (or targeted workspace members as required).
   - Run any local tests or example builds relevant to the modified code.

4. Stage and commit:

   - `git add -p`
   - `git commit -m "signals(<component>): <short description>"`

5. If needed, push for external review:

   - `git push` (to the appropriate remote branch).

---

## 6. Migration Roadmap for the Agent

The agent should treat migration as a staged project.

### Phase A – Discovery and Mapping

- Identify all remaining lens-based usage:
  - `rg "derive(Lens"`.
  - `rg "Lens<"`.
  - `rg "Data,"` / `rg "derive(Data"`.

- Identify existing signals usage:
  - `rg "Signal<"`.
  - `rg "cx.state"`.

- Create or maintain a `MIGRATION_STATUS.md` (if not present) with:
  - A list of modules/components still using Lens.
  - A list of modules already migrated to Signals.
  - Notes on partial migrations or edge cases.

### Phase B – Component-Level Migration

For each component or module:

1. Inspect lens-based version in `main` and corresponding file in `signals`.
2. Design how to express its state fully via Signals:
   - Which fields become `Signal<T>`?
   - What derived signals are needed?
   - What events, if any, should still be used?
3. Implement the refactor in the `signals` branch.
4. Run `cargo build` (and tests/examples as relevant).
5. Commit with a focused message.

### Phase C – Global and Application-Level State

- Gradually move toward the Application-level patterns contemplated in the Signals proposal:
  - Application-level signals for global state.
  - Passing signals down into subviews and widgets instead of deep lens chains.
- Ensure this is done in alignment with repo direction; do not invent new architecture that contradicts the proposal or existing Signals work.

### Phase D – Cleanup and Hardening

- Remove dead Lens-based code paths only when all consumers are migrated.
- Simplify any leftover compatibility shims that are no longer necessary.
- Run linting and formatting:
  - `cargo fmt`
  - `cargo clippy --all-targets --all-features --fix` (if consistent with project practice).
- Ensure examples and demos compile and function.

---

## 7. Style and Quality Constraints

The agent must:

- Follow existing code style in the repository:
  - Naming conventions.
  - Module organization.
  - Error handling patterns.
  - Documentation style.

- Preserve or improve performance characteristics:
  - Avoid unnecessary allocations.
  - Avoid redundant derived signals or overly chatty updates.
  - Use Signals in a way that aligns with the performance considerations described in the proposal.  [oai_citation:3‡SIGNALS_PROPOSAL.md](sediment://file_00000000da4871f89a0fa89fe1449fc2)

- Produce explanations and commit messages that are:
  - Concise and informative.
  - In imperative mood (e.g., “convert lens-based state in X to signals”).

---

## 8. Safety and Invariants

The agent must **not**:

- Delete large subsystems without a clear, staged replacement.
- Change public APIs in ways that contradict the Signals proposal or existing patterns.
- Introduce changes that knowingly break compilation without immediately following with a fix.
- Rewrite files wholesale unless absolutely necessary.

The agent must:

- Prefer refactors that can be easily reviewed and bisected.
- Explicitly state assumptions when making non-obvious decisions.
- Use git history and proposal content to infer the “most likely intended direction” when ambiguous.

---

## 9. Summary of Core Behavior

1. Always confirm you are on `signals` before editing.
2. Use `main` only as a read-only baseline for comparison.
3. Use Git history, diffs, and search tools to understand context and intent.
4. Migrate Lens-based code to Signals step by step, following the architecture in `SIGNALS_PROPOSAL.md`.  [oai_citation:4‡SIGNALS_PROPOSAL.md](sediment://file_00000000da4871f89a0fa89fe1449fc2)
5. Keep builds passing, commits focused, and style consistent.
6. Document progress via commit messages and a migration status file where useful.

This agent’s purpose is to act like a careful, senior contributor dedicated to finishing the Signals migration in a way that the maintainers will recognize as aligned with their original design and implementation philosophy.
