# Loo-Cast Docs

This directory is the canonical docs surface for active `loo_cast_alpha/` work.
If you arrived from repository root, use this file as the docs map and read-order source of truth.

## Read Order

1. `NOW.md` (current truth and active scope)
2. `ARCHITECTURE.md` (layer boundaries and ownership cadence)
3. `CONTRACTS.md` (compatibility/version/migration policy)
4. `WORKFLOWS.md` (daily loop, phase rules, and GitHub process policy)
5. `AI_COLLABORATION.md` (supervised AI workflow prompt and gate sequence)
6. `DECISIONS.md` (durable policy decisions and rationale)
7. `CHANGELOG.md` (published milestone intent and release log posture)
8. `migrations/README.md` (migration guide naming/location rules)

## Entry By Intent

- I want current direction: `NOW.md`
- I want to build/package/run/audit: `WORKFLOWS.md`
- I want compatibility guarantees: `CONTRACTS.md`
- I want architecture boundaries: `ARCHITECTURE.md`
- I want policy history: `DECISIONS.md`
- I want crate docs posture and known deferrals: `RUSTDOC_BASELINE.md`

## Rust Docs Baseline

`RUSTDOC_BASELINE.md` inventories active crates and marks each as:

- documented and sufficient for current bootstrap scope, or
- intentionally minimal with rationale and follow-up routing.

This avoids silent documentation gaps while preserving Phase 0 scope discipline.
