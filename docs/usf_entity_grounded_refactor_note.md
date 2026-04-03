# USF Entity-Grounded Breaking Refactor Note

Date: 2026-04-02
Scope: `core_mod_api::usf` runtime/persistence authority cut

## Intentionally Broken Legacy Behavior

- Chunk persistence files are no longer treated as canonical world truth.
- The old `primary` phenomenon model runtime assumption has been removed from model realization.
- Chunk realization reconcile no longer uses direct `supports.first()` behavior.
- Zone runtime no longer classifies chunks from `UsfWorld` procedural sampling as authority.
- USF runtime naming moved from `chunk_surface` / `chunk_manifestation` to `chunk_realization` for non-ontology cache/render workflows.
- USF module path naming moved from `usf.mod_runtime` / `usf.runtime` to domain-owned paths (`usf::chunk::realization`, `rhai_binding::bridges::...::realization_channels`).
- Legacy global Rhai mutation surfaces under `core_mod_api::usf::{substrate, phenomenon}` were removed from bridge exports.
- Mesh/collider/material/audio/particles/trigger application for chunk outputs moved into bridge realization channels
  (`rhai_binding::bridges::domains::core_mod_api::usf::realization_channels`).
- Chunk realization intent shape is channel-payload based (`channel_payloads`) instead of hardcoded per-channel fields.
- Phenomenon model runtime no longer stores explicit per-channel contract components on `PhenomenonModel` entities.
- `PhenomenonKind` is no longer a hardcoded enum variant list; it is normalized string-backed metadata.
- `PhenomenonCapability` / `ctx.add_capability(...)` metadata flow was removed from runtime/script API.
- `Phenomenon.kind` and `ZonePhenomenonSupport.kind` were removed from ECS runtime state; kind remains script-definition metadata.
- Partitioned model member lifecycle authority moved into dedicated runtime module (`usf::phenomenon::partition_runtime`).
- Authoritative phenomenon/model persistence writes moved from direct sync writes to queued/batched persistence runtime flush (`usf::phenomenon::persistence_runtime`).
- Workflow ownership naming moved from `Chunk::*` to `UsfChunk::*` and chunk orchestration plugin ownership moved under `UsfPlugin`.
- USF package ownership naming moved from `usf::content` to `usf::mod_packs`, with configured-mod identity split into `usf::mods`.
- Metric sampler and categorizer kernel default IDs are now domain-owned (`usf::metric_container`, `usf::zlm`) instead of modpack-owned constants.
- Legacy `usf::definition` bucket was removed; metric identity/value/storage moved to `usf::metric`,
  metric container layout moved to `usf::metric_container`, and `ZoneTypeId` moved to `usf::zone`.

## New Invariants

- Authoritative persistence is entity-grounded:
  - `Phenomenon`
  - `PhenomenaModel`
  - `PartialPhenomenaModel`
- Substrate state is adaptive and derived (`octree + polymorphic leaves`), not ontology-owned.
- Zone semantics are derived classifiers over substrate summaries.
- Chunk realization records are explicit derived caches (`cache_authority = "derived_cache"`).
- Cross-chunk coupling is explicit through chunk-edge interfaces.
- Partitioned model roots and members are topology-enforced at runtime (`root` vs `member` contracts are normalized each frame).
- Model selection is explicit by `(phenomenon_id, scale_index)`.
- Realization reconcile reads selected phenomenon from `ZoneRealizationState` authority, not a secondary selection pass.
- Runtime reconcile consumes `ChunkRealizationIntent` contracts; phenomenon-specific selection logic is separated into a binding system.
- Runtime channel toggles are explicit and orthogonal (`attach_meshes`, `enable_instance_culling`), while collider/audio/particles/trigger are model-scoped via resolved channel payloads.
- Chunk realization emits bridge-level `ChunkRealizationChannelAppliedEvent` messages and periodic channel telemetry diagnostics
  (`mesh/collider/audio/particles/trigger`) for non-mesh capability observability.
- Persistence startup restore path uses explicit restore naming (no hydration wording).
- World ownership contract is explicit via `UsfWorldAuthorityContract` resource and startup validation.

## Removed Assumptions

- No canonical authority on cached `rho/zone` arrays.
- No global primary-model shortcut as runtime resolution authority.
- No ontology ownership by zone/chunk-surface data structures.
- No frame-decay-only refine/coarsen behavior for substrate leaf representation.
- No separate chunk-realization phenomenon selector independent of zone realization state.
- No cross-phenomenon partial-record matching by model-id alone (partial matching is scoped by `phenomenon_id` + `scale` + `model_id` + `chunk_coord`).
- No bridge-level global mutation API for USF content registration outside typed script ctx flows.
