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
- `[~] Query descriptor DSL for value/ref/mut terms and filter sets
- `[ ] More pre-registered query signatures beyond entity/player smoke coverage

### World messages/events

- `[x] Script probe message write/drain smoke path
- `[~] Message iterator shape (`StringIter`) exists, still narrow
- `[ ] General MessageReader / MessageWriter bridge signatures
- `[ ] Event-style compatibility layer if needed

### World spawn/despawn/entity operations

- `[ ] World::despawn`
- `[ ] World::spawn` variants beyond `spawn_single`
- `[ ] World::spawn_batch`
- `[ ] World::entity`
- `[ ] World::entity_mut`
- `[ ] World::get_entity`
- `[ ] World::get_entity_mut`
- `[ ] World::entities`

### World resources

- `[ ] World::insert_resource`
- `[ ] World::init_resource`
- `[ ] World::remove_resource`
- `[ ] World::get_resource`
- `[ ] World::get_resource_mut`
- `[ ] World::removed` (removed-components/resources access path)

### World scheduling and systems

- `[ ] World::add_schedule`
- `[ ] World::run_schedule`
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
- `[x] EntityCommands::id`
- `[x] EntityCommands::commands`
- `[ ] EntityCommands::insert for registered component/bundle signatures
- `[ ] EntityCommands::remove / despawn variants

## Query hierarchy

- `[x] Query cursor (`next`, `remaining_len`, `is_empty`, `collect_remaining`)
- `[x] QueryDataTerm / QueryData / QueryFilter descriptors
- `[ ] Tuple/result-struct query data ergonomics beyond flat `Dynamic` payloads
- `[ ] ScopedRef / ScopedMut materialization for query terms
- `[ ] QueryFilter combinators mapped to Bevy semantics (`With`, `Without`, changed, added, etc.)

## Messages hierarchy

- `[x] Probe-message write and drain smoke path
- `[ ] Typed message writer signatures
- `[ ] Typed message reader signatures
- `[ ] Iterator bridges for message readers that mirror Rust usage patterns

## Bridge development process for each new capability

1. Add runtime wrapper/API in `rhai_binding/runtime/*`.
2. Add provider/access wiring in `bridges/domains/bevy/ecs/catalog/sysparam_providers.rs`.
3. Add reflection declarations in `bridges/domains/*`.
4. Register signatures in `bridges/domains/bevy/ecs/catalog/*` when generic/monomorphized.
5. Add startup suite coverage in `core_mod/assets/scripts/core/schedule_hooks/startup/*`.
6. Validate with:
   - `cargo check -p core_mod_api`
   - `./build.sh dev`
   - `./run.sh dev`
