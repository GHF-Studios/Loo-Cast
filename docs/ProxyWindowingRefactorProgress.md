# Proxy Windowing Refactor Progress (Temporary)

Status: `active`
Owner: `codex + user`
Last updated: `2026-03-08`

## Purpose
This is the monolithic working document for the RenderProxy/LogicProxy refactor.
It is the source of truth for goals, constraints, architecture, milestones, and execution progress.
It is temporary but authoritative while this refactor is in flight.

## Problem Statement
The current rendering/simulation setup is tightly coupled to 2D sprite assumptions and does not model large-scale phenomena representation cleanly. We need explicit world-windowing via proxies so extremely large or deeply nested scale relationships can be represented and simulated without requiring a single direct Bevy entity representation.

## Locked Constraints
1. `RenderLayer == scale index` for all 71 scales (`0..70`).
2. `RenderProxy` is the explicit representation/windowing mechanism.
3. `LogicProxy` is the simulation/windowing counterpart to `RenderProxy`.
4. No coarse-layer fadeout.
5. Full hierarchy support across all 71 scales.
6. Dynamic safety horizon for loading/prefetch: `0.5s`.
7. Chunk dev cubes:
   - deterministic random placement
   - subgrid aligned
   - single z-slice
   - cube size `100x100x100` in local unit space
   - default instance count `8` per chunk
8. Subsection rendering must be procedural/analytic ("vector-like"), not bitmap-LOD.
9. Conservative deletions: keep legacy sprite/picking paths unless explicitly removed later.
10. Per-scale depth spacing target: at least `1000` z-units between adjacent scales.
11. Camera depth range target: support almost `100,000` z-units.
12. `MainEntity` transform is non-authoritative under the proxy contract and should be treated as UB for simulation semantics.

## Formal Proxy Model
### Entities
1. `MainEntity`: canonical ECS container for semantic components; extensible host for proxy-linked systems.
2. `LogicProxy`: simulation window and operational abstraction for the main entity.
3. `RenderProxy`: visual window and operational abstraction for the main entity.
4. `EntityProxyLink`: synchronization metadata between main entity, logic proxy, and render proxy.

### Invariants
1. A main entity resolves one active logic proxy and one active render proxy.
2. Proxy state changes are versioned (`ProxySyncRevision`) to avoid stale application.
3. Logic and render windows are shallow entities that represent non-shallow phenomena.
4. Scale ordering is monotonic by canonical mapping (layer + z).
5. `MainEntity` transform is a non-authoritative safety value (zeroed under UB contract when enforced).

## Representation Policy
1. Coarse context remains present in back.
2. Finer detail emerges in front through ordering/window transforms.
3. Coarse context is not removed by alpha fade mechanics.
4. View subsection extraction is explicit in proxy parameters.
5. Render proxy windowing may include optional representability modes.

## Scope and Non-Goals
### In scope
1. Proxy formalization (`LogicProxy`, `RenderProxy`, link model).
2. Depth/layer canonicalization for 71 scales.
3. Full-hierarchy containment-based loading.
4. Dynamic safety expansion based on predicted movement.
5. 3D chunk proxy representation with dev cubes.
6. Procedural dev surface for subsection stability.

### Not in scope right now
1. Final cross-scale phenomena semantics beyond proxy window foundations.
2. Aggressive cleanup/deletion of legacy sprite and picking systems.
3. Full gameplay 3D free camera/orbit controls.

## Milestones and Acceptance Criteria
### M0 - Contract and Feature Flags
Status: `in_progress`
Goal: Introduce config/scheduling gates for the new proxy architecture path.
Acceptance:
1. New and old pipeline can be toggled by config.
2. No required code edits to switch runtime mode.

### M1 - Proxy Schema
Status: `completed`
Goal: Add formal proxy components/resources and link invariants.
Acceptance:
1. Root, logic proxy, render proxy schema compiles and registers.
2. Link resolution is deterministic.
3. Invariant validator reports broken links.

### M2 - Canonical Scale/Layer/Z Mapping
Status: `completed`
Goal: Define one canonical mapping for 71 scales.
Acceptance:
1. Scale -> layer maps to `0..70`.
2. Adjacent scales are separated by at least `1000` z-units.
3. Full span supports near target `100,000` z range.

### M3 - Camera/Projection Refactor (Fixed 2.5D Behavior)
Status: `pending`
Goal: Move main world camera to perspective while preserving fixed-view interaction semantics.
Acceptance:
1. Main game camera uses perspective.
2. Near/far planes support full scale-depth range.
3. Input feel remains fixed-style (no free orbit required).

### M4 - Full-Hierarchy Containment Loading
Status: `pending`
Goal: Derive hierarchy from visible descendants and ancestor containment (to root).
Acceptance:
1. No hard `MAX_DIFF_SCALE_EXP` truncation in active hierarchy logic.
2. Ancestor chain reaches full root when required.
3. Set growth remains containment-bounded.

