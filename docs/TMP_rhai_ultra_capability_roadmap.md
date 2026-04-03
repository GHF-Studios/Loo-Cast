# TEMP Rhai/Bevy Ultra Capability Roadmap

Date: 2026-04-02  
Status: planning artifact (authoritative pending user review)

## Mission

Build a script-first engine surface where Rhai can drive ECS/USF/gameplay content at high power, while Rust remains the capability platform and safety boundary.

## Hard Constraints

1. Rhai cannot request new Rust monomorphizations at runtime.
2. Unsafe lifetime erasure must stay inside `AccessCell` + `AccessCellProvider`.
3. Dynamic APIs must dispatch through pre-registered signatures or explicit typed providers.
4. API structure must be explicit via domain/module concept graph, not a flat/global bridge blob.
5. Backward compatibility is optional; coherence and extensibility are primary.

## Current Baseline (Repository Reality)

1. ECS bridge is present but narrow:
   - `World::flush`, `commands`, `spawn_empty`, `spawn_single`, `spawn_batch`, `spawn_components`, `entity`, `entity_mut`, `get_entity`, `get_entity_mut`, `despawn`, `entities`, `query`, `query_filtered`, `single`, `single_filtered`, `exists`, `exists_filtered`, `insert_resource`, `init_resource`, `remove_resource`, `get_resource`, `get_resource_mut`, `has_resource`, `run_schedule`, `write_message`, `drain_messages`.
   - limited `Commands` and `EntityCommands` operations.
2. Query DSL exists (`QueryDataTerm`, `QueryData`, `QueryFilter`) with a still-limited but growing signature catalog (entity/player/chunk filters).
3. Message bridge exists only as probe write/drain smoke path.
4. Bundle spawn dispatch exists for a narrow set.
5. No broad script-level `Res`, `ResMut`, `Single`, `MessageReader`, `MessageWriter`, `Local` equivalent surfaces yet.
6. No comprehensive domain-graph contract map or capability matrix is published yet (beyond local feature contracts).

## Target Architecture

1. Script entrypoint contracts remain strict and typed.
2. Script-exposed capability families are organized by domain and concept graph edges.
3. ECS operations are exposed through descriptor + resolver systems, not ad-hoc unrestricted reflection.
4. USF content and orchestration stay script-authored; Rust hosts reusable kernels/providers.
5. Safety model is explicit through provider contracts and deterministic runtime checks, not user-facing gate levels.

## Capability Families (Planned)

### A) ECS World/Entity/Commands Core

1. World entity lifecycle:
   - `spawn`, `spawn_single`, `spawn_batch`, `despawn`, `entity`, `entity_mut`, `get_entity`.
2. Entity mutation:
   - `insert/remove` component and bundle signatures.
3. Commands parity:
   - `spawn`, `entity`, `insert`, `remove`, `despawn`, deferred chaining.

### B) ECS Query and Filter System

1. Expand query signature catalog from smoke level to broad coverage.
2. Filter combinators:
   - `With`, `Without`, `Added`, `Changed`.
3. Query payload modes:
   - value/ref/mut term combinations with deterministic dispatch keys.
4. Query result ergonomics:
   - cursor API plus typed tuple/object extraction helpers.

### C) ECS Resources and SysParam-Like Access

1. `Res`/`ResMut` style resource readers/writers via typed wrappers.
2. `Single` style access for unique query contracts.
3. `Local` state bridge for system-local script execution contexts.
4. Optional reflection/introspection for resource presence and metadata.

### D) Messages/Events/Observers

1. Typed `MessageWriter`/`MessageReader` families by registered message type.
2. Drain/peek/iteration patterns with deterministic ordering rules.
3. Optional event compatibility layer if required by your architecture.
4. Observer registration bridge where lifecycle can be bounded and auditable.

### E) Scheduling and System Entry

1. Script entrypoints as first-class schedule hooks by domain.
2. Explicit schedule registration APIs:
   - add schedule, add system bundle, run schedule, run system.
3. Binding of domain-specific injected params per entrypoint type.
4. Runtime diagnostics for missing signatures/capabilities.

### F) Script Runtime/Meta Surface

1. Stronger trait-object ergonomics for script-constructible descriptors.
2. Generic-like API strategy:
   - descriptor DSL + signature catalogs + code generation.
