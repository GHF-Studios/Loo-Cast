# TEMP USF Refactor Execution Checklist (Locked)

Date: 2026-04-03  
Status: active execution checklist (status model hardened)  
Authority order: user mental model > explicit USF contract docs > current code shape

## Status Model (Mandatory)

All progress uses one of these states:

1. `implemented`: main code path exists and is wired.
2. `partially_validated`: implementation exists and has targeted proof, but coverage is incomplete.
3. `architecturally_complete`: implementation + deterministic validation + naming/tree alignment + no known blocker in that track.

Promotion rule:

1. No track/exit condition is promoted to `architecturally_complete` without evidence entries.
2. Evidence must include code anchors, automated validation refs, and runtime/demo proof notes.

## Mission

Deliver a minimal but real USF technical demo where:

1. Script entrypoints are the content authority surface.
2. Canonical persistence is only `Phenomenon` + `PhenomenonModel` (+ partition records when required).
3. Substrate/zone/chunk-realization are explicit derived runtime layers.
4. Engine Rust code is reusable platform/bridge infrastructure, not gameplay-content ownership.

## Reality Check (2026-04-03)

Strategic gaps still blocking full confidence:

1. End-to-end proof that script entrypoints can author complex multi-scale content without Rust-side content glue.
2. Full ownership/tree alignment across all USF-adjacent modules (not only chunk-realization slices).
3. Deterministic cross-scale continuity under runtime load (zone classification, model selection, child-model spawning, chunk realization).
4. Capability catalog breadth for script-first development (ECS/sysparam/message/resource/query families still incomplete).
5. Canonical persistence/migration hardening against schema drift in longer-lived save data.

## Non-Negotiable Invariants

1. USF remains entity-grounded and scale-explicit (71 scales).
2. Authority pipeline is unified, but roles stay distinct:
   zones classify/select, phenomena+models own semantic state, chunk realization reconciles outputs.
3. No cache (`chunk realization`, zone fields, metric snapshots) is canonical truth.
4. No primary-model shortcut as runtime authority.
5. Typed per-domain script ctx APIs remain mandatory.
6. No monolithic global substrate mutation bridge.
7. Sampler/categorizer logic is generic and config-bound, not hardcoded per content pack.
8. Engine output execution is bridge-registered Rust functionality; USF stores intents/state, not bridge execution logic.

## Execution Tracks

## Track A: Entrypoint-First Runtime Platform

Track state: `partially_validated`

- `[implemented]` Explicit script-entrypoint registry contract by script type and function signature.
- `[implemented]` Typed ctx injection and lifecycle deterministic at bootstrap/runtime path.
- `[implemented]` USF content declarations loaded through typed entrypoints.
- `[implemented]` Bootstrap hard-fails on missing/invalid signatures.
- `[implemented]` Bootstrap compiles deterministic entrypoint descriptors (`file + script_type + owner + entrypoint + expected signature`) and records execution in `USF_BOOTSTRAP_REPORT`.
- `[partially_validated]` Diagnostics quality for exact file + entrypoint + expected signature/error-codes.

Exit condition:

- `[partially_validated]` Invalid entrypoints fail deterministically with actionable diagnostics.

Evidence log (required for `architecturally_complete`):

- Code anchors:
  `core_mod_api/src/rhai_binding/engine/bootstrap.rs`
  (`UsfEntrypointDescriptor`, `collect_global_entrypoint_descriptors`, `collect_package_entrypoint_descriptors`, `run_usf_content_bootstrap`),
  `core_mod_api/src/rhai_binding/engine/statics.rs`
  (`ScriptUsfBootstrapReport`, `ScriptUsfEntrypointExecutionRecord`, `USF_BOOTSTRAP_REPORT`),
  `core_mod_api/src/rhai_binding/engine/schedule_entrypoint.rs`
  (bootstrap counters exposed in schedule entrypoint params).
- Automated validation:
  `cargo check -p core_mod_api` passed on 2026-04-03 after descriptor/report integration.
- Runtime/demo proof:
  pending runtime capture using schedule entrypoint bootstrap telemetry fields.
- Open gaps: bootstrap error-code normalization + stress tests across large modpack/script trees.

## Track B: Canonical Entity Persistence Boundary

Track state: `partially_validated`

