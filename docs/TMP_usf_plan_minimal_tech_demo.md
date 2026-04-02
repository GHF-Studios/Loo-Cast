# TEMP Plan: Minimal USF Technical Demo

Date: 2026-04-02  
Status: planned execution target

## Goal

Ship a focused, fully working USF technical demonstration that proves the architecture, not a content-complete game.

## Demo Proof Objectives

1. Full script-entrypoint driven content registration for USF core domains.
2. Deterministic zone-driven manifestation authority.
3. Canonical persistence only in phenomenon/model/partial-model records.
4. Deterministic substrate/zone/manifestation rebuild from canonical records.
5. At least one partitioned large-support phenomenon across chunk boundaries.
6. At least one additional manifestation channel beyond mesh (audio or particles).

## Demo Scope (Intentionally Narrow)

1. Scales
   - one coarse parent scale
   - one active gameplay scale
   - one child detail scale
2. Metric stack
   - one primary demo metric (`demo_mass_density`)
   - optional auxiliary metrics for zone behavior
3. Zones
   - `empty`, `spawn_buffer`, `solid` baseline
   - deterministic realization behavior
4. Phenomena
   - one monolithic debug manifestation phenomenon
   - one partitioned broad-support phenomenon
5. Capabilities
   - mesh/material/collider on manifestation
   - one non-mesh capability path

## Build Path

1. Content baseline
   - script files for metric, metric_set, zone, zlm, scale, phenomenon, phenomenon_model, mod, modpack.
2. Runtime authority path
   - verify zone realization is sole manifestation selector.
   - verify chunk manifestation binding follows zone realization only.
3. Persistence cycle
   - persist canonical records.
   - wipe derived caches.
   - rebuild and compare deterministic outcomes.
4. Partition demonstration
   - enforce chunk-edge coupling through explicit interfaces.
   - verify deterministic partition keying and membership reconciliation.
5. Capability demonstration
   - declare capability contracts in scripts.
   - verify runtime binder application with diagnostics.

## Demo Acceptance Criteria

1. Boot succeeds from script-only content declarations for demo scope.
2. Runtime behavior remains deterministic across restart with same seed and content.
3. Derived cache deletion does not lose world truth.
4. Partitioned phenomenon remains coherent across chunk boundaries.
5. Demo clearly shows Rust platform capabilities separated from content scripts.

## Non-Goals for This Demo

1. Full galaxy-to-quark content breadth.
2. Full performance optimization pass.
3. Complete capability library coverage.
4. Final production UX polish.

## Exit Deliverables

1. Demo content package in scripts.
2. Runtime contracts and persistence boundaries validated.
3. Short operator note describing:
   - what is canonical
   - what is derived
   - how to add next content layers using entrypoints.
