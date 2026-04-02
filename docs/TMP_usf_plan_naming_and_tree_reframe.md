# TEMP Plan: Naming and Module-Tree Reframe

Date: 2026-04-02  
Status: planned execution track

## Goal

Align naming and project/module structure with ownership reality so the architecture is obvious from the tree and identifiers.

## Target End State

1. Names do not imply caches are canonical.
2. Module paths map cleanly to ontology vs derived runtime vs capability platform.
3. Legacy terms that encode old dense-metric assumptions are removed.
4. Script tree and Rust module tree use consistent USF concept names.

## Naming Rules

1. Canonical entities use `phenomenon` / `phenomenon_model` / `partial_phenomenon_model`.
2. Derived runtime layers use `substrate`, `zone_runtime`, `manifestation_runtime`.
3. Capability execution modules use `runtime/capability/*`.
4. Debug visualization content names include explicit `debug` or `demo` prefix.
5. Avoid terms that blur identity vs projection (for example old `chunk_surface` semantics).

## Tree Reframe (Planned Shape)

1. `usf/content/*`
   - script-registered definitions and immutable registries
2. `usf/ontology/*`
   - canonical entity components/resources/persistence
3. `usf/substrate/*`
   - adaptive spatial derived runtime
4. `usf/zone/*`
   - derived classification + realization orchestration
5. `usf/runtime/capability/*`
   - engine-level execution kernels
6. `usf/runtime/manifestation/*`
   - derived binding/artifact orchestration

## Execution Steps

1. Inventory and mapping
   - map current modules and identifiers to target ownership buckets.
   - list misnamed files/types/resources with replacement names.
2. Rename pass
   - rename paths/types/functions/resources/events to target vocabulary.
   - keep breaking cuts clean; avoid long-lived alias shims.
3. Reference rewiring
   - update imports, script docs, and config keys to new names.
   - remove obsolete alias exports as soon as rewired.
4. Documentation lock-in
   - update docs to use only target names.
   - add a short migration note for removed legacy terms.

## Acceptance Criteria

1. New contributor can locate canonical authority modules without reading implementation.
2. `mod_runtime`/`chunk_surface` style drift is eliminated or strictly debug-scoped.
3. Script-facing names match Rust module intent and contract docs.

## Risks and Mitigations

1. Risk: broad rename churn creates temporary compile instability.
   - Mitigation: execute in bounded rename slices and remove shims quickly.
2. Risk: docs drift behind code.
   - Mitigation: update docs in same PR as rename slices.
3. Risk: accidental semantic changes hidden by rename.
   - Mitigation: separate pure rename commits from behavior changes where possible.

## Sequencing Notes

1. Execute early enough to prevent further semantic drift.
2. Finish before final tech-demo polish to avoid double-work on content naming.
