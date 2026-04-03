# TEMP USF Refactor Execution Checklist (Locked)

Date: 2026-04-02  
Status: active execution checklist  
Authority order: user mental model > explicit USF contract docs > current code shape

## Mission

Deliver a minimal but real USF technical demo where:

1. Script entrypoints are the content authority surface.
2. Canonical persistence is only `Phenomenon` + `PhenomenonModel` (+ partition records when required).
3. Substrate/zone/chunk-realization are explicit derived runtime layers.
4. Engine Rust code is a reusable capability platform, not gameplay-content ownership.

## Reality Check (2026-04-03)

This checklist is execution-oriented, but current checkmarks can overstate end-state confidence.
Treat every checked item as "implemented in code path(s) tested so far", not "architecturally complete".

Re-opened strategic gaps that still block full confidence:

- [ ] End-to-end proof that script entrypoints can author complex multi-scale content without Rust-side content glue.
- [ ] Full ownership/tree alignment across all USF-adjacent modules (not only chunk-realization runtime slices).
- [ ] Deterministic cross-scale continuity under real runtime load (zone realization, child-model spawning, chunk-realization binding together).
- [ ] Capability catalog breadth for script-first development (ECS/sysparam/message/resource/query families still incomplete).
- [ ] Canonical persistence/migration hardening against schema drift in longer-lived save data.

## Non-Negotiable Invariants

1. USF remains entity-grounded and scale-explicit (71 scales).
2. Zone realization and chunk realization are one authority pipeline.
3. No cache (`chunk realization`, zone fields, metric snapshots) is canonical truth.
4. No primary-model shortcut as runtime authority.
5. Typed per-domain script ctx APIs remain mandatory.
6. No monolithic global substrate mutation bridge.
7. Sampler/categorizer logic is generic and config-bound, not hardcoded per content pack.

## Execution Tracks

## Track A: Entrypoint-First Runtime Platform

- [x] Introduce explicit script-entrypoint registry contract by script type and function signature.
- [x] Make typed ctx injection and lifecycle deterministic and testable per script type.
- [x] Ensure all USF content declarations are loaded only through typed entrypoints.
- [x] Hard-fail bootstrap on missing/invalid entrypoint signatures.
- [x] Add diagnostics that point to exact script file + entrypoint + expected signature.

Exit condition:

- [x] Bootstrapping a modpack with missing or invalid USF script entrypoints fails deterministically with actionable diagnostics.

## Track B: Canonical Entity Persistence Boundary

- [x] Keep authoritative persistence restricted to phenomenon/model/partial-model records.
- [x] Ensure chunk realization and zone caches are tagged and handled as derived cache only.
- [x] Keep schema versioning + migration explicit for canonical records.
- [x] Validate deterministic substrate rebuild from canonical model records.
- [x] Remove stale code paths that can silently treat derived caches as source-of-truth.

Exit condition:

- [x] Deleting all derived USF caches still recreates equivalent runtime state from canonical records alone.

## Track C: Zone-Orchestrated Chunk Realization Authority

- [x] Keep zone realization as the only authority path that drives chunk realization intent binding.
- [x] Ensure chunk realization does not run any alternate phenomenon selection path.
- [x] Keep spawn/reconcile/realization policy unified through one policy function path.
- [x] Ensure parent-level boundary effects for child chunks flow through zone/phenomenon orchestration.
- [x] Keep runtime grouping/component logic derived and disposable.

Exit condition:

- [x] Zone updates deterministically reconcile chunk realization intents without fallback selectors.

## Track D: Adaptive Substrate Runtime

- [x] Keep octree + polymorphic leaf substrate as derived adaptive state.
- [x] Keep refine/coarsen transitions state-driven (energy/instability/gradient), not frame-decay driven.
- [x] Keep storage policy and simulation transition policy separated.
- [x] Ensure cross-chunk coupling uses explicit edge interfaces only.
- [x] Avoid global dense-field assumptions in any hot path.

Exit condition:

- [x] Broad-support phenomena can project and rebuild substrate incrementally without canonical dense arrays.

## Track E: Capability Platform Separation

- [x] Move engine-level functionality behind explicit capability contracts.
- [x] Keep phenomena/models minimal policy declarations, not heavy engine logic owners.
- [x] Keep realization/simulation service contracts script-declarable and runtime-validated.
- [x] Remove global behavior flags where model/capability-scoped authority is required.
- [x] Maintain strict separation between content definition and engine execution kernels.

Exit condition:

- [x] New realization outputs (mesh/collider/material/audio/particles) can be added via capability families without changing content ontology.

## Track F: Generic Sampler/Categorizer Pipeline

- [x] Make sampler/categorizer runtime fully driven by scale + metric-set + ZLM compatibility contracts.
- [x] Remove fixed-id assumptions from sampler/categorizer selection.
- [x] Keep metric layout and zone classification deterministic and schema-validated.
- [x] Keep generic algorithm core reusable across scales and modpacks.
- [x] Make script-defined ids bind to validated runtime kernels.

Exit condition:

- [x] A new scale can choose compatible sampler/categorizer ids without Rust-side hardcoded id branches.

## Track G: Naming and Module-Tree Reframe

- [x] Remove or rename terms that imply derived caches are ontology authority.
- [x] Keep module tree aligned with ownership boundaries (content declaration vs engine capability runtime).
- [ ] Keep singular/plural and concept naming consistent across docs, script surface, and Rust modules.
- [x] Reframe legacy `mod_runtime` semantics toward explicit runtime capability packaging. (`usf/runtime/*` removed for this pipeline; realization channels live under `rhai_binding/bridges`.)
- [x] Keep debug/test realization naming explicit and non-canonical.

Exit condition:

- [ ] A new contributor can locate ownership boundaries by names alone without reverse-engineering runtime behavior.

## Track H: Minimal Technical Demo

- [x] Define a narrow but complete multi-scale content slice implemented fully via USF scripting entrypoints.
- [x] Demonstrate deterministic load/persist/rebuild and zone-driven realization authority.
- [x] Demonstrate at least one partitioned phenomenon across chunk boundaries under runtime load.
- [x] Demonstrate at least one non-mesh capability surface (for example audio or particle trigger contract) with observable in-engine effect.
- [x] Keep Rust changes focused on reusable capability/platform extensions only.

Exit condition:

- [x] Demo runs as a coherent proof that content can be authored by scripts while Rust remains platform/capability infrastructure.

## Major Checkpoint Validation

Run heavy validation only at major checkpoints:

1. `cargo fmt --all`
2. `cargo check -p core_mod_api`
3. `cargo test -p core_mod_api`

## Completion Criteria

Checklist is complete when:

1. All track exit conditions are met.
2. The minimal technical demo is stable and deterministic on canonical persistence boundaries.
3. Remaining TODOs are optional polish, not architectural blockers.
