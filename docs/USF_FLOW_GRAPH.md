# USF Flow Graph

Purpose: canonical control/data-flow specification for USF runtime behavior, with explicit ownership for "who spawns what", "what is derived from what", and "why".

Date: 2026-04-09

## Direction Lock

1. Zones are `derived but entity-backed`.
2. Zones are an abstraction layer over metric/substrate state and can have runtime agency.
3. Zone behavior should be parameterized by derived world data/policy, not treated as canonical persisted truth.
4. Worldgen authority is zone-driven top-down, with phenomena realized as concrete interactive proxies selected by zone realization.
5. Phenomena provide cross-scale runtime agency by owning/orchestrating a stack of phenomenon models.
6. Zone feedback is indirect: phenomenon/phenomenon-model behavior influences zones by changing substrate/metric inputs, not by mutating zone truth directly.
7. Zone classification is exclusive per location per scale, with a sentinel fallback zone for unclassified locations.

## Authority Classes

| Layer | Example state | Canonical persisted authority | Entity-backed |
|---|---|---|---|
| Script-configured definitions | mod/modpack, metric, metric_set, zlm, scale, zone policy, phenomenon/model defs | Yes (as script declarations) | No |
| Canonical runtime persistence | `Phenomenon`, `PhenomenonModel`, `PartialPhenomenonModel` | Yes | Yes |
| Derived runtime abstraction | zone runtime topology, zone realization | No | Yes |
| Derived runtime/cache/output | substrate summaries, chunk realization cache/artifacts, output channel application state | No | Mixed |

## End-to-End Graph

```text
Typed USF scripts
  -> Active modpack + execution plan
  -> per-scale sampler/categorizer + schema/zlm contracts

Player/ChunkLoader movement + scale window
  -> target chunk set (active radius + parent closure)
  -> chunk spawn/despawn workflows

Live chunks + model projections + metric/zlm contracts
  -> substrate rebuild
  -> per-chunk substrate summary (metric_vector, zone_type, signatures)

Substrate summaries
  -> zone runtime topology (connected components, stable region ids, parent links)
  -> ZoneAnchor entities (derived entity-backed)

Zone runtime + zone behavior registry
  -> zone realization selection
  -> Phenomenon proxy presence per realized zone
  -> ZoneRealizationEvent stream

Zone realization events + zone runtime context
  -> phenomenon-model support/state reconciliation
  -> model selection/state updates across loaded scales

Phenomenon-model interactions
  -> substrate mutations + projection deltas
  -> next substrate rebuild/classification cycle (feedback)

Chunk + substrate summary + zone runtime + zone realization + phenomenon model defs
  -> chunk realization intents
  -> reconcile workflow
  -> output-channel application (mesh/material/collider/audio/particles/trigger/simulation_service)
```

## Startup Flow

1. Engine composes plugin graph and USF schedule sets.
2. Rhai boot script registers schedule entrypoints.
3. USF typed script bootstrap executes:
   - global contracts: `mod`, `modpack`
   - package-scoped contracts for selected mods: `metric`, `zone`, `metric_set`, `zlm`, `scale`, `phenomenon`, `phenomenon_model`
4. Active modpack is validated and execution plan is built.
5. Runtime registries initialize from script-defined contracts:
   - canonical concept catalog resource (`UsfConceptCatalog`)
   - mod manifest registry (`UsfModManifestRegistry`)
   - mod contribution registry (`UsfModContributionRegistry`)
   - unified runtime concept query service (`UsfRuntimeConceptView`)
   - metric container schemas
   - zlm registry
   - zone behavior registry
   - phenomenon definition registry

## Runtime Tick Flow (Ordered)

USF simulation set order:

1. `Substrate` (`Pre -> Runtime -> Post`)
2. `Zone` (`Pre -> Runtime -> Post`)
3. `Phenomenon` (`Pre -> Runtime -> Post`)

Then `PostUpdate` chunk realization pipeline runs from chunk realization intents/workflows.

### Substrate Runtime

1. Rebuild plans are generated from chunk/model/runtime deltas.
2. Per chunk, metric vectors/projection contributions are resolved.
3. ZLM classification resolves exactly one zone type per location/scale.
4. Unclassified locations are assigned the configured sentinel zone type.
5. Updated substrate summaries are applied and delta state is emitted.

### Zone Runtime

1. Chunk summaries are grouped into connected zone components by `(scale, zone_type)`.
2. Stable `ZoneId` records are assigned/reused.
3. `ZoneAnchor` entities are spawned/updated/despawned from derived zone topology.
4. Realization selects supported phenomenon per zone.
5. Phenomenon proxy presence is reconciled for zone realization.

