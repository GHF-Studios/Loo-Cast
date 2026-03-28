# RhaiBridgeDevelopment — Extension Playbook

## Scope

This is the operational guide for adding new Rhai-exposed Bevy/Rust features and maintaining the signature catalogs used to emulate generics.

Related design context: `docs/RhaiDialect.md`.
Generic contract: `docs/RhaiGenericBindingPolicy.md`.
Coverage backlog: `docs/RhaiBindingRoadmap.md`.
Semantics intent reference: `docs/RhaiValueSemantics.md`.
Macro surface notes: `docs/RhaiMacroSurface.md`.

## File map

- Bridge declarations:
  - `core_mod_api/src/rhai_binding/bridges/domains/*`
  - `core_mod_api/src/rhai_binding/bridges/testing/*`
- ECS compile-time catalogs:
  - `core_mod_api/src/rhai_binding/bridges/domains/bevy/ecs/catalog/query_signatures.rs`
  - `core_mod_api/src/rhai_binding/bridges/domains/bevy/ecs/catalog/message_signatures.rs`
  - `core_mod_api/src/rhai_binding/bridges/domains/bevy/ecs/catalog/bundle_signatures.rs`
  - `core_mod_api/src/rhai_binding/bridges/domains/bevy/ecs/catalog/sysparam_providers.rs`
- Runtime wrappers:
  - `core_mod_api/src/rhai_binding/runtime/*`
- bundle spawn dispatch runtime registry:
  - `core_mod_api/src/rhai_binding/runtime/ecs/bundle/internals/types.rs`
  - `core_mod_api/src/rhai_binding/runtime/ecs/bundle/internals/statics.rs`
- message write/drain dispatch runtime registries:
  - `core_mod_api/src/rhai_binding/runtime/ecs/message/internals/types.rs`
  - `core_mod_api/src/rhai_binding/runtime/ecs/message/internals/statics.rs`
- query dispatch runtime registry:
  - `core_mod_api/src/rhai_binding/runtime/ecs/system/query/internals/types.rs`
  - `core_mod_api/src/rhai_binding/runtime/ecs/system/query/internals/statics.rs`
- shared generic-dispatch policy and validation:
  - `core_mod_api/src/rhai_binding/runtime/ecs/dispatch_policy.rs`
- Script integration tests:
  - `core_mod/assets/scripts/ecs/schedule_hooks/startup/tests/*`

## Workflow: add a new bridge feature

1. Define runtime wrapper/API methods.
- Add/update wrapper types and traits under `rhai_binding/runtime/*`.
- Keep API shape close to intended Rhai usage.

2. Implement access provider wiring.
- Add `AccessCellProvider<T>` impl(s) in `ecs/catalog/sysparam_providers.rs`.
- Keep method names and argument payload structs explicit.
- Preserve strict `start_access` -> use -> `end_access` lifecycle.
- For scoped ECS wrappers, pass AccessCell-backed wrapper values directly;
  avoid introducing `rhai::Shared` wrappers.

3. Expose reflection macros.
- Add or update extern reflection declarations in domain modules:
  - `reflect_extern_top_level_module!`
  - `reflect_extern_sub_module!`
  - `reflect_extern_type!`
  - method/constructor/item-associated/module-associated reflection macros

4. Register runtime functions into Rhai.
- Use `register_fn`, `register_raw_fn`, property getters, and module insertion APIs consistently.
- Keep conversions between Rhai `Dynamic` and wrapper types explicit.

5. Add startup test coverage.
- Add/extend `.rhai` test files under categorized startup `tests/*` folders.
- Keep `startup.rhai` as the entrypoint; call orchestrator helpers from there.

6. Validate.
- `cargo check -p core_mod_api`
- `cargo test -p core_mod_api dispatch_policy --lib`
- `./build.sh dev`
- `./run.sh dev`

## Workflow: add a new generic-like signature

Use this for Query/Message/Bundle patterns that require Rust monomorphization.

1. Pick the normalized dispatch-key shape.
- Query: `(data_key, filter_key)`.
- Message: `message_type_id` (per operation registry, e.g. write/drain).
- Bundle: `(instance_type_id, trait_id)`.

2. Add signature catalog entry.
- Register a dispatch entry with the domain policy macro:
  - `submit_query_dispatch_entry!`
  - `submit_message_write_dispatch_entry!`
  - `submit_message_drain_dispatch_entry!`
  - `submit_bundle_spawn_dispatch_entry!`
- Define:
  - signature id,
  - dispatch key fields,
  - dispatch function.

3. Add/extend descriptor DSL if needed.
- Update runtime descriptor types (`QueryDataTerm`, `QueryData`, `QueryFilter`, etc.).
- Keep descriptor -> dispatch-key mapping deterministic.

4. Resolve in provider path.
- Ensure AccessCellProvider request path maps request payload keys to runtime resolvers (`resolve_*_dispatch`).
- Avoid hardcoded one-off logic in providers when the operation has a catalog.
- Keep signature IDs and type/trait path IDs canonical so policy validators pass.

5. Add reflection metadata for discoverability.
- Add `reflect_extern_generic_definition!` and `reflect_extern_generic_instantiation!` where applicable.

6. Cover with startup script tests/examples.
- Add at least one positive-path script that exercises the new signature.
- Prefer alias-driven type-id tokens in scripts when available:
  - `use bevy::ecs::entity::Entity as Entity;`
  - `QueryData::single_t(Entity)`
  - `QueryFilter::require_t(Player)`

## Workflow: add a new bundle spawn signature

1. Add dispatch entry in `catalog/bundle_signatures.rs`.
- Register `BundleSpawnDispatchEntry` with:
  - signature id,
  - `instance_type_id`,
  - `trait_id` (`bevy::ecs::bundle::Bundle`),
  - dispatch function.

2. Resolve through provider path.
- `World::spawn_single` flows through typed request payload
  (`WorldSpawnSingleRequest`) and `resolve_bundle_spawn_dispatch`.

3. Keep constructor model explicit.
- Prefer reflected constructors + trait-object conversion
  (`PlayerBundle::new_default` + `as_trait_obj` style).
- Do not reintroduce legacy `BundleFromDynamic` flow.

## Naming conventions (recommended)

Use explicit and grep-friendly signature constants in catalogs:

- `QUERY_SIG__ENTITY`
- `QUERY_SIG__ENTITY__WITH_PLAYER`
- `QUERY_SIG__ENTITY_AND_PLAYER_REF__WITH_CHUNK_LOADER`
- `MESSAGE_SIG__SCRIPT_PROBE__WRITE`
- `MESSAGE_SIG__SCRIPT_PROBE__DRAIN`
- `BUNDLE_SIG__PLAYER__SPAWN_SINGLE`

Keep names auto-derivable from descriptor definitions where practical.

## Testing policy

- Treat startup hook tests as integration tests (with optional example-tests).
- Keep test-only bridges under `bridges/testing/*`.
- Keep production bridge APIs under `bridges/domains/*`.

## Migration policy

- Hard refactors are acceptable.
- Backwards compatibility is optional during migration, as long as final API is coherent and covered by startup test execution.
