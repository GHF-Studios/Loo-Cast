# Rhai Generic Binding Policy

This document is the canonical policy for exposing Rust generic-like behavior
into Rhai.

## Core constraint

Rhai cannot request new Rust monomorphizations at runtime.

Therefore, all "generic" behavior in scripts must resolve through:

1. reflected generic metadata (definition + instantiation),
2. compile-time signature catalogs (`inventory`),
3. runtime resolver/provider dispatch keyed by canonical IDs.

## Canonical pipeline

1. **Declare generic metadata**
   - use `reflect_extern_generic_definition!`
   - use `reflect_extern_generic_instantiation!` for concrete exposed cases
2. **Register dispatch signatures**
   - query/message/bundle signature entries in `bridges/domains/bevy/ecs/catalog/*`
3. **Resolve by normalized key**
   - query key: `(query_data_key, query_filter_key)`
   - message key: `message_type_id`
   - bundle key: `(instance_type_id, trait_id)`
4. **Bridge through providers**
   - `AccessCellProvider` implementations resolve dispatchers and execute against world access
5. **Exercise from scripts**
   - startup test scripts are the integration surface

## Required invariants

- Signature IDs must be explicit and prefixed:
  - `QUERY_SIG__*`
  - `MESSAGE_SIG__*`
  - `BUNDLE_SIG__*`
- Type/trait IDs must be canonical Rust path-style strings (for example `a::b::Type`).
- Dispatch keys must be deterministic and normalized.
- Duplicate registrations for the same dispatch key must panic during registry build.
- Resolver misses must panic with available-key context.

## Central enforcement

- Policy module:
  - `core_mod_api/src/rhai_binding/runtime/ecs/dispatch_policy.rs`
- Runtime registries that enforce policy:
  - `runtime/ecs/system/query/internals/statics.rs`
  - `runtime/ecs/message/internals/statics.rs`
  - `runtime/ecs/bundle/internals/statics.rs`
- Macroized registration entry shapes:
  - `submit_query_dispatch_entry!`
  - `submit_message_write_dispatch_entry!`
  - `submit_message_drain_dispatch_entry!`
  - `submit_bundle_spawn_dispatch_entry!`

## Automated checks

Policy tests live in `runtime/ecs/dispatch_policy.rs` and verify:

- signature prefixes and canonical path IDs for all registered entries,
- registration presence for known signatures,
- generic reflection metadata coherence with query dispatch registrations,
- resolver availability for known query/message/bundle keys.

## Add a new generic-like binding (checklist)

1. Add or extend reflected generic metadata.
2. Add signature constants and dispatch function(s) in a catalog file.
3. Register with the policy submission macro for that domain.
4. Resolve through provider path (no ad-hoc bypass).
5. Add startup script integration coverage.
6. Run:
   - `cargo check -p core_mod_api`
   - `cargo test -p core_mod_api dispatch_policy --lib`
   - `./build.sh dev`
   - `./run.sh dev`