### M5 - Dynamic Safety Margin (0.5s)
Status: `pending`
Goal: Prediction-based prefetch/load expansion.
Acceptance:
1. Crossing risk within `0.5s` expands load set.
2. Slow/deep-zoom state does not over-expand.
3. Simulation/load and render-visible sets are explicitly separated.

### M6 - LogicProxy Runtime
Status: `in_progress`
Goal: Run simulation against logic proxies, synchronized to roots.
Acceptance:
1. Logic operations can execute against proxy windows.
2. Revision/version prevents stale state application.
3. Deterministic update order per frame.

### M7 - RenderProxy Windowed Mode
Status: `in_progress`
Goal: Add windowed representability mode for render proxies.
Acceptance:
1. Coarse context persists while zooming.
2. Fine detail appears in front via depth/window transforms.
3. No coarse fade dependency.

### M8 - Chunk Dev Cubes (3D Proxy Visualization)
Status: `completed`
Goal: Replace flat chunk visual proxy with deterministic cube fields.
Acceptance:
1. Default `8` cubes per chunk.
2. Cube size is `100x100x100` local units.
3. Single z-slice occupancy.
4. Cube placement is deterministic and subgrid aligned.

### M9 - Procedural Subsection Surface
Status: `pending`
Goal: Analytic/procedural subsection rendering (vector-like behavior).
Acceptance:
1. Deep zoom does not show texture upscale artifacts.
2. Neighboring subsection seams remain stable.
3. Detail limits are model-defined, not bitmap-resolution-defined.

### M10 - Legacy Retention and Regression Hardening
Status: `in_progress`
Goal: Keep legacy systems intact while validating new path.
Acceptance:
1. Legacy sprite/picking systems still compile.
2. New chunk proxy path is isolated from legacy behavior.
3. Regression tests cover deterministic cube layout and depth monotonicity.

## Execution Order
1. M0
2. M1
3. M2
4. M3
5. M4
6. M5
7. M6
8. M7
9. M8
10. M9
11. M10

## Milestone Log
### 2026-03-08
1. Created this document.
2. Locked core constraints from planning conversation.
3. Locked full-hierarchy requirement and dynamic safety horizon.
4. Locked layer-per-scale mapping and procedural subsection direction.
5. Implemented `EntityProxyLink`, `LogicProxy`, and `ProxySyncRevision` components in code.
6. Added runtime systems for revision stepping, deterministic main->logic sync, and root transform UB enforcement.
7. Replaced `RenderProxyHandle` usage in active code paths with unified `EntityProxyLink`.
8. Added live `EntityProxyLink` invariant validation and reporting in simulation.
9. Marked M1 as `completed`; M6 remains `in_progress`.
10. Added canonical scale mapping constants/methods (`layer_index`, canonical z spacing/span, camera depth contract) in `Scale`.
11. Switched proxy placement updates to recompute depth via canonical scale mapping each frame (no ad-hoc preserved z).
12. Added `RenderProxyWindowMode` and window parameters on `RenderProxy`; default mode is windowed subsection with coarse context persistence.
13. Marked M2 as `completed`; M7 moved to `in_progress`.
14. Added deterministic window-parameter updates (`window_center_local`, `window_size_local`) driven by scale difference and effective camera zoom.
15. Added a dedicated `ChunkCubeCamera` marker and camera path with perspective projection on a dedicated render layer for chunk cube proxies.
16. Hardened camera update systems to target `MainCamera` only, avoiding projection conflicts when additional camera types exist.
17. Implemented deterministic chunk dev cube layout (`10x10` subgrid slots, default `8`, single z-slice, `100x100x100` local cube size).
18. Switched chunk render proxy spawning from sprite bundle to chunk-cube bundle while preserving legacy sprite bundle code path for future reuse.
19. Added deterministic layout unit tests in `render/functions.rs` and kept existing render windowing tests passing.
20. Marked M8 as `completed`; M10 moved to `in_progress`.
21. Added config toggle `render/use_chunk_cube_proxies` and restored legacy sprite-proxy spawning path behind the toggle.
22. Marked M0 as `in_progress` (feature-gating started).

## Open Questions (Must Resolve During Implementation)
1. Exact near/far projection values after z remap and camera pose lock.
2. Exact main/proxy ownership semantics for non-chunk entities.
3. Whether some logic proxy modes should remain chunk-only initially.
4. Render-layer composition details for main camera vs UI camera interactions.

## Change Control
1. Update this file at every milestone start and completion.
2. Do not change locked constraints without explicit user confirmation.
3. If implementation reality conflicts with a locked constraint, stop and reconcile here first.
