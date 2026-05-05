# Rust Docs Baseline

Purpose:

- Make current crate documentation posture explicit.
- Mark what is intentionally minimal during bootstrap.
- Route larger documentation expansion to concrete follow-up issues instead of silent drift.

## Active Crate Inventory

| Crate | Type | Current rustdoc/readme posture | Current status |
|---|---|---|---|
| `core_engine` | binary | crate-level bootstrap doc comment in `src/main.rs`; no crate README | intentionally minimal (bootstrap stub) |
| `core_mod` | library | crate-level bootstrap doc comment in `src/lib.rs`; no crate README | intentionally minimal (bootstrap stub) |
| `base_mod` | library | crate-level bootstrap doc comment in `src/lib.rs`; no crate README | intentionally minimal (bootstrap stub) |
| `core_engine_macros` | proc-macro lib | crate-level bootstrap doc comment in `src/lib.rs`; no crate README | intentionally minimal (bootstrap stub) |
| `core_mod_macros` | library | crate-level bootstrap doc comment in `src/lib.rs`; no crate README | intentionally minimal (bootstrap stub) |
| `base_mod_macros` | proc-macro lib | crate-level bootstrap doc comment in `src/lib.rs`; no crate README | intentionally minimal (bootstrap stub) |
| `bevy_consumable_message` | library | module/crate docs in `src/lib.rs` + `README.md` | documented enough for current reuse |
| `launcher` | binary | crate-level doc comment in `src/main.rs`; no crate README | intentionally minimal (bootstrap launcher stub) |
| `xtask` | binary | crate-level doc comment in `src/main.rs`; no crate README | documented enough for current command surface |

## Intentional Minimalism Rationale

- `core_engine`, `core_mod`, `base_mod`, and macro support crates are bootstrap stubs in this phase.
- Their module graphs are intentionally not restored yet; duplicating deep crate READMEs now would create churn and stale
  pseudo-contracts.
- Current required behavior is limited to crate identity surfaces and execution-rail continuity, which is captured by
  existing crate-level doc comments and `WORKFLOWS.md`.

## Deferred Documentation Work (Explicit Handoff)

Larger docs expansion is intentionally deferred to later phase work:

- `#7`: Phase 1 docs baseline and read-order alignment in live execution context.
- `#35`: umbrella for docs discoverability and Rust docs posture (parent of this baseline pass).

When crate/module surfaces become non-stub, add or expand crate READMEs and API-focused rustdoc in the same PR that
introduces the real surface.
