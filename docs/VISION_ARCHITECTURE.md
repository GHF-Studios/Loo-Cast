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
- zones / ZLMs
- scales
- phenomena / phenomenon models

This is the content-authoring and ontology declaration surface.

## 2) Canonical Runtime Persistence Authority (Entity-grounded)

Authoritative persisted runtime state:

- `Phenomenon`
- `PhenomenonModel`
- `PartialPhenomenonModel`

## 3) Derived Runtime Projections

Derived/execution-state layers:

- metric container sampling/classification products
- substrate state
- zone runtime/realization state (derived but entity-backed)
- chunk realization caches/artifacts

These are rebuildable and not canonical persistence truth.

## 4) Capability Platform (Rust/Bevy)

Rust owns runtime execution kernels, workflows, and bridge/channel output application:

- mesh/material/collider/audio/particles/trigger/simulation_service
- schedule orchestration
- validation and fail-fast enforcement

Bevy ECS is the internal execution substrate behind the USF-facing model.

## Composition Flow (Current Implementation)

1. `core_engine` builds app + plugin groups.
2. `CoreApiPluginGroup` wires subsystems (`input`, `player`, `render`, `usf`, `workflow`, `rhai_binding`, etc.).
3. Rhai boot script registers schedule entrypoints.
4. USF bootstrap loads typed scripts:
   - global contracts (`mod`, `modpack`)
   - package-scoped contracts (`metric`, `zone`, `metric_set`, `zlm`, `scale`, `phenomenon`, `phenomenon_model`)
5. Active modpack is selected from config and composed deterministically.
6. Canonical runtime concept registries are materialized from the composed concept catalog:
   - concept catalog
   - runtime concept query view (`UsfRuntimeConceptView`)
   - mod/modpack/manifest/contribution registries
   - metric/metric_set/scale registries
7. Runtime systems execute domain pipelines (substrate, zone, phenomenon, realization), including optional top-down bootstrap worldgen descent control.

## Core Boundaries

1. Scripts configure content and policy contracts.
2. Engine code implements capability kernels and orchestrates execution.
3. Zones are a derived abstraction layer with runtime agency and entity representation; they are not canonical persisted authority.
4. Zone-driven realization is the primary driver for ensuring concrete phenomenon proxy presence.
5. Phenomena orchestrate cross-scale model stacks; phenomenon models carry scale-specific runtime behavior.
6. Phenomena/phenomenon-models influence zones indirectly through substrate/metric change, not direct zone truth mutation.
7. Zone assignment is exclusive per location/scale; unclassified locations are mapped to a sentinel zone.
8. Direct reconciliation vs intent-queue realization is an implementation concern; semantic authority remains unchanged.
9. Content should not depend on hidden fallback behavior.
10. Mismatches between intended architecture and code must be made explicit and resolved with the user before further implementation.

## Status Note

This architecture is still evolving. When behavior differs from this intent, treat the user direction as authoritative and update this document with the resolved decision.