- `[implemented]` Authoritative persistence restricted to phenomenon/model/partial-model records.
- `[implemented]` Chunk realization and zone caches treated as runtime cache only.
- `[implemented]` Schema versioning + migration hooks explicit for canonical records.
- `[partially_validated]` Deterministic substrate rebuild from canonical model records.
- `[implemented]` Stale source-of-truth cache paths removed from primary flow.

Exit condition:

- `[partially_validated]` Deleting runtime caches recreates equivalent runtime state from canonical records.

Evidence log:

- Code anchors:
  `core_mod_api/src/usf/phenomenon/persistence.rs`
  (canonical record schemas + versions),
  `core_mod_api/src/usf/phenomenon/persistence_runtime.rs`
  (queued/batched canonical persistence flush),
  `core_mod_api/src/usf/substrate/mod.rs`
  (`rebuild_substrate_from_persisted_models`, deterministic partial-record matching),
  `core_mod_api/src/usf/chunk/realization/field.rs`
  (runtime-cache schema/version + `cache_authority = runtime_cache` contract).
- Automated validation:
  deterministic substrate/persistence tests are present in
  `core_mod_api/src/usf/substrate/mod.rs` (`substrate_rebuild_from_persisted_models_is_deterministic`,
  partition-key/order tests); broader persistence round-trip suite still pending checkpoint run.
- Runtime/demo proof:
  pending run capture proving cache wipe + restore equivalence from canonical records only.
- Open gaps: migration drift and long-lived save compatibility matrix.

## Track C: Zone Classification + Model-Driven Chunk Realization

Track state: `partially_validated`

- `[implemented]` Zones are the classifier/selector authority for realization candidates.
- `[implemented]` Chunk realization does not use alternate fallback selectors.
- `[implemented]` Spawn/reconcile/realization path shares one policy function chain.
- `[partially_validated]` Parent-level effects into child chunks run through zone/model orchestration.
- `[implemented]` Runtime grouping/component logic remains derived/disposable.

Exit condition:

- `[partially_validated]` Zone updates deterministically reconcile chunk realization intents without fallback selectors.

Evidence log:

- Code anchors:
  `core_mod_api/src/usf/zone/policy.rs` (zone selection authority),
  `core_mod_api/src/usf/zone/systems.rs` (zone realization lifecycle),
  `core_mod_api/src/usf/chunk/realization/intent.rs`
  (chunk intent resolved from zone realization + runtime phenomenon-model entities at scale, then validated against selected model id contracts).
- Automated validation:
  `cargo check -p core_mod_api` passed on 2026-04-03 after selected-model-id + intent path updates.
- Runtime/demo proof:
  pending targeted run capture after next integration slice.
- Open gaps: cross-scale continuity under sustained runtime load.

## Track D: Adaptive Substrate Runtime

Track state: `partially_validated`

- `[implemented]` Octree + polymorphic leaf substrate is treated as derived adaptive runtime state.
- `[implemented]` Refine/coarsen transitions are state-driven, not frame-decay-driven.
- `[implemented]` Storage policy and simulation transition policy are separated.
- `[partially_validated]` Cross-chunk coupling uses explicit edge interfaces only.
- `[implemented]` No global dense-field assumption in primary hot paths.

Exit condition:

- `[partially_validated]` Broad-support phenomena project/rebuild substrate without canonical dense arrays.

Evidence log:

- Code anchors:
  `core_mod_api/src/usf/substrate/mod.rs`
  (adaptive leaf transitions, contribution aggregation, deterministic rebuild/runtime composition),
  `core_mod_api/src/usf/substrate/policy.rs`
  (state-driven substrate policy boundaries),
  `core_mod_api/src/usf/phenomenon/partition_runtime.rs`
  (explicit chunk-edge partition member topology + coupling contracts).
- Automated validation:
  substrate determinism tests exist in `core_mod_api/src/usf/substrate/mod.rs`;
  high-load profiling/continuity coverage remains pending.
- Runtime/demo proof:
  pending sustained-load run capture for cross-chunk boundary exchange and stabilization behavior.
- Open gaps: high-load behavior and boundary exchange profiling.

## Track E: Bridge Registration + Realization Intent Schema

Track state: `partially_validated`

