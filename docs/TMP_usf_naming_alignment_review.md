# TEMP USF Naming Alignment Review (Deep Pass)

Date: 2026-04-03  
Status: pending user corrections (authority = user mental model)

## Why this exists

The code currently compiles, but naming still leaks implementation-centric framing (`runtime/*`, `capability/*`, `hydrate_*`) instead of reflecting the intended architecture:

1. Rhai entrypoints are content authority.
2. Rust is execution kernel / bridge platform.
3. Ontology authority is `Phenomenon` + `PhenomenonModel` (+ partial model records).
4. Substrate/zone/output are derived runtime layers.

This document is intentionally opinionated and designed for line-by-line correction.

## Observed High-Impact Naming Drift

## A) `runtime/manifestation` + `runtime/capability` are too explicit and implementation-centric

Current code surface:

- `core_mod_api/src/usf/runtime/mod.rs`
- `core_mod_api/src/usf/runtime/manifestation/*`
- `core_mod_api/src/usf/runtime/capability/manifestation.rs`

Problem:

1. Path names encode internal execution mechanics instead of concept ownership.
2. `capability` appears as a top-level structural noun, but in the desired framing capabilities are implicit bridge mechanics, not first-class user-facing architecture buckets.
3. “runtime” as an explicit subtree for these concepts reinforces “engine side owns content behavior” perception.

Impact:

1. Misleads contributors about authority boundaries.
2. Encourages more code to accrete under generic “runtime” and “capability” buckets.

## B) `chunk_manifestation` is overloaded and mixes ontology/projection/debug semantics

Current code surface:

- types and systems prefixed with `ChunkManifestation*`
- config namespace `[usf.runtime.chunk_manifestation]`
- workflow id `HydrateChunkManifestationInstances`

Problem:

1. “manifestation” is a valid concept at phenomenon-model contract level, but `chunk_manifestation` now also names cache artifacts, mesh building, collider attach, and workflow execution.
2. It reads like “chunk owns manifestation ontology”, which conflicts with entity-grounded authority.
3. It still carries historical “chunk surface” flavor in a new name.

Impact:

1. Hard to tell what is authority vs cache vs output application.
2. Encourages global behavior flags over per-model contracts.

## C) `hydration` naming is too mechanical and wrong at semantic level

Current code surface:

- `hydration_workflow.rs`
- `queue_chunk_manifestation_hydration_requests_system`
- `run_chunk_manifestation_hydration_workflow_system`

Problem:

1. “Hydration” is storage/process jargon, not domain meaning.
2. The real operation is “reconcile chunk output artifacts from zone+model contracts”.

Impact:

1. Hides the actual policy boundary (selection already decided upstream).
2. Makes pipeline harder to reason about.

## D) workflow registry naming remains chunk-owned for USF projection execution

Current code surface:

- `core_mod_api/src/chunk/workflows/mod.rs` defines `HydrateChunkManifestationInstances`
- implementation now lives in USF path

Problem:

1. Ownership and execution id names diverge.
2. Registry macro coupling (`Chunk` namespace) forces mixed framing.
3. The `core_mod_api/src/chunk` module is not yet fully integrated and moved into `core_mod_api/src/usf` as a submodule.

Impact:

1. Persistent contributor confusion around “is this chunk core behavior or USF derived projection behavior?”

## E) Capability IDs and authority domain IDs are still tied to old phrasing

Current code surface:

- capability ids: `presentation.chunk_manifestation.instance_render`, etc.
- authority domain: `usf.runtime.manifestation.runtime`

Problem:

1. Repeats the overloaded `chunk_manifestation` term in contracts.
2. `...manifestation.runtime` is tautological and low-signal.

Impact:

1. Diagnostic messages remain semantically noisy.
2. Slows future capability-family growth (audio/particles/fx/etc.) under clean names.

## F) `PhenomenonKind` currently encodes demo/debug literal in ontology enum

Current code surface:

- `PhenomenonKind::ManifestationDensityDebug`
- script uses `ctx.register("manifestation_density_debug")`

Problem:

1. Debug content category is embedded in ontology type.
2. This is content framing leaking into core type system.

Impact:

1. Limits generalization.
2. Reinforces wrong mental model that engine ontology should enumerate gameplay/debug kinds.

## G) Hook/entrypoint vocabulary split is still uneven

Current code surface:

- strict USF typed entrypoints in bootstrap (`register_zone(ctx)`, etc.)
- separate “hook” system still uses `main(world, params)` with legacy fallback `main(world)`

Problem:

1. Two different mental models for script invocation coexist.
2. “hook” naming is generic and not aligned with “entrypoint contract” framing.

Impact:

1. Makes script architecture feel less unified.
2. Encourages accidental drift back to ad-hoc scripting patterns.

## H) `DPT` naming still carries legacy dense-metric historical baggage

Current code surface:

- module `usf/dpt/*`
- types `DptChunkRecord`, `DptStore`, `DptSchema`

Problem:

1. The acronym is historically tied to dense “Data Point Matrix” framing.
2. Current architecture treats these as schema/sampler/categorizer vector contracts, not dense canonical storage.

Impact:

1. Constant cognitive mismatch for new contributors.
2. Makes docs and code feel like two eras stitched together.

## Proposed Naming Direction (Strict, Final-State-Oriented)

This is the proposed target vocabulary to review and correct before broad rename.

## 1) Top-level USF tree framing

Proposed buckets:

1. `usf/core/*`  
   Script entrypoint declarations + immutable registration artifacts.
2. `rhai_binding/bridges/*`  
   Engine-facing execution kernels/channels (mesh/collider/audio/particles/interaction), mostly invisible from content perspective.
3. `usf/phenomenon/*`
4. `usf/full_phenomenon_model/*`
5. `usf/partial_phenomenon_model/*`
6. `usf/metric/*`
7. `usf/metric_set/*`
8. `usf/zone/*`
9. `usf/zlm/*`
10. `usf/scale/*`
11. `usf/mods/*`
12. `usf/mod_packs/*`  

## Consequences of your chosen framing

1. `usf/runtime/*` stops being a first-class architecture bucket.
2. `usf/capability/*` stops being a first-class architecture bucket.
3. Content-facing concept boundaries are represented by explicit domain modules (`phenomenon`, `zone`, `metric`, `zlm`, `scale`, `mods`, `mod_packs`).
4. Engine execution mechanics are hosted under `rhai_binding/bridges/*` and consumed implicitly via ctx/entrypoints.
5. `core_mod_api/src/chunk/*` should ultimately become a USF-owned submodule path instead of a parallel ownership root.

## 2) Replace `chunk_manifestation` with `chunk_realization` as model-driven output build

Default proposal:

1. Stop using `manifestation` as an architecture noun for this pipeline.
2. Treat this pipeline as chunk realization from selected phenomena + model state + DPT-driven realization intents.
3. `ChunkManifestationBinding` -> `ChunkRealizationIntent` (structural, not rename-only).
4. `UsfChunkManifestationInstance` -> `ChunkRealizationInstance`.
5. `UsfChunkManifestationStore` -> `ChunkRealizationCache`.
6. `CachedChunkManifestationRecord` -> `ChunkRealizationRecord`.

Reason:

1. Zones select phenomena; they do not own or expose engine capabilities.
2. Phenomena Models do not define engine capability contracts; they provide model state/projection/simulation semantics.
3. Engine capabilities are rust-native and exposed through bridge registries into Rhai.
4. Chunk realization should consume resolved intents, not hardcoded per-model capability contract structs.
5. “Manifestation” is overloaded and imprecise for this process.
6. “Realization” better communicates: empty chunk -> populated chunk state/output.
7. Chunk realization remains a cache/output of model-driven intents, never canonical authority.

## 3) Replace `hydration` wording with reconciliation wording

Default proposal:

1. `hydration_workflow.rs` -> `reconcile_chunk_realization_artifacts_workflow.rs`.
2. `HydrateChunkManifestationInstances` workflow id -> `ReconcileChunkRealizationArtifacts`.
3. Queue/run systems renamed from `*_hydration_*` to `*_reconcile_*`.
4. Reconcile workflow is split into two explicit stages:
   `resolve_chunk_realization_intents` and `apply_chunk_realization_outputs`.
5. Remove direct artifact assembly that embeds output capability assumptions in USF runtime structs.

Reason:

1. “Hydration” describes mechanism, not intent.
2. “Reconcile” reflects authoritative policy flow from zone selection + model state + DPT/runtime intent resolution.
3. This is a pipeline refactor, not just a naming refactor.

## 4) Move engine-output application under bridges, not USF runtime nouns

Default proposal:

1. Remove `usf/runtime/capability/*`.
2. Host mesh/collider/audio/particle/trigger application code under:
   `rhai_binding/bridges/domains/core_mod_api/usf/realization_channels/*`, plus other bridge domains as needed (`render`, `camera`, `logging`, etc.).
3. Add a bridge-side capability registry that maps rust-native channel ids to handlers.
4. Keep USF modules focused on declaration/state/contracts + intent orchestration, not direct capability application.
5. Remove USF-side per-capability application structs that duplicate bridge responsibility.

Reason:

1. This matches your “capabilities are implicit bridges” model.
2. Keeps capability execution in engine/bridge layer while model semantics/state stay in USF ontology.
3. Avoids reintroducing hidden contracts in USF modules.

## 5) Flatten capability identifiers into top-level capability domains

Default proposal:

1. `presentation.chunk_manifestation.instance_render` -> `mesh`.
2. `simulation.chunk_manifestation.instance_collider` -> `collider`.
3. `presentation.chunk_manifestation.instance_audio` -> `audio`.
4. `presentation.chunk_manifestation.instance_particles` -> `particles`.
5. `interaction.chunk_manifestation.instance_trigger` -> `trigger`.

Reason:

1. Removes forced middle segment and forced lane split (`presentation` vs `simulation`).
2. Capability identities become simple, top-level, and bridge-native.
3. Requires migration of capability graph validation and bridge handler registration, not just string replacement.

## 6) Authority ids as role-based domains (non-hierarchical)

Default proposal:

1. Drop `derived` as a naming class in domain ids.
2. Use a neutral pattern: `usf.<domain>.<role>`.
3. Role tags are explicit but flexible:
   `definition`, `selection_state`, `runtime_state`, `realization_state`, `persistence_state`, or whatever you need really.
4. Example ids:
   `usf.phenomenon.definition`,
   `usf.phenomenon.runtime_state`,
   `usf.full_phenomenon_model.runtime_state`,
   `usf.partial_phenomenon_model.runtime_state`,
   `usf.zone.definition`,
   `usf.zone.selection_state`,
   `usf.zlm.definition`,
   `usf.dpt.runtime_state`.
5. Persistence policy is expressed as a separate contract, not by “derived/not-derived” wording in the id:
   phenomenon/model(+partial) are persisted authority;
   zone/dpt/realization states are rebuildable execution state.

Reason:

1. All USF domains stay first-class; only role differs.
2. Avoids loaded wording that implies conceptual inferiority.

## 7) Path/type rename matrix (medium/high-impact only, aligned to model-owned capabilities)

Default proposal:

1. `core_mod_api/src/usf/runtime/manifestation/*`
   -> `core_mod_api/src/usf/chunk/realization/*`
2. `core_mod_api/src/usf/runtime/capability/manifestation.rs`
   -> `core_mod_api/src/rhai_binding/bridges/domains/core_mod_api/usf/realization_channels.rs`
3. `ChunkManifestationBinding`
   -> `ChunkRealizationIntent`
4. `UsfChunkManifestationInstance`
   -> `ChunkRealizationInstance`
5. `UsfChunkManifestationStore`
   -> `ChunkRealizationCache`
6. `CachedChunkManifestationRecord`
   -> `ChunkRealizationRecord`
7. `HydrateChunkManifestationInstances`
   -> `ReconcileChunkRealizationArtifacts`
8. `*hydration*` function/type names
   -> `*reconcile*` names
