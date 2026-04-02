# TEMP USF Refactor Execution Checklist (Locked)

Date: 2026-04-02  
Status: active execution checklist  
Authority order: user mental model > explicit USF contract docs > current code shape

## Mission

Deliver a minimal but real USF technical demo where:

1. Script entrypoints are the content authority surface.
2. Canonical persistence is only `Phenomenon` + `PhenomenonModel` (+ partition records when required).
3. Substrate/zone/manifestation are explicit derived runtime layers.
4. Engine Rust code is a reusable capability platform, not gameplay-content ownership.

## Non-Negotiable Invariants

1. USF remains entity-grounded and scale-explicit (71 scales).
2. Zone realization and chunk manifestation are one authority pipeline.
3. No cache (`chunk manifestation`, zone fields, metric snapshots) is canonical truth.
4. No primary-model shortcut as runtime authority.
5. Typed per-domain script ctx APIs remain mandatory.
6. No monolithic global substrate mutation bridge.
7. Sampler/categorizer logic is generic and config-bound, not hardcoded per content pack.

## Execution Tracks

## Track A: Entrypoint-First Runtime Platform

- [ ] Introduce explicit script-entrypoint registry contract by script type and function signature.
- [ ] Make typed ctx injection and lifecycle deterministic and testable per script type.
- [ ] Ensure all USF content declarations are loaded only through typed entrypoints.
- [ ] Hard-fail bootstrap on missing/invalid entrypoint signatures.
- [ ] Add diagnostics that point to exact script file + entrypoint + expected signature.

Exit condition:

- [ ] Bootstrapping a modpack with missing or invalid USF script entrypoints fails deterministically with actionable diagnostics.

## Track B: Canonical Entity Persistence Boundary

- [ ] Keep authoritative persistence restricted to phenomenon/model/partial-model records.
- [ ] Ensure chunk manifestation and zone caches are tagged and handled as derived cache only.
- [ ] Keep schema versioning + migration explicit for canonical records.
- [ ] Validate deterministic substrate rebuild from canonical model records.
- [ ] Remove stale code paths that can silently treat derived caches as source-of-truth.

Exit condition:

- [ ] Deleting all derived USF caches still recreates equivalent runtime state from canonical records alone.

## Track C: Zone-Orchestrated Manifestation Authority

- [ ] Keep zone realization as the only authority path that drives chunk manifestation binding.
- [ ] Ensure chunk manifestation does not run any alternate phenomenon selection path.
- [ ] Keep spawn/hydration/realization policy unified through one policy function path.
- [ ] Ensure parent-level boundary effects for child chunks flow through zone/phenomenon orchestration.
- [ ] Keep runtime grouping/component logic derived and disposable.

Exit condition:

- [ ] Zone updates deterministically reconcile manifestation bindings without fallback selectors.

## Track D: Adaptive Substrate Runtime

- [ ] Keep octree + polymorphic leaf substrate as derived adaptive state.
- [ ] Keep refine/coarsen transitions state-driven (energy/instability/gradient), not frame-decay driven.
- [ ] Keep storage policy and simulation transition policy separated.
- [ ] Ensure cross-chunk coupling uses explicit edge interfaces only.
- [ ] Avoid global dense-field assumptions in any hot path.

Exit condition:

- [ ] Broad-support phenomena can project and rebuild substrate incrementally without canonical dense arrays.

## Track E: Capability Platform Separation

- [ ] Move engine-level functionality behind explicit capability contracts.
- [ ] Keep phenomena/models minimal policy declarations, not heavy engine logic owners.
- [ ] Keep manifestation/simulation service contracts script-declarable and runtime-validated.
- [ ] Remove global behavior flags where model/capability-scoped authority is required.
- [ ] Maintain strict separation between content definition and engine execution kernels.

Exit condition:

- [ ] New manifestation outputs (mesh/collider/material/audio/particles) can be added via capability families without changing content ontology.

## Track F: Generic Sampler/Categorizer Pipeline

- [ ] Make sampler/categorizer runtime fully driven by scale + metric-set + ZLM compatibility contracts.
- [ ] Remove fixed-id assumptions from sampler/categorizer selection.
- [ ] Keep metric layout and zone classification deterministic and schema-validated.
- [ ] Keep generic algorithm core reusable across scales and modpacks.
- [ ] Make script-defined ids bind to validated runtime kernels.

Exit condition:

- [ ] A new scale can choose compatible sampler/categorizer ids without Rust-side hardcoded id branches.

## Track G: Naming and Module-Tree Reframe

- [ ] Remove or rename terms that imply derived caches are ontology authority.
- [ ] Keep module tree aligned with ownership boundaries (content declaration vs engine capability runtime).
- [ ] Keep singular/plural and concept naming consistent across docs, script surface, and Rust modules.
- [ ] Reframe legacy `mod_runtime` semantics toward explicit runtime capability packaging.
- [ ] Keep debug/test manifestation naming explicit and non-canonical.

Exit condition:

- [ ] A new contributor can locate ownership boundaries by names alone without reverse-engineering runtime behavior.

## Track H: Minimal Technical Demo

- [ ] Define a narrow but complete multi-scale content slice implemented fully via USF scripting entrypoints.
- [ ] Demonstrate deterministic load/persist/rebuild and zone-driven manifestation authority.
- [ ] Demonstrate at least one partitioned phenomenon across chunk boundaries.
- [ ] Demonstrate at least one non-mesh capability surface (for example audio or particle trigger contract).
- [ ] Keep Rust changes focused on reusable capability/platform extensions only.

Exit condition:

- [ ] Demo runs as a coherent proof that content can be authored by scripts while Rust remains platform/capability infrastructure.

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
