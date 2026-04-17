# Vision & Architecture

Purpose: shared conceptual model for what Loo Cast is, what USF is, and how the implementation is layered.

## Vision (Owner Direction)

Loo Cast targets an ontology-first simulation model where content is not hand-scripted as isolated game objects, but recognized, maintained, and evolved as structured USF concepts across scales.  
The ambition is reverse-emergent/systemic behavior: higher-level structure drives lower-level manifestation, while detailed state remains classifiable back into stable semantic concepts.

## Layer Model

## 1) Script-Configured Canonical Definitions (USF-facing)

Declared primarily through typed Rhai entrypoints:

- mods / modpacks
- metrics / metric sets
- scales
- phenomenon realizers
- phenomena / phenomenon models
- capability usage contracts

This is the content-authoring and ontology declaration surface.

## 2) Canonical Runtime Persistence Authority (Entity-grounded)

Authoritative persisted runtime state:

- metric state per loaded scale/chunk
- `Phenomenon`
- `PhenomenonModel`
- `PartialPhenomenonModel`

## 3) Derived Runtime Projections

Derived/execution-state layers:

- substrate summaries and indices
- realization queues and decision artifacts
- capability runtime output state
- chunk realization caches/artifacts

These are rebuildable and not canonical persistence truth.

## 4) Capability Platform (Rust/Bevy)

Rust owns runtime execution kernels, workflows, and capability-channel backends:

- mesh/material/collider/audio/particles/trigger/simulation_service
- schedule orchestration
- validation and fail-fast enforcement

Bevy ECS is the internal execution substrate behind the USF-facing model.

## Composition Flow (Target Contract)

1. `core_engine` builds app + plugin groups.
2. `CoreApiPluginGroup` wires subsystems (`input`, `player`, `render`, `usf`, `workflow`, `rhai_binding`, etc.).
3. Rhai boot script registers schedule entrypoints.
4. USF bootstrap loads typed scripts:
   - global contracts (`mod`, `modpack`)
   - package-scoped contracts (`metric`, `metric_set`, `scale`, `phenomenon_realizer`, `phenomenon`, `phenomenon_model`)
5. Active modpack is selected from config and composed deterministically.
6. Canonical runtime concept registries are materialized from the composed concept catalog:
   - concept catalog
   - runtime concept query view (`UsfRuntimeConceptView`)
   - mod/modpack/manifest/contribution registries
   - metric/metric_set/scale registries
   - phenomenon realizer registry
   - phenomenon definition/model registries
7. Runtime systems execute deterministic domain pipelines (substrate, realization, phenomenon, capability commit), including optional top-down bootstrap worldgen descent control.

## Core Boundaries

1. Scripts configure content and policy contracts.
2. Engine code implements capability kernels and orchestrates execution.
3. Phenomenon realizers are canonical scale-level realization policy; they are not replaced by derived runtime caches.
4. Realization decisions are driven by metrics/substrate + realizer contracts.
5. Phenomena orchestrate cross-scale model stacks; phenomenon models carry scale-specific runtime behavior.
6. Capability channels are part of the ctx graph and scripts write through intent emission only.
7. Cross-scale state/logic access must pass through explicit cross-scale gateways.
8. Direct reconciliation vs intent-queue realization is an implementation concern; semantic authority remains unchanged.
9. Content should not depend on hidden fallback behavior.
10. Mismatches between intended architecture and code must be made explicit and resolved with the user before further implementation.

## Status Note

This architecture is still evolving.  
Current code may still contain pre-target naming and flows; owner direction in this document is authoritative for forward design.