9. `[usf.runtime.chunk_manifestation]`
   -> `[usf.chunk.realization]`
10. Remove/rework fields encoding direct output contracts in USF structs
    (for example collider/audio/particle toggles and payloads currently baked into realization binding records).

Reason:

1. Makes zone role explicit as selector/orchestrator, not capability owner.
2. Makes model role explicit as semantic source, while bridge layer owns rust-native capability execution.
3. Includes required structural data-shape refactors.

## 8) Entrypoint taxonomy and naming lock

Default proposal:

1. Registration entrypoints remain one-file/one-entity typed contracts:
   `register_metric(ctx)`, `register_metric_set(ctx)`, `register_zone(ctx)`,
   `register_zlm(ctx)`, `register_scale(ctx)`,
   `register_phenomenon(ctx)`, `register_phenomenon_model(ctx)`,
   `register_mod(ctx)`, `register_modpack(ctx)`.
2. Schedule execution scripts are reframed as schedule entrypoints, not hooks.
3. Type names migrate from `Hook*` to `ScheduleEntrypoint*`.
4. Legacy fallback `main(world)` is deprecated and then removed.

Reason:

1. One invocation model avoids conceptual split-brain in scripting surface.

## 9) `chunk` integration into USF ownership

Default proposal:

1. Target ownership path becomes `core_mod_api/src/usf/chunk/*`.
2. Current workflow macro coupling to `Chunk` namespace is treated as temporary infrastructure constraint.
3. Workflow ids and types follow USF naming even if macro container temporarily stays `Chunk`.
4. Final state removes dual ownership signals (`chunk/*` vs `usf/*` for same pipeline).

Reason:

1. Chunk orchestration is part of USF world realization, not separate ontology.

## 10) Keep DPT concept as first-class mutable container (rename acronym, do not remove concept)

Default proposal:

1. Keep the conceptual layer currently called DPT:
   a generic runtime data container that is parameterized by a metric set and stores/interacts/mutates metric data.
2. Rename acronym and types for clarity, but preserve semantics:
   `DptSchema` -> `MetricContainerLayout`,
   `DptStore` -> `MetricContainerStore`,
   `DptChunkRecord` -> `MetricContainerRecord`.
3. Keep `metric/*` and `metric_set/*` as declaration domains that feed the container layer.
4. Keep temporary aliases only during migration slice, then delete.

Reason:

1. Metrics and metric sets are definitions; container state is a separate needed concept.
2. You want to rename legacy wording, not erase the container abstraction.

## 11) Suggested execution slices (structure-first, then naming lock)

1. Slice A: Introduce bridge-side rust capability registry + Rhai exposure path.
2. Slice B: Replace legacy realization-binding structs with `ChunkRealizationIntent`.
3. Slice C: Split hydration workflow into intent-resolution and output-application reconciliation stages.
4. Slice D: Re-home output application code into `rhai_binding/bridges/.../realization_channels/*`.
5. Slice E: Rename paths/types/config keys to `chunk_realization` once behavior is migrated.
6. Slice F: Introduce `usf/chunk/*` ownership path and re-home chunk USF logic.
7. Slice G: Rename `dpt` layer/types while preserving the container concept.
8. Slice H: Remove aliases and stale runtime/capability wording.

Current implementation pass status:

