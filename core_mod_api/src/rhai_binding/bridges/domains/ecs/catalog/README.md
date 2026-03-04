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

## Naming guideline

Use explicit signature IDs, for example:

- `QUERY_SIG__ENTITY__WITH_PLAYER`
- `MESSAGE_SIG__SCRIPT_PROBE__DRAIN`
- `BUNDLE_SIG__PLAYER__SPAWN_SINGLE`

Keep signature names deterministic and grep-friendly.
