# RhaiBridgeDevelopment — Extension Playbook

## Scope

This is the operational guide for adding new Rhai-exposed Bevy/Rust features and maintaining the signature catalogs used to emulate generics.

Related design context: `docs/RhaiDialect.md`.

## File map

- Bridge declarations:
  - `core_mod_api/src/rhai_binding/bridges/domains/*`
  - `core_mod_api/src/rhai_binding/bridges/testing/*`
- ECS compile-time catalogs:
  - `core_mod_api/src/rhai_binding/bridges/domains/ecs/catalog/query_signatures.rs`
  - `core_mod_api/src/rhai_binding/bridges/domains/ecs/catalog/message_signatures.rs`
  - `core_mod_api/src/rhai_binding/bridges/domains/ecs/catalog/bundle_signatures.rs`
  - `core_mod_api/src/rhai_binding/bridges/domains/ecs/catalog/sysparam_providers.rs`
- Runtime wrappers:
  - `core_mod_api/src/rhai_binding/runtime/*`
- Script integration suites:
  - `core_mod/assets/scripts/core/schedule_hooks/startup/*`

## Workflow: add a new bridge feature

1. Define runtime wrapper/API methods.
- Add/update wrapper types and traits under `rhai_binding/runtime/*`.
- Keep API shape close to intended Rhai usage.

2. Implement access provider wiring.
- Add `AccessCellProvider<T>` impl(s) in `ecs/catalog/sysparam_providers.rs`.
- Keep method names and argument payload structs explicit.
- Preserve strict `start_access` -> use -> `end_access` lifecycle.

3. Expose reflection macros.
- Add or update extern reflection declarations in domain modules:
  - `reflect_extern_top_level_module!`
  - `reflect_extern_sub_module!`
  - `reflect_extern_type!`
  - method/constructor/item-associated/module-associated reflection macros

4. Register runtime functions into Rhai.
- Use `register_fn`, `register_raw_fn`, property getters, and module insertion APIs consistently.
- Keep conversions between Rhai `Dynamic` and wrapper types explicit.

5. Add startup suite coverage.
- Add/extend `.rhai` suite files under categorized startup companion folders.
- Keep `startup.rhai` as the entrypoint; call orchestrator helpers from there.

6. Validate.
- `cargo check -p core_mod_api`
- `./build.sh dev`
- `./run.sh dev`

## Workflow: add a new generic-like signature

Use this for Query/Message/Bundle patterns that require Rust monomorphization.

1. Add signature catalog entry.
- Register a dispatch entry with `inventory::submit!`.
- Define:
  - signature id,
  - data/filter term definitions,
  - dispatch function.

2. Add/extend descriptor DSL if needed.
- Update runtime descriptor types (`QueryDataTerm`, `QueryData`, `QueryFilter`, etc.).
- Keep descriptor -> dispatch-key mapping deterministic.

3. Resolve in provider path.
- Ensure AccessCellProvider request path maps descriptor keys to catalog dispatchers.

4. Add reflection metadata for discoverability.
- Add `reflect_extern_generic_definition!` and `reflect_extern_generic_instantiation!` where applicable.

5. Cover with startup script examples.
- Add at least one positive-path script that exercises the new signature.

## Naming conventions (recommended)

Use explicit and grep-friendly signature constants in catalogs:

- `QUERY_SIG__ENTITY`
- `QUERY_SIG__ENTITY__WITH_PLAYER`
- `QUERY_SIG__ENTITY_AND_PLAYER_REF__WITH_CHUNK_LOADER`
- `MESSAGE_SIG__SCRIPT_PROBE__DRAIN`
- `BUNDLE_SIG__PLAYER__SPAWN_SINGLE`

Keep names auto-derivable from descriptor definitions where practical.

## Testing policy

- Treat startup hook suites as integration tests/examples.
- Keep test-only bridges under `bridges/testing/*`.
- Keep production bridge APIs under `bridges/domains/*`.

## Migration policy

- Hard refactors are acceptable.
- Backwards compatibility is optional during migration, as long as final API is coherent and covered by startup suite execution.
