# ADR-002: Security Audit & Fixes

**Status:** Accepted
**Date:** 2025-07-18
**Author:** Security audit (automated)

## Context

The `convergio-agents` crate was extracted from the monorepo and needed a
security-first audit before broader adoption. This crate defines the agent
catalog (70+ agent specs) and exposes CRUD HTTP endpoints. Although it does not
spawn processes itself, the data it stores (capabilities, model tiers, budgets)
directly governs what the daemon will allow agents to do — making input
integrity critical.

## Audit Checklist

| Category | Risk | Finding |
|----------|------|---------|
| SQL injection | Low | All queries use `params![]` — safe. LIMIT was interpolated via `usize` format (safe but non-idiomatic); now parameterized. |
| Path traversal | None | No filesystem operations. |
| Command injection | None | No `std::process` usage. Catalog is read-only data for the daemon spawner. |
| SSRF | None | No outbound HTTP requests. |
| Secret exposure | None | No secrets in code or config. |
| Race conditions | Low | SQLite serialized via r2d2 pool — acceptable. |
| Unsafe blocks | None | Zero `unsafe` in crate. |
| Input validation | **HIGH** | `AgentInput` accepted unbounded strings, negative budgets, arbitrary model tiers. **Fixed.** |
| Auth/AuthZ bypass | Medium | Routes have no auth middleware. Documented as known limitation — auth is provided by the daemon host, not individual crates. |
| Privilege escalation | Low | Capabilities are strings; enforcement is in the daemon spawner, not here. Validation ensures only well-formed capability names are stored. |

## Changes Made

1. **Input validation** (`types.rs`):
   - Name: 1–128 chars, `[a-zA-Z0-9_-]` only
   - Role: 1–512 chars
   - Org: 1–128 chars
   - Model tier: must be `t1`/`t2`/`t3`/`t4`
   - Max tokens: 1–2,000,000
   - Hourly budget: 0.0–10,000.0
   - Capabilities: max 32 entries, each max 64 chars

2. **Parameterized LIMIT** (`store.rs`):
   - LIMIT now uses a bind parameter instead of format interpolation
   - Capped at 500 to prevent excessive result sets

3. **Validation enforced at both layers**:
   - Routes return HTTP 400 on invalid input
   - Store functions reject invalid input at the DB layer (defense in depth)

4. **8 new validation tests** covering all boundary conditions.

5. **README** fixed: replaced template placeholder with actual description.

6. **ADRs copied** from monorepo: ADR-007, ADR-009, ADR-029, ADR-044.

## Known Limitations (Future Work)

- **Authentication**: Routes rely on the daemon host to enforce auth. If
  mounted standalone, all endpoints are open. Consider adding optional auth
  middleware.
- **Rate limiting**: Not implemented. Should be added at the gateway/daemon
  level.
- **Capability allowlist**: Currently any string up to 64 chars is accepted.
  A future ADR should define the canonical capability vocabulary.

## Decision

Accept all fixes. The crate is safe for production use within the daemon's
auth boundary.
