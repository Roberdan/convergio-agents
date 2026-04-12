# ADR-044: Role-Specific Agent Rules (AGENTS.md Split)

## Status: Accepted

## Context

AGENTS.md was 329 lines (~4400 tokens), loaded by every spawned agent regardless of role.
An executor reading planning rules wastes tokens. A validator reading delegation protocol
wastes tokens. At ~10-12K tokens boot tax per spawn, this was the single largest source
of avoidable token burn.

## Decision

Split AGENTS.md into role-specific files under `agents/`:

| File | Lines | Tokens | Who reads it |
|------|-------|--------|-------------|
| `agents/common.md` | 52 | ~650 | All agents |
| `agents/executor.md` | 44 | ~550 | Task executors |
| `agents/planner.md` | 42 | ~525 | Planners |
| `agents/validator.md` | 28 | ~350 | Thor validators |

An executor loads `common.md + executor.md` = ~1200 tokens vs 4400 before (**-73%**).

AGENTS.md reduced to a 9-line redirect pointing to `agents/`.

### Entry points updated

All three agent entry points now point to `agents/`:

- `.claude/CLAUDE.md` — Claude Code interactive sessions
- `.github/copilot-instructions.md` — Copilot agents
- `DELEGATION_HEADER` in `harness.rs` — all spawned agents (both backends)

### What moved where

- Code rules, test rules, checklist → `agents/common.md`
- Plan execution protocol, gates, 4 questions, protected files → `agents/executor.md`
- Planning rules, adversarial workflow, claimed_files, OODA → `agents/planner.md`
- Validation protocol, rejection criteria → `agents/validator.md`
- Key learnings → knowledge store (not static files)
- Merge protocol, workspace isolation → daemon enforces (not agent rules)

## Consequences

- 73% reduction in boot tax per spawn
- Role-specific rules: agents only read what they need
- AGENTS.md is a redirect, not a source of truth — reduces drift
- New rules go in the appropriate role file, not in a monolith
