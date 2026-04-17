# USF Flow Graph

Purpose: canonical control/data-flow specification for USF runtime behavior, with explicit ownership for "who realizes what", "what is authoritative", and "why".

Date: 2026-04-17

## Direction Lock

1. Scales are first-class runtime partitions and must be treated explicitly in data and logic.
2. Metrics are canonical substrate state per scale/chunk context.
3. Phenomenon realizers are canonical realization policy per scale.
4. Phenomena/phenomenon-models are canonical runtime entities and model stacks.
5. Capability channels are part of typed ctx graphs and are used via intent emission.
6. Reconciliation may use optional declared evaluator hooks; evaluator failures are panic-fast.
7. Canonical definition surfaces are sealed after startup; changes require explicit full rebootstrap.

## Authority Classes

| Layer | Example state | Canonical persisted authority | Entity-backed |
|---|---|---|---|
| Script-configured definitions | mod/modpack, metric, metric_set, scale, phenomenon_realizer, phenomenon/model defs | Yes (as script declarations) | No |
| Canonical runtime persistence | metric state, `Phenomenon`, `PhenomenonModel`, `PartialPhenomenonModel` | Yes | Yes |
| Derived runtime abstraction | substrate summaries, realization queues/decisions | No | Mixed |
| Derived runtime/cache/output | chunk realization artifacts, capability output application state | No | Mixed |

## End-to-End Graph

```text
Typed USF scripts
  -> Active modpack + execution plan
  -> per-scale metric and realization contracts

Player/ChunkLoader movement + scale window
  -> target chunk set (active radius + parent closure)
  -> chunk spawn/despawn workflows

Live chunks + model projections + metric contracts
  -> substrate rebuild
  -> per-chunk substrate summary (metric vectors, signatures)

Substrate summaries + scale contracts
  -> phenomenon realizer evaluation per (scale, chunk)
  -> realization decision stream

Realization decisions + phenomenon definition/model contracts
  -> phenomenon presence/model support reconciliation
  -> phenomenon/model runtime updates across loaded scales

Phenomenon-model interactions
  -> substrate mutations + projection deltas
  -> next substrate + realization cycle (feedback)

Chunk + substrate summary + realization decisions + phenomenon model defs
  -> chunk realization intents
  -> reconcile workflow
  -> output-channel application (mesh/material/collider/audio/particles/trigger/simulation_service)
```

## Startup Flow

1. Engine composes plugin graph and USF schedule sets.
2. Rhai boot script registers schedule entrypoints.
3. USF typed script bootstrap executes:
   - global contracts: `mod`, `modpack`
   - package-scoped contracts for selected mods: `metric`, `metric_set`, `scale`, `phenomenon_realizer`, `phenomenon`, `phenomenon_model`
4. Active modpack is validated and execution plan is built.
5. Runtime registries initialize from script-defined contracts:
   - canonical concept catalog resource (`UsfConceptCatalog`)
   - mod manifest registry (`UsfModManifestRegistry`)
   - mod contribution registry (`UsfModContributionRegistry`)
   - unified runtime concept query service (`UsfRuntimeConceptView`)
   - metric container schemas + registries
   - scale registry
   - phenomenon realizer registry
   - phenomenon definition/model registry

## Runtime Tick Flow (Ordered)

USF simulation set order:

1. `Substrate` (`Pre -> Runtime -> Post`)
2. `Realization` (`Pre -> Runtime -> Post`)
3. `Phenomenon` (`Pre -> Runtime -> Post`)
4. `CapabilityCommit` (`Pre -> Runtime -> Post`)

Then `PostUpdate` chunk realization pipeline runs from chunk realization intents/workflows.

### Substrate Runtime

1. Rebuild plans are generated from chunk/model/runtime deltas.
2. Per chunk, metric vectors/projection contributions are resolved.
3. Updated substrate summaries are applied and delta state is emitted.

### Realization Runtime

1. Each loaded `(scale, chunk)` resolves against its configured phenomenon realizer.
2. Realizer outputs phenomenon realization intents/decisions.
3. Realization decision stream is emitted for downstream reconciliation.

### Phenomenon Runtime

1. Realization decisions drive phenomenon-model support/state synchronization.
2. Scale models are ensured for active live scales.
3. Child-scale model requests and partition member sync run.
4. Persistence/runtime maintenance runs for phenomenon/model authority.
5. Model interactions update substrate/projection state for the next cycle.

### Capability Commit Runtime

1. Capability intent streams are merged/reconciled per channel.
2. Optional declared evaluator hooks run at configured reconciliation hook stages.
3. Accepted intents are committed into ECS/capability runtime state.
4. Runtime cache/pruning/telemetry updates run.

## Lifecycle Ownership Matrix

| Runtime object | Lifecycle authority | Mechanism (implementation detail) | Source authority |
|---|---|---|---|
| `Chunk` entity | chunk spawn/despawn workflows | direct workflow reconciliation | chunk target set from loader |
| Metric chunk state | substrate runtime + commit pass | reconcile + authoritative commit | metric contracts + runtime updates |
| `Phenomenon` runtime instances | realization + phenomenon reconciliation | direct reconciliation or intent queue | realizer decisions + phenomenon defs |
| `PhenomenonModel` root-scale models | phenomenon ensure-scale-model pass | direct reconciliation or intent queue | phenomenon defs + loaded scales |
| Partitioned phenomenon model members | partition runtime sync | direct reconciliation or intent queue | root model topology + chunk support |
| Chunk realization instance/components | chunk realization reconcile workflow | intent/runtime cleanup | chunk realization intent (realizer + model driven) |

## Aggregation and Derivation Map

Primary derivation chain:

1. `Metric container values` + `model projection contributions`
2. -> `SubstrateChunkSummary` (`metric_vector`, signatures)
3. -> `RealizationDecisionState` (per-scale/per-chunk realization decisions)
4. -> `PhenomenonModel support/state`
5. -> `ChunkRealizationIntent`
6. -> `Chunk realization artifacts` + output channel components/events

## Worldgen Contract (Top-Down Realizer-Driven)

Target operational worldgen mode for bootstrap/demo:

1. Temporarily lock direct player control.
2. Start from coarse scale context and establish chunk/substrate/realization state.
3. Descend scale-by-scale in deterministic steps.
4. At each scale slice, wait for chunk + substrate + realization stabilization.
5. Unlock player control at target gameplay scale window.

Runtime implementation note:

- A dedicated bootstrap worldgen controller exists and can be toggled with environment flags:
  - `LOOCAST_USF_BOOTSTRAP_ENABLED`
  - `LOOCAST_USF_BOOTSTRAP_START_SCALE_INDEX`
  - `LOOCAST_USF_BOOTSTRAP_TARGET_SCALE_INDEX`
  - `LOOCAST_USF_BOOTSTRAP_SETTLE_FRAMES`
  - `LOOCAST_USF_BOOTSTRAP_ZOOM_STEP_MULTIPLIER`
- While active, player movement/zoom input is locked and zoom descent is automation-driven until stabilization criteria pass per scale slice.

## Guardrails

1. No hidden fallback selectors outside the documented chain.
2. No treating derived runtime state or chunk realization caches as canonical persistence authority.
3. Cross-scale read/write must pass through explicit scale-aware gateway contracts.
4. Capability intents must honor per-channel layer invariants; cross-layer effects require explicit bridge operations.
5. Canonical definition changes at runtime are disallowed without explicit full rebootstrap.
6. Any new system that spawns/despawns authoritative entities must be added to this document's ownership matrix.

## Status Note

Current code may still include pre-target naming and intermediate runtime structure.
This document defines the forward contract for architecture and behavior.
