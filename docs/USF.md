# Universal Simulation Framework (USF) — Entity-Grounded Runtime Contract

Date: 2026-04-02
Scope: current `core_mod_api::usf` architecture contract

## Core Invariants

1. Canonical persistence authority is entity-grounded:
   - `Phenomenon`
   - `PhenomenonModel`
   - `PartialPhenomenaModel`
2. Metrics, substrate leaves, zone maps, and manifestation records are derived runtime/cache state.
3. Runtime model resolution is explicit by `(phenomenon_id, scale_index)`. There is no primary-model authority path.
4. Zone realization is manifestation authority for chunk-level phenomenon binding.
5. Substrate stores adaptive geometry and leaf containers only; transition policy is simulation-owned.

## Ownership Boundaries

### Phenomenon Layer (Canonical)

- `Phenomenon`: global identity + kind.
- `PhenomenonModel`: scale-specific manifestation contract and support topology.
- `PartialPhenomenaModel`: partition member state keyed by `(phenomenon_id, scale, chunk_coord)`.

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
- Zone lifecycle is the single authority path for chunk manifestation binding.

### Manifestation Layer (Derived Capability Application)

- Chunk manifestation is non-authoritative cache/projection state.
- Runtime bindings are derived from zone realization + substrate summaries + phenomenon model contracts.
- Engine-level capability application (mesh/material/collider) is isolated in runtime capability code.
- Runtime toggles (`attach_meshes`, `enable_instance_culling`) control presentation behavior; collider attachment is model-scoped capability intent.

## Scale Contract

- USF defines 71 scales (`Scale::SCALE_LEVEL_COUNT = 71`).
- Selection and runtime behavior are scale-explicit.
- Zone realization window around active scale is configurable:
  - `usf/zone/realization/levels_above_active`
  - `usf/zone/realization/levels_below_active`

## Runtime Pipeline (Current)

1. Load/hydrate canonical phenomenon/model entities.
2. Build/rebuild affected chunk substrate from model projections + support contracts.
3. Derive zone runtime state from substrate summaries.
4. Reconcile zone realization (spawn/despawn phenomenon containers by zone policy).
5. Derive chunk manifestation bindings from zone realization + substrate authority.
6. Build manifestation cache artifacts and apply engine capabilities to chunk instances.

## Persistence Contract

Authoritative:
- phenomenon record
- phenomenon-model record
- partial phenomenon-model record

Derived cache:
- chunk manifestation cache records (`cache_authority = "derived_cache"`)

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
- Treating chunk cache/surface/manifestation records as ontology.
- Reintroducing dense authoritative world fields as persistence truth.