- `[implemented]` Engine output application moved behind explicit bridge registrations.
- `[implemented]` Phenomena/models remain semantic declarations/state, not heavy engine logic owners.
- `[partially_validated]` Realization intent schema is script-declarable and runtime-validated.
- `[implemented]` Global behavior flags removed where model-scoped intent authority is required.
- `[implemented]` Content definitions remain separated from engine execution kernels.

Exit condition:

- `[partially_validated]` New output channels (mesh/collider/material/audio/particles/trigger/simulation_service) can be added via bridge registrations without ontology changes.

Evidence log:

- Code anchors:
  `core_mod_api/src/rhai_binding/bridges/domains/core_mod_api/usf/output_channels.rs`
  (channel execution registration + payload apply, including first-class `material` + `simulation_service` channels and no separate enabled-channel gate model),
  `core_mod_api/src/usf/chunk/realization/runtime.rs`
  (registration validation + selected-model-id cache keying),
  `core_mod_api/src/usf/chunk/realization/intent.rs`
  (separate mesh/material/simulation_service channel payload emission from model definitions),
  `core_mod_api/src/usf/chunk/realization/reconcile_workflow.rs`
  (intent->artifact reconcile path),
  `core_mod_api/src/usf/chunk/realization/field.rs`
  (record schema v3 with selected_model_id),
  `core_mod_api/src/rhai_binding/engine/schedule_entrypoint.rs`
  (`chunk_realization_material_instances` + `chunk_realization_simulation_services` telemetry exposure).
- Automated validation:
  `cargo check -p core_mod_api` passed on 2026-04-03.
- Runtime/demo proof:
  pending targeted runtime verification for channel payload families after registry rename.
- Open gaps: channel-family expansion proof beyond current demo pathways.

## Track F: Generic Sampler/Categorizer Pipeline

Track state: `partially_validated`

- `[implemented]` Sampler/categorizer driven by scale + metric-set + ZLM compatibility.
- `[implemented]` Fixed-id assumptions removed from selection path.
- `[implemented]` Metric-container layout and zone classification deterministic/schema-validated.
- `[implemented]` Runtime-supported kernel-id guardrails exist (`metric_sampler.kernel.*` / `metric_categorizer.kernel.*`), preventing bootstrap-valid but runtime-unsupported ids.
- `[implemented]` Generic algorithm core reusable across scales/mod packs.
- `[partially_validated]` Script-defined ids bind to validated runtime kernels end-to-end.

Exit condition:

- `[partially_validated]` New scales choose compatible sampler/categorizer ids without Rust hardcoded id branches.

Evidence log:

- Code anchors:
  `core_mod_api/src/usf/mod_packs/mod.rs`
  (`UsfScaleDefinition` explicit `metric_sampler_id` + `metric_categorizer_id` bindings + validation),
  `core_mod_api/src/usf/metric_container/mod.rs`
  (sampler dispatch via modpack/scale contract),
  `core_mod_api/src/usf/zlm/mod.rs`
  (categorizer-driven classification and zlm lookup path),
  `core_mod_api/src/rhai_binding/engine/bootstrap.rs`
  (`register_metric_sampler_kernel_id`, `register_metric_categorizer_kernel_id`, scale binding validation).
- Automated validation:
  targeted tests validate custom runtime-supported kernel ids and reject runtime-unsupported registered ids
  (`active_modpack_accepts_runtime_supported_custom_kernel_ids`,
  `active_modpack_rejects_registered_but_runtime_unsupported_sampler_id`,
  `active_modpack_rejects_registered_but_runtime_unsupported_categorizer_id`,
  `ensure_chunk_for_scale_accepts_custom_sampler_kernel_id_prefix`,
  `classify_for_scale_accepts_custom_categorizer_kernel_id_prefix`);
  broader malformed-contract coverage across large script trees remains pending.
- Runtime/demo proof:
  pending run capture showing different scale kernel ids selected without Rust branching.
- Open gaps: wider script-first coverage and failure-mode diagnostics.

## Track G: Naming and Module-Tree Reframe

Track state: `partially_validated`