3. Type-id and trait-id helper APIs for stable canonical path IDs.
4. Better error models (`Result`-style where useful) instead of panic-only user-facing experience.

### G) Host Std/Engine Facilities

1. Logging:
   - structured log APIs (`trace/debug/info/warn/error`) with domain tags.
2. File IO:
   - default unrestricted script access; optional project-level sandbox mode for development tooling.
3. Asset/service hooks:
   - audio trigger, particles, mesh/collider/material, timers, random streams.
4. Time/math/util sets aligned to gameplay and simulation authoring.

### H) USF Domain Capability Surface

1. Keep typed USF contexts first-class and explicit.
2. Expand script APIs for phenomenon model contracts, zone orchestration, partition behavior.
3. Preserve strict canonical persistence boundaries.
4. Keep runtime capability implementations decoupled from content definitions.

## API Domain Graph Model (Proposed)

1. No capability gate tiers.
2. Expose broad functionality through explicit domain tree and concept graph:
   - `bevy::ecs::world::*`
   - `bevy::ecs::system::*`
   - `bevy::ecs::query::*`
   - `bevy::ecs::message::*`
   - `core_mod_api::usf::*`
3. Keep the graph coherent:
   - each module owns related types/functions,
   - cross-domain references are explicit and documented.
4. Keep enforcement at implementation layer:
   - provider contracts,
   - dispatch validation,
   - deterministic failure diagnostics.

## Implementation Strategy

1. Keep manual, curated signatures for critical paths.
2. Add codegen for large repetitive catalogs (query/resource/message signatures).
3. Keep resolver registries deterministic and validated at startup.
4. Keep all unsafe boundaries centralized and heavily tested.

## Phased Execution Plan

### Phase 1: Domain Graph and Contract Infrastructure

1. Formalize domain graph metadata and ownership map.
2. Ensure bridge modules mirror concept boundaries.
3. Add diagnostics for unresolved/invalid cross-domain bindings.

### Phase 2: ECS Core Expansion

1. Expand world/entity/commands operations with provider-backed wrappers.
2. Add bundle/component insertion and removal signatures.
3. Add deterministic integration tests.

### Phase 3: Query System Expansion

1. Implement broad query/filter signature catalog growth.
2. Add codegen pipeline for query signatures.
3. Add stress tests for dispatch-key determinism.

### Phase 4: Resource and SysParam Surface

1. Introduce `Res`/`ResMut`/`Single` wrappers.
2. Add `Local` and resource lifecycle helpers.
3. Add runtime contract checks and diagnostics.

### Phase 5: Messages and Observers

1. Add typed reader/writer signatures.
2. Add message/event iteration helpers.
3. Add observer registration and teardown contracts.

### Phase 6: Scheduling and Entrypoint Integration

1. Expose explicit schedule APIs from scripts.
2. Bind domain-specific entrypoint params to these systems.
3. Validate compatibility with USF phased runtime.

### Phase 7: Host Utilities and IO

1. Add logging/time/math utilities.
2. Add file IO and optional project-level sandbox policies.
3. Add audio/particle/asset capability helpers.

### Phase 8: USF-First Demonstration and Hardening

1. Build one complete scripted technical demo using expanded surfaces.
2. Validate deterministic behavior and persistence invariants.
3. Lock documentation and capability matrix for mod authors.

## Testing and Quality Gates

1. Unit tests for each registry and descriptor normalizer.
2. Startup integration scripts for each capability family.
3. Determinism tests for dispatch keys and resolver behavior.
4. Major checkpoints:
   - `cargo fmt --all`
   - `cargo check -p core_mod_api`
   - `cargo test -p core_mod_api`

## Locked Decisions (2026-04-03)

1. No user-facing capability gate tiers or allowlist model.
2. Pipeline stays Rust-defined and mostly static; scripts primarily supply data and entrypoint implementations.
3. Failure surface should support both explicit handling (`Result`-style paths) and easy panic fallback for fail-fast workflows.
4. File IO defaults to unrestricted for scripts; optional sandbox mode remains a project-level tooling option.

## Remaining Decision

1. Type exposure growth strategy:
   - fully manual catalog expansion only, or catalog codegen from domain manifests?
