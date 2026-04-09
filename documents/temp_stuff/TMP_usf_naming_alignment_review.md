# TEMP USF Naming Alignment Review (Deep Pass)

Date: 2026-04-03  
Status: revised after user corrections + drift reset (authority = user mental model)

## Why this exists

The code currently compiles, and the largest naming migrations are already landed.  
Remaining drift is now mostly documentation/status coherence and residual terminology pockets that still obscure the intended architecture:

1. Rhai entrypoints are content authority.
2. Rust is execution kernel / bridge platform.
3. Ontology authority is `Phenomenon` + `PhenomenonModel` (+ partial model records).
4. Substrate/zone/output are derived runtime layers.

This document is intentionally opinionated and designed for line-by-line correction.

## Terminology lock (authoritative)

1. Zones classify/select; zones do not own or expose engine output execution.
2. Phenomena + Phenomenon Models own semantic state and projection behavior.
3. Chunk realization reconciles selected model outputs into derived chunk artifacts.
4. Engine output execution is bridge-registered Rust functionality exposed to scripts; USF stores state/intents, not bridge execution logic.
5. Metric-container layer (formerly DPT naming) remains a first-class mutable container concept and is not removed.

## Observed High-Impact Naming Drift

## A) Resolved architectural naming migrations (historical record, not current drift)

These were major drift vectors and are now migrated on active runtime paths:

1. `usf/runtime/*` + `usf/runtime/capability/*` were removed from active ownership boundaries.
2. `chunk_manifestation` pipeline naming was migrated to `chunk_realization`.
3. `hydration` wording on active realization/persistence paths was migrated to `reconcile_*` / `restore_*`.
4. Workflow ownership moved under `core_mod_api/src/usf/chunk/*` (`core_mod_api/src/chunk/*` no longer exists).
5. Capability identifiers were flattened to top-level ids (`mesh`, `material`, `collider`, `audio`, `particles`, `trigger`, `simulation_service`).
6. Schedule entrypoint vocabulary replaced legacy hook naming and removed `main(world)` fallback.
7. `PhenomenonKind` debug enum hardcoding was removed from ontology runtime types.
8. `Dpt*` runtime type/module naming was migrated to `metric_container/*` on active paths.

This section remains only to preserve decision provenance.

## B) Remaining high-impact drift (active)

1. Some docs (especially historical/reference notes) still describe migrated surfaces and need explicit historical labeling.
2. Status/evidence semantics are inconsistent between tmp documents, creating planning drift.
3. Canonical entity naming must stay normalized (`PhenomenonModel` / `PartialPhenomenonModel`) across all docs and scripts.
4. Sampler/categorizer planning language still drifts between historical `dpt_*` ids and active `metric_*` ids.
5. Module-tree end-state framing (`usf/core`, deeper ownership map) is not yet fully reflected by real tree/state and should remain explicitly marked as target architecture, not current state.

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

Migration target (`historical -> current`, largely completed on active path):

1. Stop using `manifestation` as an architecture noun for this pipeline.
2. Treat this pipeline as chunk realization from selected phenomena + model state + DPT-driven realization intents.
3. `ChunkManifestationBinding` -> `ChunkRealizationIntent` (structural, not rename-only).
4. `UsfChunkManifestationInstance` -> `ChunkRealizationInstance`.
5. `UsfChunkManifestationStore` -> `ChunkRealizationCache`.
6. `CachedChunkManifestationRecord` -> `ChunkRealizationRecord`.

Reason:

1. Zones select phenomena; they do not own or expose engine capabilities.
2. Phenomena Models do not define bridge registrations; they provide model state/projection/simulation semantics.
3. Engine capabilities are rust-native and exposed through bridge registries into Rhai.
4. Chunk realization should consume resolved intents, not hardcoded per-model bridge-application structs.
5. “Manifestation” is overloaded and imprecise for this process.
6. “Realization” better communicates: empty chunk -> populated chunk state/output.
7. Chunk realization remains a cache/output of model-driven intents, never canonical authority.

## 3) Replace `hydration` wording with reconciliation wording

Migration target (`historical -> current`, largely completed on active path):

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

Migration target (`historical -> current`, largely completed on active path):