- `[implemented]` Terms implying cache-as-authority removed on core realization path.
- `[partially_validated]` Module tree aligned to ownership boundaries beyond chunk realization slices.
- `[implemented]` Legacy `mod_runtime` split reframed into bridge/runtime packaging in active path.
- `[partially_validated]` Singular/plural and concept naming consistency across docs/scripts/Rust modules.
- `[implemented]` Debug/test realization naming explicit and non-canonical.

Exit condition:

- `[partially_validated]` New contributor can locate ownership boundaries by names alone.

Evidence log:

- Code anchors:
  `core_mod_api/src/usf/chunk/realization/intent.rs`
  (`ChunkRealizationIntentGrace`, `IntentSync*` naming pass + intent authority sync),
  `core_mod_api/src/usf/chunk/realization/runtime.rs` (validation renamed to channel registrations),
  `core_mod_api/src/usf/authority.rs`
  (`USF_DOMAIN_CHUNK_REALIZATION_STATE = "usf.chunk_realization.runtime_state"`),
  `core_mod_api/src/rhai_binding/bridges/domains/core_mod_api/usf/output_channels.rs`
  (`OutputChannelExecutionRegistration`, `has_registration`),
  `core_mod_api/src/usf/phenomenon/types.rs`
  (`PhenomenonOutputFieldSpec`),
  `core_mod_api/src/usf/phenomenon/components.rs`
  (`PhenomenonModelProjection` with `spec` field),
  `core_mod_api/src/usf/phenomenon/systems.rs`
  (`reconcile_zone_realization_model_state_system`),
  `core_mod_api/src/usf/phenomenon/resources.rs`
  (`projection_spec_*` registry APIs),
  `core_mod_api/src/rhai_binding/engine/bootstrap.rs`
  (`set_projection_spec` and `set_output_*` script APIs),
  `docs/Scripting.md`, `docs/USF.md`, `docs/usf_entity_grounded_refactor_note.md` (terminology pass).
- Automated validation:
  `cargo check -p core_mod_api` passed on 2026-04-03 after naming refactor.
- Runtime/demo proof:
  pending follow-up run to confirm no naming-drift regressions in script diagnostics output.
- Open gaps: remaining naming drift and parallel ownership signals.

## Track H: Minimal Technical Demo

Track state: `partially_validated`

- `[implemented]` Narrow multi-scale content slice exists via USF scripting entrypoints.
- `[partially_validated]` Deterministic load/persist/rebuild and zone-driven realization authority under repeated runs.
- `[partially_validated]` At least one partitioned phenomenon demonstrated under runtime load.
- `[partially_validated]` At least one non-mesh output channel has observable runtime effect.
- `[implemented]` Rust changes focused on reusable platform/bridge extensions.

Exit condition:

- `[partially_validated]` Demo currently demonstrates architecture direction, but still lacks full determinism/load confidence to count as `architecturally_complete`.

Evidence log:

- Code anchors:
  `core_mod/assets/scripts/usf/*` (typed script-entrypoint demo content),
  `core_mod_api/src/usf/chunk/realization/intent.rs`
  (zone/model-driven intent resolution),
  `core_mod_api/src/rhai_binding/bridges/domains/core_mod_api/usf/output_channels.rs`
  (channel application/telemetry including non-mesh families),
  `core_mod_api/src/usf/substrate/mod.rs`
  (deterministic rebuild from persisted canonical records),
  `core_mod_api/src/rhai_binding/engine/schedule_entrypoint.rs`
  (bootstrap + channel telemetry surface for demo observability).
- Automated validation:
  `cargo check -p core_mod_api` passed after recent integration slices; full demo-suite and long-session tests pending major checkpoint.
- Runtime/demo proof:
  demo boots and renders after refactor slices, but deterministic long-session capture and repeated restore-cycle capture are still pending.
- Open gaps: determinism runs, long-session stability, and scale-transition continuity.

## Major Checkpoint Validation

Run heavy validation only at major checkpoints:

1. `cargo fmt --all`
2. `cargo check -p core_mod_api`
3. `cargo test -p core_mod_api`

## Completion Criteria

Checklist is complete only when:

1. Every track exit condition is `architecturally_complete`.
2. Every track evidence log is populated with concrete refs (code/tests/runtime proof).
3. Minimal technical demo is stable and deterministic on canonical persistence boundaries.
4. Remaining unresolved evidence items are explicitly tracked with owner + next checkpoint date; none are left as silent `TODO`.
