# USF Scale Engine — Current Handoff

## Working state before Pass F
- +35→0 sparse semantic worldgen spine exists.
- `UsfViewFrame` supports continuous observer-relative scale projection.
- Alt+wheel zooms; Shift+Alt+wheel moves by whole-ish scales.
- S0 voxel render geometry is presentation-separated from S0 physics/collision.
- Visible player model scales with S0 presentation while player physics stays S0.
- Dense local voxel demand is isotropic: 96 units half-extent on X/Y/Z.
- Placeholder visible coverage exists from S0 through S+35.

## Pass F goal
Replace eager +1…+35 placeholder spawning with demand-driven realization lifecycle.

- `UsfViewFrame::active_scale_demands()` exposes only the adjacent scales required by the continuous transition.
- `ProceduralScaleStack` owns disposable active representation cache only.
- `sync_scale_stack` spawns missing active positive scales and despawns stale ones.
- S0 remains owned by voxel realization.
- Semantic state remains owned by worldgen.
- Transition demand is explicitly separated from future distance-LOD demand.

## Next architectural target
Distance LOD inside one view scale.

Example at S0:
- near: dense editable voxel chunks + collision;
- middle/far: progressively coarser volumetric/terrain representations;
- no arbitrary visual wall;
- equal vertical/lateral visibility policy;
- physics demand can remain a smaller high-detail bubble independent of visual demand.

Do not solve this by loading the full 96³ demand region at 1 m sample detail forever. Split semantic visibility demand, render realization demand, and collision/interaction demand.

After that: replace S0 heightfield authority with true 3D volumetric geology (caves, overhangs, strata, mountains, valleys) driven by S0 semantic geology state.