1. Remove `usf/runtime/capability/*`.
2. Host mesh/collider/audio/particle/trigger application code under:
   `rhai_binding/bridges/domains/core_mod_api/usf/output_channels/*`, plus other bridge domains as needed (`render`, `camera`, `logging`, etc.).
3. Add a bridge-side capability registry that maps rust-native channel ids to handlers.
4. Keep USF modules focused on declaration/state/intent schemas + intent orchestration, not direct capability application.
5. Remove USF-side per-capability application structs that duplicate bridge responsibility.

Reason:

1. This matches your “capabilities are implicit bridges” model.
2. Keeps capability execution in engine/bridge layer while model semantics/state stay in USF ontology.
3. Avoids reintroducing hidden bridge-execution assumptions in USF modules.

## 5) Flatten capability identifiers into top-level capability domains

Migration target (`historical -> current`, largely completed on active path):

1. `presentation.chunk_manifestation.instance_render` -> `mesh`.
2. `simulation.chunk_manifestation.instance_collider` -> `collider`.
3. `presentation.chunk_manifestation.instance_audio` -> `audio`.
4. `presentation.chunk_manifestation.instance_particles` -> `particles`.
5. `interaction.chunk_manifestation.instance_trigger` -> `trigger`.
6. `simulation.chunk_manifestation.instance_runtime_service` -> `simulation_service`.

Reason:

1. Removes forced middle segment and forced lane split (`presentation` vs `simulation`).
2. Capability identities become simple, top-level, and bridge-native.
3. Requires migration of capability graph validation and bridge handler registration, not just string replacement.

## 6) Authority ids as role-based domains (non-hierarchical)

Target proposal (partially migrated):

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

Migration matrix (`historical -> current`):

1. `core_mod_api/src/usf/runtime/manifestation/*`
   -> `core_mod_api/src/usf/chunk/realization/*`
2. `core_mod_api/src/usf/runtime/capability/manifestation.rs`
   -> `core_mod_api/src/rhai_binding/bridges/domains/core_mod_api/usf/output_channels.rs`
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
10. Remove/rework fields encoding direct bridge-execution assumptions in USF structs
    (for example collider/audio/particle toggles and payloads currently baked into realization binding records).

Reason:

1. Makes zone role explicit as selector/orchestrator, not capability owner.
2. Makes model role explicit as semantic source, while bridge layer owns rust-native capability execution.
3. Includes required structural data-shape refactors.

## 8) Entrypoint taxonomy and naming lock

Migration target (`historical -> current`, largely completed):

1. Registration entrypoints remain one-file/one-entity typed contracts:
   `register_metric(ctx)`, `register_metric_set(ctx)`, `register_zone(ctx)`,
   `register_zlm(ctx)`, `register_scale(ctx)`,
   `register_phenomenon(ctx)`, `register_phenomenon_model(ctx)`,
   `register_mod(ctx)`, `register_modpack(ctx)`.
2. Schedule execution scripts are reframed as schedule entrypoints, not hooks.
3. Type names migrate from `Hook*` to `ScheduleEntrypoint*`.
4. Legacy fallback `main(world)` is removed from schedule entrypoint execution.

Reason:

1. One invocation model avoids conceptual split-brain in scripting surface.

## 9) `chunk` integration into USF ownership

Target proposal (partially migrated):

1. Target ownership path becomes `core_mod_api/src/usf/chunk/*`.
2. Current workflow macro coupling already uses `UsfChunk` naming and is treated as temporary infrastructure constraint.
3. Workflow ids and types stay USF-owned even while workflow-macro infrastructure remains.
4. Final state removes dual ownership signals (`chunk/*` vs `usf/*` for same pipeline).

Reason:

1. Chunk orchestration is part of USF world realization, not separate ontology.

## 10) Keep DPT concept as first-class mutable container (rename acronym, do not remove concept)

Migration target (`historical -> current`, partially completed):

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
4. Slice D: Re-home output application code into `rhai_binding/bridges/.../output_channels/*`.
5. Slice E: Rename paths/types/config keys to `chunk_realization` once behavior is migrated.
6. Slice F: Introduce `usf/chunk/*` ownership path and re-home chunk USF logic.
7. Slice G: Rename `dpt` layer/types while preserving the container concept.
8. Slice H: Remove aliases and stale runtime/capability wording.

