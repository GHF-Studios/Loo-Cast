# Rhai Generic Binding Policy (Summary)

Core constraint: Rhai cannot request new Rust monomorphizations at runtime.

## Required Pipeline

1. Declare generic metadata.
2. Register compile-time signatures in catalogs.
3. Resolve by normalized deterministic dispatch keys.
4. Execute through provider/resolver paths.
5. Cover with startup integration registration flows.

## Required Invariants

- Stable signature ID prefixes:
  - `QUERY_SIG__*`
  - `MESSAGE_SIG__*`
  - `BUNDLE_SIG__*`
- Canonical Rust-path type/trait IDs.
- Duplicate key registration must fail fast.
- Resolver misses must fail fast with useful context.

## Enforcement Anchors

- `core_mod_api/src/backend/rhai_binding/runtime/ecs/dispatch_policy.rs`
- query/message/bundle runtime registry internals in `runtime/ecs/*/internals/*`