### Phenomenon Runtime

1. Zone realization events drive phenomenon-model support/state synchronization.
2. Scale models are ensured for active live scales.
3. Child-scale model requests and partition member sync run.
4. Persistence/runtime maintenance runs for phenomenon/model authority.
5. Model interactions update substrate/projection state; zone consequences are realized on subsequent substrate/zone passes.

### Chunk Realization Runtime

1. Chunk realization intent is synchronized per chunk.
2. Reconcile workflow builds or restores artifacts.
3. Output channels are applied to chunk instances.
4. Runtime cache/pruning/telemetry updates run.

## Lifecycle Ownership Matrix

| Runtime object | Lifecycle authority | Mechanism (implementation detail) | Source authority |
|---|---|---|---|
| `Chunk` entity | chunk spawn/despawn workflows | direct workflow reconciliation | chunk target set from loader |
| `ZoneAnchor` entity | zone runtime reconcile | direct workflow reconciliation | substrate-derived zone topology |
| `Phenomenon` proxy for zone | zone realization authority | direct reconciliation or intent queue | zone selection policy + zone runtime |
| `PhenomenonModel` root-scale models | phenomenon ensure-scale-model pass | direct reconciliation or intent queue | phenomenon defs + loaded scales + zone context |
| Partitioned phenomenon model members | partition runtime sync | direct reconciliation or intent queue | root model topology + chunk support |
| Chunk realization instance/components | chunk realization reconcile workflow | intent/runtime cleanup | chunk realization intent (zone + model driven) |

## Aggregation and Derivation Map

Primary derivation chain:

1. `Metric container values` + `model projection contributions`
2. -> `SubstrateChunkSummary` (`metric_vector`, `zone_type`, signatures)
3. -> `ZoneRuntimeState` (`ZoneId`, extents, parent relations) + `ZoneAnchor` entities
4. -> `ZoneRealizationState` (selected phenomenon proxy per zone)
5. -> `PhenomenonModel support/state` aligned to zone runtime
6. -> `ChunkRealizationIntent`
7. -> `Chunk realization artifacts` + output channel components/events

## Zone Autonomy Contract

Zones are allowed runtime agency through entity-backed behavior, but remain derived:

1. Zone structure is reconstructed from substrate/chunk state.
2. Zone-driven actions should be parameterized from derived data and zone policy.
3. Zones may influence realization/spawn decisions, but do not become canonical persisted world truth.
4. If zone state is discarded, it must be reconstructible from upstream derived/canonical inputs.
5. Zones must not be mutated as a canonical feedback target by phenomena/models; feedback flows through substrate/metrics.

## Classification Contract

1. At a given scale/location, classification resolves to one zone type only.
2. Overlap is disallowed in canonical zone assignment.
3. Unclassified locations map to a sentinel zone type (for example `zone_id = 0`).

## Worldgen Contract (Top-Down Zone-Driven)

Current implementation already supports a zone-driven spawn chain (`substrate -> zone runtime -> zone realization -> phenomenon proxy -> model/intent`).

Target operational worldgen mode for bootstrap/demo:

1. Temporarily lock direct player control.
2. Start from coarse scale context and establish chunk/substrate/zone/realization state.
3. Descend scale-by-scale in deterministic steps.
4. At each scale slice, wait for chunk + substrate + zone + realization stabilization.
5. Unlock player control at target gameplay scale window.

Runtime implementation note:

- A dedicated bootstrap worldgen controller now exists and can be toggled with environment flags:
  - `LOOCAST_USF_BOOTSTRAP_ENABLED`
  - `LOOCAST_USF_BOOTSTRAP_START_SCALE_INDEX`
  - `LOOCAST_USF_BOOTSTRAP_TARGET_SCALE_INDEX`
  - `LOOCAST_USF_BOOTSTRAP_SETTLE_FRAMES`
  - `LOOCAST_USF_BOOTSTRAP_ZOOM_STEP_MULTIPLIER`
- While active, player movement/zoom input is locked and zoom descent is automation-driven until stabilization criteria pass per scale slice.
- Stabilization now includes:
  - chunk load gate/batch quietness,
  - active-scale chunk and zone availability,
  - zone realization presence for zones that require phenomenon support,
  - active-scale phenomenon-model readiness for realized zone proxies.

## Guardrails

1. No hidden fallback selectors outside the documented chain.
2. No treating zone state or chunk realization cache as canonical persistence authority.
3. Maintain the indirect feedback rule: phenomena/models can only influence zones via substrate/metric updates.
4. Maintain single-zone classification at each location/scale with sentinel fallback.
5. Any new system that spawns/despawns entities must be added to this document's ownership matrix.