Current implementation pass status (recalibrated):

Status semantics align with the execution checklist:
`implemented`, `partially_validated`, `architecturally_complete`.

1. Slice A: `implemented` (bridge-side `OutputChannelRegistry` owns channel enablement + execution validation; USF-side `capability` module removed).
2. Slice B: `implemented` (`ChunkRealizationIntent` replaced legacy binding type).
3. Slice C: `implemented` (workflow stages are `ResolveIntents` + `ApplyOutputs`; hydration wording removed on active path).
4. Slice D: `implemented` (output application moved to bridge module `.../usf/output_channels.rs`).
5. Slice E: `partially_validated` (`chunk_realization` paths/config migrated, but naming drift can still exist outside active paths).
6. Slice F: `partially_validated` (`usf/chunk/*` owns active workflow definitions/callsites; broader module-tree alignment still pending).
7. Slice G: `partially_validated` (`metric_container` module/types + script API names migrated from `dpt_*`).
8. Slice H: `partially_validated` (stale runtime/capability wording remains in some docs/legacy paths).
9. Schedule-entrypoint vocabulary pass: `implemented` (`hook.rs` migrated to `schedule_entrypoint.rs`; legacy `main(world)` fallback removed).
10. Workflow namespace ownership lock: `implemented` (`Chunk` workflow module id renamed to `UsfChunk`; generated paths under `usf::chunk::workflows::usf_chunk::*`; plugin ownership moved into `UsfPlugin`).
11. Kind ontology decoupling: `implemented` (`PhenomenonKind` normalized string-backed, no hardcoded debug enum variants).
12. Capability-tag cleanup: `implemented` (removed `ctx.add_capability(...)` and runtime capability-tag storage/parsing).
13. Phenomenon runtime state cleanup: `implemented` (`Phenomenon.kind` removed from ECS runtime component state; kind remains definition metadata).
14. Persistence restore naming pass: `implemented` (`hydrate_*` restore naming migrated to `restore_*` on persistence-restore path).
15. Modpack ownership naming pass: `partially_validated` (`usf/content` -> `usf/mod_packs`; `usf/mods` introduced; consistency pass still pending).
16. Non-mesh channel observability pass: `implemented` (`ChunkRealizationChannelAppliedEvent` + per-channel diagnostics).
17. Partition-runtime observability pass: `implemented` (`PhenomenonDebugStats` includes partitioned model counters exposed to schedule entrypoint params).
18. Definition-bucket removal pass: `implemented` (`usf::definition` removed; metric/layout/zone types moved to domain modules).
19. Global mesh gate removal pass: `implemented` (`attach_meshes` removed; mesh output is now intent+channel-registration driven, matching model-scoped output authority).
20. Channel registration terminology pass: `implemented` (`RealizationChannelExecutionContract` -> `OutputChannelExecutionRegistration`,
    `has_contract` -> `has_registration`, and validation path renamed to registration wording).
21. Selected-model identity pass: `implemented` (`ChunkRealizationIntent`/snapshot/cache record now include `selected_model_id`; chunk realization cache schema bumped to v3 and stale v2 records are regenerated).
22. Projection/field spec terminology pass: `implemented` (`PhenomenonRealizationFieldContract` -> `PhenomenonOutputFieldSpec`,
    `PhenomenonModelProjectionContract` -> `PhenomenonModelProjection`, projection component field `contract` -> `spec`,
    and registry API renamed from `projection_contract_*` to `projection_spec_*`).
23. Script API naming pass: `implemented` (`set_projection_contract(...)` renamed to `set_projection_spec(...)` and demo model scripts updated).
24. Chunk intent-sync naming pass: `implemented` (`usf/chunk/realization/binding.rs` moved to `intent.rs`,
    `ChunkRealizationAuthorityGrace` -> `ChunkRealizationIntentGrace`, `binding_grace_frames` -> `intent_grace_frames`,
    and sync probes/counters renamed to `IntentSync*`).
