# Universal Simulation Framework (USF) — Entity-Grounded Runtime Contract

Date: 2026-04-02
Scope: current `core_mod_api::usf` architecture contract

## Core Invariants

1. Canonical persistence authority is entity-grounded:
   - `Phenomenon`
   - `PhenomenonModel`
   - `PartialPhenomenonModel`
2. Metrics, substrate leaves, zone maps, and chunk realization records are runtime/cache state.
3. Runtime model resolution is explicit by `(phenomenon_id, scale_index)`. There is no primary-model authority path.
4. Zone classification/selection is authority for chunk-level phenomenon selection.
5. Substrate stores adaptive geometry and leaf containers only; transition policy is simulation-owned.

## Ownership Boundaries

### Phenomenon Layer (Canonical)

- `Phenomenon`: global identity + kind.
- `PhenomenonModel`: scale-specific semantic state/projection and support topology.
- `PartialPhenomenonModel`: partition member state keyed by `(phenomenon_id, scale, chunk_coord)`.

This layer is the only authoritative world truth in USF runtime persistence.

### Substrate Layer (Derived)

- Per loaded chunk: adaptive octree + polymorphic leaf representation.
- Leaf forms currently include:
  - `Uniform`
  - `DenseBrick`
  - `PaletteBrick`
  - `Gradient`
  - `Statistical`
  - `Heightfield`
  - `DelegatedToPhenomenon`
- Substrate summary provides projection signatures, zone classification inputs, and chunk-edge interfaces.
- Cross-chunk coupling is explicit via edge interfaces; no implicit global dense-field coupling is treated as canonical.

### Zone Layer (Derived Classification)

- Zones are classifier outputs over substrate summaries.
- Zone runtime computes stable region/grouping and parent relations.
- Zone realization decides which phenomenon entity is active for a zone and emits spawn/despawn lifecycle events.
- Zone lifecycle is the single authority path for chunk realization intent resolution.

### Chunk Realization Layer (Derived Output Application)

- Chunk realization is non-authoritative cache/projection state.
- Runtime intents are derived from zone realization + substrate summaries + runtime phenomenon-model entities/support.
- Engine-level output application (mesh/material/collider/audio/particles/trigger/simulation_service) is isolated in bridge registrations.
- Instance culling is runtime-configured presentation behavior; mesh/material/collider/audio/particles/trigger/simulation_service are model-emitted intent payloads.

## Scale Contract

- USF defines 71 scales (`Scale::SCALE_LEVEL_COUNT = 71`).
- Selection and runtime behavior are scale-explicit.
- Zone realization window around active scale is configurable:
  - `usf/zone/realization/levels_above_active`
  - `usf/zone/realization/levels_below_active`

## Runtime Pipeline (Current)

1. Load/restore canonical phenomenon/model entities.
2. Build/rebuild affected chunk substrate from model projections + support contracts.
3. Derive zone runtime state from substrate summaries.
4. Reconcile zone realization (spawn/despawn phenomenon containers by zone policy).
5. Derive chunk realization intents from zone realization + substrate authority.
6. Build realization cache artifacts and apply bridge-registered outputs to chunk instances.

## Persistence Contract

Authoritative:
- phenomenon record
- phenomenon-model record
- partial phenomenon-model record

Runtime cache:
- chunk realization cache records (`cache_authority = "runtime_cache"`)

Rule:
- Cached arrays/records are never canonical truth and may be dropped/rebuilt.

## Script Surface Contract

- Domain-specific typed script contexts remain mandatory:
  - mod
  - modpack
  - metric
  - metric_set
  - zone
  - zlm
  - phenomenon
  - phenomenon_model
  - scale
- Single-entity-per-file descriptors remain the authoring model.
- No monolithic global substrate mutation API is exposed in Rhai.

## Explicit Non-Goals

- Backward compatibility with legacy primary-model assumptions.
- Treating chunk cache/surface/realization records as ontology.
- Reintroducing dense authoritative world fields as persistence truth.
