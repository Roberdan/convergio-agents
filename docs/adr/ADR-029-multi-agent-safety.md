# ADR-029: Multi-Agent Safety Infrastructure

**Status:** Accepted
**Date:** 2026-04-06
**Context:** Convergio runs multiple autonomous agents in parallel. Without safety guardrails, agents can conflict, waste resources, or produce inconsistent state.

## Decision

We implement six complementary safety mechanisms:

### 1. Worktree Reaper

Each agent works in an isolated git worktree under `.worktrees/`. The reaper (convergio-build) periodically garbage-collects stale worktrees whose branches have been merged or abandoned, preventing disk bloat on long-running hosts.

### 2. File Locks

Agents acquire advisory file locks before editing shared resources (Cargo.lock, main.rs). This prevents concurrent edits from producing corrupted merges. The lock is scoped per-file and released on commit or timeout.

### 3. PR Dependency Enforcement

The daemon tracks `depends_on` relationships between tasks and waves. A PR cannot be merged until all upstream PRs are merged first (WaveSequenceGate). This prevents merge-order bugs where a later wave lands before its prerequisite.

### 4. Per-Project Validation Scope

Thor (the validator agent) reviews work scoped to the specific project and plan. Validation checks are isolated: a Thor review for project A cannot accidentally approve or block project B. The ValidatorGate enforces this at the API level.

### 5. Drift Detection

The monitor periodically compares expected state (plan DB) against actual state (git branches, open PRs, running processes). Orphaned worktrees, zombie agent processes, and stale PRs are flagged for cleanup. This catches work that started but was never completed or submitted.

### 6. Role Dispatcher

POST `/api/org/:id/dispatch` assigns tasks to agents based on capability matching and current workload. The dispatcher queries the agent catalog for agents whose `capabilities_json` intersects the task requirements, then picks the agent with the lowest number of in-progress tasks. This ensures the right agent handles the right task and prevents overloading a single agent.

## Consequences

- Agents can safely work in parallel without manual coordination.
- Disk usage is bounded by the reaper.
- Merge order is guaranteed by PR dependencies.
- Validation is scoped and cannot leak across projects.
- Orphaned work is detected and surfaced.
- Task assignment is automated and load-balanced.