25. Zone-model reconcile naming pass: `implemented` (`apply_zone_realization_startup_hooks_system` renamed to
    `reconcile_zone_realization_model_state_system` to remove hook-centric vocabulary from phenomenon runtime).
26. Chunk realization authority-id normalization pass: `implemented` (`USF_DOMAIN_CHUNK_REALIZATION_RUNTIME` renamed to
    `USF_DOMAIN_CHUNK_REALIZATION_STATE` with domain id `usf.chunk_realization.runtime_state`).
27. Runtime-cache terminology pass: `implemented` (chunk realization cache authority tag renamed from `derived_cache`
    to `runtime_cache` in record generation + load filter contracts).
28. USF module-tree cleanup pass: `implemented` (stale empty `usf/content` and `usf/definition` directories removed).
29. Material-channel extraction pass: `implemented` (`OutputChannelPayload::Material(...)` introduced as a first-class
    channel, `material` added to channel registry defaults, intent resolution emits separate mesh+material payloads,
    and schedule entrypoint telemetry now exposes `chunk_realization_material_instances`).
30. Runtime-model intent authority pass: `implemented` (chunk realization intent resolution now selects model ids from
    live `PhenomenonModel` entities at `(phenomenon_entity, scale)` with support-bounds matching, instead of directly
    sourcing selected model ids from definition-only lookup).
31. Phenomenon-model output API naming pass: `implemented` (Rhai model ctx APIs renamed from
    `set_realization_*`/`set_interaction_trigger` to `set_output_*` names, matching output-intent framing instead of
    forced realization/capability-contract wording).
32. Channel-gate removal pass: `implemented` (`OutputChannelRegistry` no longer carries a separate
    `enabled_channels` gate set; execution registrations are the single authority for channel availability).
33. Entrypoint descriptor/report pass: `implemented` (bootstrap now compiles deterministic `UsfEntrypointDescriptor` lists,
    records discovered/executed entrypoints in `USF_BOOTSTRAP_REPORT`, and exposes bootstrap counters to schedule entrypoint params).

Evidence backlog (required before claiming `architecturally_complete`):

1. Code anchors for every status claim:
   partially populated in this doc + `docs/TMP_usf_refactor_execution_checklist.md`; continue filling while tracks change.
2. Automated validation refs per slice:
   `cargo check -p core_mod_api` anchors are recorded for active slices; targeted long-run/determinism suites remain pending.
3. Runtime/demo capture refs per slice:
   pending capture artifact links for sustained runtime sessions and cache-wipe restore cycles.
4. Open gaps with explicit owners:
   should be tracked in checklist track logs (owner + next checkpoint date), not as freeform `TODO`.

## 12) Explicit decision checklist (please mark each)

1. Confirm authority id scheme:
   `usf.<domain>.<role>` with role tags
   (`definition`, `selection_state`, `runtime_state`, `realization_state`, `persistence_state`, etc.).
2. Confirm model-output intent family naming (`ChunkRealizationIntent`, etc.).
3. Confirm reconciliation naming (`ReconcileChunkRealizationArtifacts`, `*_reconcile_*`).
4. Confirm bridge ownership for realization channels under `rhai_binding/bridges`.
5. Confirm flattened top-level capability ids (`mesh`, `material`, `collider`, `audio`, `particles`, `trigger`, `simulation_service`) with no lane split.
6. Confirm workflow ownership direction: temporary macro coupling accepted, final ownership under `usf/chunk/*`.
7. Confirm DPT concept is retained as container abstraction (rename only).
8. Confirm `Dpt*` rename targets (`MetricContainerLayout`, `MetricContainerStore`, `MetricContainerRecord`).
9. Confirm schedule-entrypoint vocabulary migration and removal of `main(world)` fallback.
10. Confirm removal of remaining first-class `runtime/capability` architecture wording after migration.
11. Confirm removal of USF-side explicit bridge-application structs in favor of bridge registry + intent resolution.

Current pass note:
11 is now implemented for chunk realization path and phenomenon-model runtime shape
(USF no longer stores per-channel simulation/audio/particle/trigger model components;
channel payloads are resolved into `ChunkRealizationIntent.channel_payloads` and applied in bridge layer).
