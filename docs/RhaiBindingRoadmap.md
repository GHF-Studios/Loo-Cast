# Rhai Binding Roadmap

This is the canonical backlog for expanding the Rhai dialect bridge surface.

## Status legend

- `[x]` implemented and wired into bridge API
- `[~]` partially implemented or prototype-only
- `[ ]` not implemented yet

## Current architecture rules

- Runtime wrappers and internals live in `core_mod_api/src/rhai_binding/runtime/*`.
- Reflection/registration declarations live in `core_mod_api/src/rhai_binding/bridges/*`.
- Generic-like runtime dispatch is catalog-based (`bridges/domains/bevy/ecs/catalog/*`).
- Generic-like dispatch policy/invariants are centralized in:
  - `runtime/ecs/dispatch_policy.rs`
- Query/message/bundle runtime resolver registries are normalized under:
  - `runtime/ecs/system/query/internals/*`
  - `runtime/ecs/message/internals/*`
  - `runtime/ecs/bundle/internals/*`
- Bundle spawn dispatch is catalog-backed via typed world access requests (no `BundleFromDynamic` path).
- Scoped ECS wrapper bindings avoid `rhai::Shared`; AccessCell-backed wrappers are passed directly.
- Unsafe ECS access boundaries must go through `AccessCell` + `AccessCellProvider`.

## World bindings hierarchy

### World core lifecycle and command entry

- `[x] World::flush`
- `[x] World::commands`
- `[x] World::spawn_empty`
- `[x] World::spawn_single` (registered bundle signatures only)
- `[x] Bundle spawn dispatch registry (`BUNDLE_SIG__PLAYER__SPAWN_SINGLE`)

### World querying and runtime generic dispatch

- `[x] World::query`
- `[x] World::query_filtered`
- `[x] World::single` / `World::single_filtered`
- `[x] World::exists` / `World::exists_filtered`
- `[~] Query descriptor DSL for value/ref/mut terms and filter sets
- `[~] More pre-registered query signatures beyond entity/player smoke coverage (added chunk actor/loader filters)

### World messages/events

- `[x] Script probe message write/drain smoke path
- `[x] Type-id keyed `World::write_message` / `World::drain_messages` dispatch path
- `[x] Message write/drain dispatch registry (`MESSAGE_SIG__SCRIPT_PROBE__WRITE`, `MESSAGE_SIG__SCRIPT_PROBE__DRAIN`)
- `[~] Message iterator shape (`StringIter`) exists, still narrow
- `[ ] General MessageReader / MessageWriter bridge signatures
- `[ ] Event-style compatibility layer if needed

### World spawn/despawn/entity operations

- `[x] World::despawn`
- `[~] World::spawn` variants beyond `spawn_single` (`spawn_batch`, `spawn_components`)
- `[x] World::spawn_batch`
- `[x] World::spawn_components`
- `[x] World::entity`
- `[x] World::entity_mut`
- `[x] World::get_entity`
- `[x] World::get_entity_mut`
- `[x] World::entities`

### World resources

- `[x] World::insert_resource` (type-id keyed payload dispatch)
- `[x] World::init_resource` (catalog-registered types)
- `[x] World::remove_resource` (payload return for registered types)
- `[x] World::get_resource` (payload return for registered types)
- `[x] World::get_resource_mut` (payload return for registered types)
- `[x] World::has_resource`
- `[ ] World::removed` (removed-components/resources access path)

### World scheduling and systems

- `[x] World::known_schedules`
- `[ ] World::add_schedule`
- `[x] World::run_schedule` (known schedule-name map)
- `[ ] World::run_system`
- `[ ] Registering systems from Rhai (catalog-backed, predeclared signatures)
- `[ ] Observer registration bridge (`add_observer`)

### World ECS metadata/introspection

- `[ ] World::archetypes`
- `[ ] World::bundles`
- `[ ] World::components`
- `[ ] World::storages`

### World assets and misc

- `[ ] World::add_asset`
- `[ ] World::clear_all` (decide whether to expose at all)

## Commands / EntityCommands hierarchy

- `[x] Commands::spawn_empty`
- `[x] Commands::spawn_components` (registered component ctors, deferred insertion)
- `[x] Commands::spawn_components_batch` (batch deferred insertion)
- `[x] Commands::entity`
- `[x] Commands::despawn`
- `[x] EntityCommands::id`
- `[x] EntityCommands::commands`
- `[x] EntityCommands::insert_component` (registered component ctors, deferred insertion)
- `[x] EntityCommands::insert_components` (batch deferred insertion)
- `[x] EntityCommands::remove_component` (registered component removers, deferred removal)
- `[x] EntityCommands::despawn`
- `[ ] EntityCommands::remove bundle/multi-component variants beyond single type-id removal

## Entity World Mut hierarchy

- `[x] EntityWorldMut::id`
- `[x] EntityWorldMut::insert_component` (ctor-registry based)
- `[x] EntityWorldMut::insert_components` (batch ctor-registry based)
- `[x] EntityWorldMut::remove_component` (remover-registry based)

## Query hierarchy

- `[x] Query cursor (`next`, `remaining_len`, `is_empty`, `collect_remaining`)
- `[x] QueryDataTerm / QueryData / QueryFilter descriptors
- `[ ] Tuple/result-struct query data ergonomics beyond flat `Dynamic` payloads
- `[ ] ScopedRef / ScopedMut materialization for query terms
- `[ ] QueryFilter combinators mapped to Bevy semantics (`With`, `Without`, changed, added, etc.)

## Messages hierarchy

- `[x] Probe-message write and drain smoke path
- `[x] Catalog-backed message write/drain dispatcher resolution
- `[x] `World::write_message` / `World::drain_messages` type-id keyed API
- `[ ] Typed message writer signatures
- `[ ] Typed message reader signatures
- `[ ] Iterator bridges for message readers that mirror Rust usage patterns

## Bridge development process for each new capability

1. Add runtime wrapper/API in `rhai_binding/runtime/*`.
2. Add provider/access wiring in `bridges/domains/bevy/ecs/catalog/sysparam_providers.rs`.
3. Add reflection declarations in `bridges/domains/*`.
4. Register signatures in `bridges/domains/bevy/ecs/catalog/*` when generic/monomorphized.
5. Add startup test coverage in `core_mod/assets/scripts/ecs/schedule_entrypoints/startup/tests/*`.
6. Validate with:
   - `cargo check -p core_mod_api`
   - `./build.sh dev`
   - `./run.sh dev`