1. Slice A: done (bridge-side `RealizationChannelRegistry` now owns enabled-channel + execution-contract validation; USF-side `capability` module removed).
2. Slice B: done (`ChunkRealizationIntent` replaced legacy binding type).
3. Slice C: done (workflow stages are `ResolveIntents` + `ApplyOutputs`, with commit stage now `applied/skipped` semantics and no hydration vocabulary left in chunk realization path).
4. Slice D: done (output application moved to bridge module `.../usf/realization_channels.rs`).
5. Slice E: done for runtime/contracts (`chunk_realization` paths + config keyspace + `manifestation*` contract terms migrated to `realization*` in code and scripts).
6. Slice F: done (`usf/chunk/*` owns chunk workflow definitions and callsites; workflow macros now resolve through explicit `root_path` wiring and no `crate::chunk` shim remains).
7. Slice G: done for runtime and scripts (`metric_container` module/types + script API names migrated from `dpt_*` to `metric_*` / `metric_container_layout_*`).
8. Slice H: in progress (`usf/runtime/capability/*` and `usf/runtime/manifestation/*` removed; stale wording remains primarily in temporary/historical docs).
9. Schedule-entrypoint vocabulary pass: done for runtime contract (`rhai_binding/engine/hook.rs` migrated to `schedule_entrypoint.rs`, `schedule_entrypoints` runtime module/path renamed to `schedule_entrypoints`, script assets moved to `scripts/ecs/schedule_entrypoints`, and legacy `main(world)` fallback removed in favor of `main(world, params)` only).
10. Workflow namespace ownership lock: done (`Chunk` workflow module id renamed to `UsfChunk`; generated paths now live under `usf::chunk::workflows::usf_chunk::*`; `ChunkPlugin` ownership moved into `UsfPlugin`).
11. Kind ontology decoupling: done (`PhenomenonKind` is now normalized string-backed and no longer hardcoded to `RealizationDensityDebug` enum variants).
12. Capability-tag cleanup: done (removed `ctx.add_capability(...)`, removed capability-tag storage/parsing in phenomenon definition registry, and removed `PhenomenonCapability` type from runtime API surface).
13. Phenomenon runtime state cleanup: done (`Phenomenon.kind` removed from ECS component and `ZonePhenomenonSupport.kind` removed; kind remains definition metadata used for persistence annotation only).
14. Persistence restore naming pass: done (`hydrate_persisted_phenomena_state_system` -> `restore_persisted_phenomena_state_system`, state `hydrated` -> `restored`).
15. Modpack ownership naming pass: done for primary path split (`usf/content` moved to `usf/mod_packs`; `usf/mods` introduced for configured-mod identities).
16. Non-mesh channel observability pass: done (`ChunkRealizationChannelAppliedEvent` introduced and chunk realization telemetry (`mesh/collider/audio/particles/trigger`) added under `[usf.chunk.realization]` diagnostics settings).
17. Partition-runtime observability pass: done (`PhenomenonDebugStats` now tracks `partitioned_root_models` + `partitioned_member_models`, exposed through schedule entrypoint params for script-side monitoring).
18. Definition-bucket removal pass: done (`usf::definition` removed; metric identity/value/storage types moved to `usf::metric`, layout moved to `usf::metric_container`, zone type id moved to `usf::zone`).

## 12) Explicit decision checklist (please mark each)

1. Confirm authority id scheme:
   `usf.<domain>.<role>` with role tags
   (`definition`, `selection_state`, `runtime_state`, `realization_state`, `persistence_state`, etc.).
2. Confirm model-output intent family naming (`ChunkRealizationIntent`, etc.).
3. Confirm reconciliation naming (`ReconcileChunkRealizationArtifacts`, `*_reconcile_*`).
4. Confirm bridge ownership for realization channels under `rhai_binding/bridges`.
5. Confirm flattened top-level capability ids (`mesh`, `collider`, `audio`, `particles`, `trigger`) with no lane split.
6. Confirm workflow ownership direction: temporary macro coupling accepted, final ownership under `usf/chunk/*`.
7. Confirm DPT concept is retained as container abstraction (rename only).
8. Confirm `Dpt*` rename targets (`MetricContainerLayout`, `MetricContainerStore`, `MetricContainerRecord`).
9. Confirm schedule-entrypoint vocabulary migration and removal of `main(world)` fallback.
10. Confirm removal of remaining first-class `runtime/capability` architecture wording after migration.
11. Confirm removal of USF-side explicit capability contract structs in favor of bridge registry + intent resolution.

Current pass note:
11 is now implemented for chunk realization path and phenomenon-model runtime shape
(USF no longer stores per-channel simulation/audio/particle/trigger model components;
channel payloads are resolved into `ChunkRealizationIntent.channel_payloads` and applied in bridge layer).
