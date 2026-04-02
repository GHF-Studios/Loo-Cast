# USF Entity-Grounded Breaking Refactor Note

Date: 2026-04-02
Scope: `core_mod_api::usf` runtime/persistence authority cut

## Intentionally Broken Legacy Behavior

- Chunk persistence files are no longer treated as canonical world truth.
- The old `primary` phenomenon model runtime assumption has been removed from model realization.
- Chunk manifestation hydration no longer uses direct `supports.first()` behavior.
- Zone runtime no longer classifies chunks from `UsfWorld` procedural sampling as authority.
- USF runtime naming moved from `chunk_surface` to `chunk_manifestation` for non-ontology cache/render workflows.
- USF module path naming moved from `usf.mod_runtime` to `usf.runtime`.
- Legacy global Rhai mutation surfaces under `core_mod_api::usf::{substrate, phenomenon}` were removed from bridge exports.
- Mesh/collider/material application for manifestation instances moved into a dedicated runtime capability module (`usf::runtime::manifestation_capability`).
- Phenomenon model field validation now checks capability contracts instead of hardcoded phenomenon-kind string equality.
- Partitioned model member lifecycle authority moved into dedicated runtime module (`usf::phenomenon::partition_runtime`).
- Authoritative phenomenon/model persistence writes moved from direct sync writes to queued/batched persistence runtime flush (`usf::phenomenon::persistence_runtime`).

## New Invariants

- Authoritative persistence is entity-grounded:
  - `Phenomenon`
  - `PhenomenaModel`
  - `PartialPhenomenaModel`
- Substrate state is adaptive and derived (`octree + polymorphic leaves`), not ontology-owned.
- Zone semantics are derived classifiers over substrate summaries.
- Chunk manifestation records are explicit derived caches (`cache_authority = "derived_cache"`).
- Cross-chunk coupling is explicit through chunk-edge interfaces.
- Partitioned model roots and members are topology-enforced at runtime (`root` vs `member` contracts are normalized each frame).
- Model selection is explicit by `(phenomenon_id, scale_index)`.
- Manifestation hydration reads selected phenomenon from `ZoneRealizationState` authority, not a secondary selection pass.
- Runtime hydration consumes `ChunkManifestationBinding` contracts; phenomenon-specific selection logic is separated into a binding system.
- Runtime capability toggles are explicit and orthogonal (`attach_meshes`, `enable_instance_culling`), while collider attachment is model-scoped via phenomenon capability contracts.
- World ownership contract is explicit via `UsfWorldAuthorityContract` resource and startup validation.

## Removed Assumptions

- No canonical authority on cached `rho/zone` arrays.
- No global primary-model shortcut as runtime resolution authority.
- No ontology ownership by zone/chunk-surface data structures.
- No frame-decay-only refine/coarsen behavior for substrate leaf representation.
- No separate chunk-manifestation phenomenon selector independent of zone realization state.
- No cross-phenomenon partial-record matching by model-id alone (partial matching is scoped by `phenomenon_id` + `scale` + `model_id` + `chunk_coord`).
- No bridge-level global mutation API for USF content registration outside typed script ctx flows.
