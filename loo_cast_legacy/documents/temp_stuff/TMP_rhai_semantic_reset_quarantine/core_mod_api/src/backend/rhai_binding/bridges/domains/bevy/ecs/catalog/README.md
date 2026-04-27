# ECS Bridge Catalogs

This folder holds compile-time registrations for ECS features that cannot be materialized dynamically at runtime.

## Files

- `query_signatures.rs`
  - query dispatch signatures (`inventory`) keyed by data/filter descriptors.
- `message_signatures.rs`
  - message reader/writer/drain signatures.
- `bundle_signatures.rs`
  - bundle insertion/spawn signatures.
- `sysparam_providers.rs`
  - `AccessCellProvider` implementations wiring world/sysparam access boundaries.
- `runtime/ecs/dispatch_policy.rs`
  - shared generic-dispatch policy and invariant checks used by query/message/bundle registries.

## Naming guideline

Use explicit signature IDs, for example:

- `QUERY_SIG__ENTITY__WITH_PLAYER`
- `MESSAGE_SIG__SCRIPT_PROBE__WRITE`
- `MESSAGE_SIG__SCRIPT_PROBE__DRAIN`
- `BUNDLE_SIG__PLAYER__SPAWN_SINGLE`

Keep signature names deterministic and grep-friendly.

## Registration shape

Prefer policy macros instead of raw `inventory::submit!` blocks:

- `submit_query_dispatch_entry!`
- `submit_message_write_dispatch_entry!`
- `submit_message_drain_dispatch_entry!`
- `submit_bundle_spawn_dispatch_entry!`
