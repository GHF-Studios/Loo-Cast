# Spacetime Engine architectural audit

**Revision:** GHF-Studios/Loo-Cast `main`, `ad479d55f04c194653c6ae83e80c1dfb070a533b`, committed 4 October 2026 at 22:04:37 CEST. Fresh remote clone; repository code unchanged.

**Conclusion:** The engine needs consolidation around existing ownership boundaries, especially chart conversion and terrain publication. It does not need another parallel framework. Much of the architecture is sensible; its adoption is incomplete, and recent terrain work has concentrated too many responsibilities in a few orchestration functions. The reported half-planet disappearance, invisible player, and broken third person are not established as one common bug by this audit.

**Owner policy:** The implementation pass must **delete all repository-owned tests and create none**. This includes inline test modules, test-only source, executable doctests, and test-only dependencies/fixtures. Keep production assertions and operational diagnostics. Validation must use normal compilation, static inspection, shader validation, profiling, and direct runtime observation. This supersedes repository guidance recommending tests. No tests or other code were deleted during this audit-only pass.

## 1. Size, growth, and complexity

The census covers every Rust/WGSL file beneath `spacetime-engine/src`. Assets, dependency sources, build output, Markdown, and lockfiles are excluded.

| Measure | Result |
|---|---:|
| Rust | 401 files; 78,812 physical lines |
| WGSL | 5 files; 1,681 physical lines |
| Combined | **80,493 lines**, including 8,766 blank lines |
| Lexically identified test regions | Approximately **6,925 lines**; 310 `#[test]` attributes |
| Production Rust function/method definitions | Approximately 2,846 |
| Production function median / 90th percentile | 8 / 42 physical lines |
| Production functions over 60 / 100 / 200 lines | **185 / 69 / 13** |
| Implementation in `mod.rs` | 54,326 lines; 36 such files exceed 300 lines |
| Companion macro crate | 611 Rust lines in 3 files |
| Actual `loo-cast-game` crate | 23 Rust lines; most game implementation remains inside the engine |

Function measurements use comment/string-aware lexical brace matching, not a Rust AST. They include signatures, blanks, and comments; exclude identified test modules; and may include nested functions. The supplied branch proxy counts decisions syntactically and is **not** formal cyclomatic complexity. Line count is a navigation aid, not proof of unnecessary code.

| Snapshot, same local time where applicable | Source lines |
|---|---:|
| 10 September, end of day (`e274357`) | 1,163 |
| 20 September (`1df9dea`) | 37,806 |
| 27 September (`944418d`) | 49,695 |
| 2 October, 48h baseline (`cb094c1`) | 68,234 |
| 3 October, 24h baseline (`6ff0cc2`) | 73,971 |
| Audited HEAD | 80,493 |

The last 48 hours contain **91 commits and +12,259 net source lines**: endpoint diff +14,491/−2,232 across 71 files. Summing each commit instead gives +21,545/−9,286, showing substantial rewrite churn. The last 24 hours contain 41 commits and +6,522 net lines. These are HEAD-relative windows, not a guess at when the earlier conversation happened.

Recent concentration: `resolution/live.rs` grew by 2,984 net lines in 48h; `base/noise/mod.rs` by 1,532; `base/celestial/mod.rs` by 941. The GPU adapter and two compute shaders added another 2,670 lines. Historical growth also includes real portal, physics, tooling, and game functionality; it is not all terrain duplication.

| Principal hotspot | Function lines | Responsibility problem |
|---|---:|---|
| [sync_celestial_clipmap_realizations](https://github.com/GHF-Studios/Loo-Cast/blob/ad479d55f04c194653c6ae83e80c1dfb070a533b/spacetime-engine/src/voxel/resolution/live.rs#L2871) | 775 | Demand, asynchronous plans, cache ownership, admission, entities/assets, publication, retirement |
| [sync_celestial_clipmap_transforms](https://github.com/GHF-Studios/Loo-Cast/blob/ad479d55f04c194653c6ae83e80c1dfb070a533b/spacetime-engine/src/voxel/resolution/live.rs#L3737) | 465 | Projection, dirty tracking, materials, whole-frontier readiness, visibility, coverage |
| [sync_planetary_surface_realizations](https://github.com/GHF-Studios/Loo-Cast/blob/ad479d55f04c194653c6ae83e80c1dfb070a533b/spacetime-engine/src/voxel/planetary_surface/mod.rs#L317) | 405 | Legacy planner and materializer retained after scheduling was removed |
| [handle_spacecraft_actions](https://github.com/GHF-Studios/Loo-Cast/blob/ad479d55f04c194653c6ae83e80c1dfb070a533b/spacetime-engine/src/game/spacecraft/mod.rs#L437) | 340 | Multiple gameplay transactions and lifecycle changes |
| [apply_spatial_transitions](https://github.com/GHF-Studios/Loo-Cast/blob/ad479d55f04c194653c6ae83e80c1dfb070a533b/spacetime-engine/src/spatial/transition/mod.rs#L315) | 326 | Handoff orchestration mixed with transfer mechanics |
| [rebuild_dirty_manifestations](https://github.com/GHF-Studios/Loo-Cast/blob/ad479d55f04c194653c6ae83e80c1dfb070a533b/spacetime-engine/src/voxel/manifestation/rebuild/mod.rs#L88) | 303 | Mesh/material pooling, ownership and publication |
| [sync_capability_realizations](https://github.com/GHF-Studios/Loo-Cast/blob/ad479d55f04c194653c6ae83e80c1dfb070a533b/spacetime-engine/src/voxel/manifestation/coverage.rs#L43) | 302 | Several readiness sources reconciled in one system |

The 60-ish-line policy is useful because the long tail is concentrated. Most functions are already short. Extract meaningful operations and ownership, not arbitrary blocks named `step_1`/`step_2`; allow documented exceptions for declarative registration and irreducible algorithms.

## 2. USF: preserve the number, complete the chart contract

**Existing strengths:** `UsfPosition` owns 71 balanced-decimal scale slots (S−35…S+35), a resolved leaf scale, and a bounded `f32` offset. `UsfChart` already provides projection/unprojection. Runtime chart maintenance, semantic body orientation, body-local edit coordinates, interaction slices, presentation scale, and binary terrain resolution are distinct concepts. Preserve these distinctions. [Canonical number](https://github.com/GHF-Studios/Loo-Cast/blob/ad479d55f04c194653c6ae83e80c1dfb070a533b/spacetime-engine/src/usf/mod.rs#L23) · [Chart algebra](https://github.com/GHF-Studios/Loo-Cast/blob/ad479d55f04c194653c6ae83e80c1dfb070a533b/spacetime-engine/src/usf/chart.rs#L7) · [Body orientation](https://github.com/GHF-Studios/Loo-Cast/blob/ad479d55f04c194653c6ae83e80c1dfb070a533b/spacetime-engine/src/spatial/semantic_frame.rs#L1) · [Body-local edits](https://github.com/GHF-Studios/Loo-Cast/blob/ad479d55f04c194653c6ae83e80c1dfb070a533b/spacetime-engine/src/voxel/frame.rs#L18)

**The adoption gap is concrete:** the source search finds only one runtime `.chart(...).unproject(...)` call outside the chart factory, in semantic synchronization. Most consumers call lower-level `relative_*`, perform manual metre/native conversion, or exchange bare `Vec3`, `DVec3`, and `Transform`. `UsfChart::project` itself returns an untagged vector. Scale does not identify origin, body orientation, view, or rebase generation. A vector plus a scale alone is insufficient.

Several callers pass `f32::MAX`/`f64::MAX` as their “bounded” extent. That enforces representability, not a useful precision budget. `world_to_local_metres` converts a canonical displacement into body-wide SI doubles; caves narrow body-local metre coordinates to `f32`; gravity projects to `f32` before widening to `f64`. These are finite-domain adapters, not a general implementation of staying within an appropriate USF digit. [Semantic-frame conversion](https://github.com/GHF-Studios/Loo-Cast/blob/ad479d55f04c194653c6ae83e80c1dfb070a533b/spacetime-engine/src/spatial/semantic_frame.rs#L67) · [Cave narrowing](https://github.com/GHF-Studios/Loo-Cast/blob/ad479d55f04c194653c6ae83e80c1dfb070a533b/spacetime-engine/src/voxel/base/celestial/caves.rs#L17) · [Gravity conversion](https://github.com/GHF-Studios/Loo-Cast/blob/ad479d55f04c194653c6ae83e80c1dfb070a533b/spacetime-engine/src/physics/gravity/source.rs#L55)

**Recommended facility:** chart-bound points, displacements, rays, bounds, and unit-bearing physical quantities at subsystem boundaries. A runtime chart identity should resolve origin, scale, orientation where applicable, and generation. Keep inner loops ordinary floats. Let a projection request state precision/extent requirements and either choose an appropriate scale or return a typed failure. Distinguish points from directions and render compression from physical conversion. Extend current USF machinery; do not introduce BigFloat everywhere or a universal numerical DSL.

**Canonical text already exists.** `Display` on `UsfCoordinate` and `UsfPosition` assembles ordinary decimal text without collapsing the whole stack through a machine float. The 71 refers to scale slots, not a fixed 71-character decimal string: chunk digits contribute at exponent `scale + 3`, and leaf-offset fractional text contributes further precision. The offset uses its shortest `f32` decimal rendering; this is not an exact binary-rational serialization format. No `FromStr` implementation was found. Preserve display; specify parsing/round-trip identity separately only if required. [Decimal representation](https://github.com/GHF-Studios/Loo-Cast/blob/ad479d55f04c194653c6ae83e80c1dfb070a533b/spacetime-engine/src/usf/mod.rs#L140) · [Offset formatting](https://github.com/GHF-Studios/Loo-Cast/blob/ad479d55f04c194653c6ae83e80c1dfb070a533b/spacetime-engine/src/usf/mod.rs#L1045)

## 3. Cross-domain spatial audit

| Boundary | Current contract / finding | Consolidation target |
|---|---|---|
| Canonical ↔ runtime | Explicit scale and shared origin, but untagged output; unlayered roots inherit anchor scale during rebase | Explicit chart membership and checked conversion |
| Body-local ↔ canonical | `UsfSemanticFrame` and `VoxelFrameSnapshot` are useful; body identity is not carried by every local value | Frame-bound local points and one conversion owner |
| Runtime ↔ physics | Scale-filtered `UsfPhysicsSlices`; custom Avian rebase updates positions and BVHs | One backend adapter with shared membership resolution |
| Canonical ↔ gravity/navigation/sweeps | Typed canonical queries exist; measurement bounds and precision vary | Preserve query contracts; centralize numeric projection policy |
| Runtime ↔ primary/context view | Two camera domains, several projection functions, copied eye/scale algebra | One explicit view projection result consumed by all renderers |
| Player model ↔ camera | Metre-authored model and scale-aware boom already exist | Resolve representation scale, depth limits and self-layer policy together |
| Portal ↔ camera/physics | Rigid mapping accepts raw transforms; boom portal search has no scale/chart filter | Typed same-chart portal mapping or explicit cross-chart mapping |
| Tools/rays ↔ gameplay | `ViewRay` has bare origin/direction; heat ray uses raw range 100 | Chart-bound rays, physical range conversion |
| Thermal ↔ manifestations | Runtime distances compared directly with metre-valued heat radius | Same physical chart before interaction weighting |
| Projectile ↔ collision | Raw transform containment across queried damageables; no slice filter | Shared chart-aware collision query |
| Authored geometry ↔ rebasing | `AuthoredMotion` retains a base transform and rewrites runtime translation | Rebase-aware authored frame or canonical motion source |
| Debug draw/gizmo ↔ view | `WorldPrimitive` stores bare vectors/transforms; gizmo edits runtime transforms | Explicit chart/view draw batches and domain-owned edits |
| CPU terrain ↔ GPU | Bounded descriptor exists, but schema, normalization and field logic have multiple owners | Versioned descriptor/schema and explicit approximation contract |

The thermal mismatch is directly visible in `propagate_combustion_heat`: it stores raw translations, computes their distance, and compares it to `radius_meters` without scale conversion. The third-person loop compares **native-unit length** to `f32::EPSILON`; at S+8 the default 4m boom is `4e-8` native units, below `1.192e-7`, so it exits before extending. These are source-level defects under those conditions, not runtime confirmation of the user's exact scene. [Thermal propagation](https://github.com/GHF-Studios/Loo-Cast/blob/ad479d55f04c194653c6ae83e80c1dfb070a533b/spacetime-engine/src/thermal/simulation/combustion/mod.rs#L63) · [Boom termination](https://github.com/GHF-Studios/Loo-Cast/blob/ad479d55f04c194653c6ae83e80c1dfb070a533b/spacetime-engine/src/game/player/camera/third_person/mod.rs#L63)

Other concrete boundary evidence: [Rebase fallback membership](https://github.com/GHF-Studios/Loo-Cast/blob/ad479d55f04c194653c6ae83e80c1dfb070a533b/spacetime-engine/src/spatial/chart/rebase.rs#L62), [Different physics membership resolver](https://github.com/GHF-Studios/Loo-Cast/blob/ad479d55f04c194653c6ae83e80c1dfb070a533b/spacetime-engine/src/physics/chart_rebase.rs#L19), [ViewRay](https://github.com/GHF-Studios/Loo-Cast/blob/ad479d55f04c194653c6ae83e80c1dfb070a533b/spacetime-engine/src/view/mod.rs#L58), [Projectile containment](https://github.com/GHF-Studios/Loo-Cast/blob/ad479d55f04c194653c6ae83e80c1dfb070a533b/spacetime-engine/src/game/combat/projectile/mod.rs#L22), [Stored motion base](https://github.com/GHF-Studios/Loo-Cast/blob/ad479d55f04c194653c6ae83e80c1dfb070a533b/spacetime-engine/src/geometry/runtime/motion/mod.rs#L16), [Draw primitives](https://github.com/GHF-Studios/Loo-Cast/blob/ad479d55f04c194653c6ae83e80c1dfb070a533b/spacetime-engine/src/devtools/draw/frame/mod.rs#L29). The rebase preflight and Avian application resolve collider scale differently; consolidate them before treating the later `expect("…preflighted")` as structurally guaranteed.

## 4. Visibility: distinguish exclusion proof from sampling policy

The clipmap already converts view basis into body-local axes, includes eye/boom offset, applies four frustum side planes with prefetch margin, and performs inner-sphere horizon occlusion. View rejection occurs before cached surface classification. **I did not find evidence that a frustum rejection itself is inserted into the semantic classification cache.** Preserve that separation. [Visibility demand](https://github.com/GHF-Studios/Loo-Cast/blob/ad479d55f04c194653c6ae83e80c1dfb070a533b/spacetime-engine/src/voxel/resolution/live.rs#L1717) · [Child admission](https://github.com/GHF-Studios/Loo-Cast/blob/ad479d55f04c194653c6ae83e80c1dfb070a533b/spacetime-engine/src/voxel/resolution/live.rs#L711)

However:

- **Roll is omitted from validity.** Refresh/relevance compare forward direction and half-angles, not right/up or all side planes. A rolled rectangular frustum can expose omitted blocks while the old plan remains “valid.”
- **Prediction can reject current visibility.** The planner uses the predicted observer position for its culling eye. Prediction should enlarge prefetch demand; it is not proof that geometry visible from the actual eye is irrelevant. [Predicted demand](https://github.com/GHF-Studios/Loo-Cast/blob/ad479d55f04c194653c6ae83e80c1dfb070a533b/spacetime-engine/src/voxel/resolution/live.rs#L2922)
- **“Empty” is stronger than the evidence.** Refinement uses radial thresholds and center/corner samples of a documented pseudo-SDF. No global bound proves that an unsampled surface cannot cross the block. Failures can return false; root admission can then enter `known_empty`. Replace booleans with outcomes such as proven empty, possibly occupied, unavailable, and view-excluded. [Occupancy heuristic](https://github.com/GHF-Studios/Loo-Cast/blob/ad479d55f04c194653c6ae83e80c1dfb070a533b/spacetime-engine/src/voxel/resolution/live.rs#L1558) · [Admission and empty caching](https://github.com/GHF-Studios/Loo-Cast/blob/ad479d55f04c194653c6ae83e80c1dfb070a533b/spacetime-engine/src/voxel/resolution/live.rs#L3340)
- **The occluder proof has the wrong apparent owner.** Horizon radius and broad shell bounds come from a developer-script relief envelope, while live GPU density samples canonical terrain. A scripting clamp is not, by itself, a certified bound on every canonical profile/band/edit. Obtain bounds from the field that actually produces geometry. [Presentation envelope](https://github.com/GHF-Studios/Loo-Cast/blob/ad479d55f04c194653c6ae83e80c1dfb070a533b/spacetime-engine/src/voxel/developer_policy.rs#L332)
- **This is not terrain self-occlusion.** A planet's inner sphere cannot generally reject same-side underground cave surfaces behind the outer crust. Frustum demand is not visibility through terrain. Any later depth/occlusion facility needs conservative temporal validity and view ownership.

CPU collision/editing residency may legitimately include invisible terrain. Keep that separate from presentation demand; do not make gameplay depend on whether a camera sees a chunk. The current clipmap also skips any authority with a nonempty edit log, so editing is a major feature-parity boundary, not a small GPU optimization detail. [Edited authority exclusion](https://github.com/GHF-Studios/Loo-Cast/blob/ad479d55f04c194653c6ae83e80c1dfb070a533b/spacetime-engine/src/voxel/resolution/live.rs#L2920)

For player visibility, investigate the full chain: semantic subject → realization scale → metre-authored model → parent visibility → self-render layer → physical camera clip range. The source contains several recent corrections already; another global scale multiplier would risk double conversion. Gizmos similarly need the viewed object's projection, not simply its raw `GlobalTransform`.

## 5. Terrain and GPU facilities

The **CPU dense path** owns editable samples, derived Surface Nets surfaces, render manifestations and collision caches. Its generation tokens, edit catch-up, sparse materialization store and separate readiness roles are valuable. The **GPU binary path** is a presentation-only Transvoxel hierarchy. Its binary LOD must remain independent of decimal USF scale. Do not merge these representations merely because both emit triangles. [Sparse store](https://github.com/GHF-Studios/Loo-Cast/blob/ad479d55f04c194653c6ae83e80c1dfb070a533b/spacetime-engine/src/voxel/store/mod.rs#L1) · [Publication/edit catch-up](https://github.com/GHF-Studios/Loo-Cast/blob/ad479d55f04c194653c6ae83e80c1dfb070a533b/spacetime-engine/src/voxel/streaming/generation/mod.rs#L90) · [Shared extraction intermediate](https://github.com/GHF-Studios/Loo-Cast/blob/ad479d55f04c194653c6ae83e80c1dfb070a533b/spacetime-engine/src/voxel/mesh/mod.rs#L1)

`resolution/live.rs` should become a composition boundary over independent owners: block topology; field classification; view demand; frontier planning/balancing; versioned jobs; GPU admission; make-before-break publication; projection; coverage; diagnostics. ECS systems should capture snapshots, invoke these operations, and apply results.

The GPU adapter has real architectural value: local semantic descriptors, immutable lookup tables, persistent mesh allocations, and no production geometry readback. Its weaknesses are concentrated:

1. **Shared schema is copied.** Density/topology WGSL contain matching runs of 50 and 54 nonblank, noncomment lines (schema/constants and face mapping), in addition to the Rust descriptor. Extract shared WGSL modules and centralize layout/table offsets.
2. **Semantic algebra is reimplemented.** `gpu.rs::canonical_axis`, balanced digits, profile parameters and seeds overlap USF/noise/celestial machinery; WGSL separately implements density and caves. Define the CPU reference contract, bounded GPU approximation/error, and shared data. Do not demand bit-identical floats without evidence. [GPU normalization](https://github.com/GHF-Studios/Loo-Cast/blob/ad479d55f04c194653c6ae83e80c1dfb070a533b/spacetime-engine/src/voxel/resolution/gpu.rs#L353)
3. **Backend lifecycle is mixed with terrain policy.** Each admitted block creates a uniform and two bind groups/passes; one scratch allocation is reused sequentially. Extract descriptor upload, allocation lifetime, dispatch and publication acknowledgment only to the extent real consumers need them. Profile batching before choosing it.
4. **Allocator assumptions are global.** The plugin changes shared MeshAllocator slab settings and assumes eight vertex floats. Treat these as an explicit backend/layout contract. [Allocator configuration](https://github.com/GHF-Studios/Loo-Cast/blob/ad479d55f04c194653c6ae83e80c1dfb070a533b/spacetime-engine/src/voxel/resolution/gpu.rs#L255)
5. **“Completed” means commands encoded.** Completion IDs are queued immediately after dispatch encoding, not GPU-fence completion. Ordered render submission may make that sufficient for publication; document and verify that contract rather than renaming it into a stronger guarantee. The processed-asset map has no visible retirement/pruning path. [Dispatch/acknowledgment](https://github.com/GHF-Studios/Loo-Cast/blob/ad479d55f04c194653c6ae83e80c1dfb070a533b/spacetime-engine/src/voxel/resolution/gpu.rs#L1033)

Keep ordinary material specialization and refinement clipping reusable. The existing `ExtendedMaterial`/shared clipping path is a better foundation than new per-feature PBR shaders. Separate the analytic debug-grid shader function from terrain policy when another material needs it. There are only five WGSL files: build a small shared library, not a speculative shader framework.

## 6. Workers, planning, and publication

The worker pool has bounded lane admission, weighted service, critical-burst fairness, typed tickets, and a compute lease separate from result lifetime. Those are useful facilities. Its failure contract is incomplete: `try_take` collapses channel disconnection into “not ready,” while the durable worker loop calls jobs without its own panic recovery. Consumers can retain an impossible pending result; one panicking job can terminate a service loop. Cancellation only suppresses queued execution/output, not an already-running expensive solve. [Tickets and worker lifetime](https://github.com/GHF-Studios/Loo-Cast/blob/ad479d55f04c194653c6ae83e80c1dfb070a533b/spacetime-engine/src/voxel/worker/mod.rs#L315)

Consolidate lifecycle vocabulary across dense generation, derivation, clipmap planning and GPU work: queued, running, ready, failed, cancelled, stale, published. Keep validity tokens specific enough to include authority/field/edit revision, chart/view generation where relevant, and output specification. Preserve make-before-break publication and independently keyed semantic computation.

Admission policy is scattered: worker lane limits, dense configuration, a wall-clock frame budget, clipmap leaf/stage limits, GPU 32-in-flight/8-per-frame limits, warm pools, and cache retention. The planner cache's maximum is a pruning trigger rather than a strict capacity bound. Establish one accounting model for CPU time, outstanding work, resident bytes, and publication cost; retain separate per-domain policies. HashMap iteration currently influences which authorities win limited admission, so fairness should be explicit. Wall-clock estimates may pace disposable work but must not choose semantic outcomes.

## 7. Coupling, documentation, and assertions

The intended engine→game dependency rule largely holds in the reusable domains inspected. The problem is not a universal import tangle. A specific cycle exists between generic developer scripting and voxel policy: the workbench registers voxel APIs, while voxel presentation consumes workbench resources. Give the domain an explicit scripting adapter; the compiler/document/editor service should not know celestial height paths. [Workbench registration](https://github.com/GHF-Studios/Loo-Cast/blob/ad479d55f04c194653c6ae83e80c1dfb070a533b/spacetime-engine/src/devtools/script_workbench/mod.rs#L176)

Most game implementation remains in `spacetime-engine/src/game` (17,051 lines); relocating it would improve packaging, but would not delete those lines. Generic locomotion/navigation currently live there too. Separate reusable mechanisms from game policy before moving crates.

Documentation is strongest around semantic authority, local charts, sparse storage, and coverage. It is weaker where comments describe previous repairs rather than a stable contract. Examples include “dropping Bevy's Task handle cancels it” after switching to custom tickets, `mod.rs` implementation despite the composition rule, and stale `game::thermal`/`game::portal` paths in tooling docs. Preserve concise invariants; remove tranche labels and obsolete migration narratives. [Existing rules](https://github.com/GHF-Studios/Loo-Cast/blob/ad479d55f04c194653c6ae83e80c1dfb070a533b/spacetime-engine/ARCHITECTURE.md#L1) · [Stale cancellation comment](https://github.com/GHF-Studios/Loo-Cast/blob/ad479d55f04c194653c6ae83e80c1dfb070a533b/spacetime-engine/src/voxel/streaming/generation/mod.rs#L176)

Assertions should protect established invariants, not replace boundary error handling. Approximately 65 production `assert!`/`assert_eq!`/`assert_ne!`, 43 `debug_assert!` variants, 90 `expect`, and 7 `unwrap` occurrences were found inside lexically identified production functions (heuristic counts). Useful examples include digit normalization and checked collision-query construction. Weak spots include unproven “preflighted” assumptions and failures converted to empty terrain, zero offsets, or omitted gravity contributions. Give recoverable projection/worker failures explicit status and observable reasons. **Production assertions stay; tests do not.**

## 8. Deletion and consolidation opportunities

| Opportunity | Scope / confidence | Required preservation |
|---|---|---|
| Delete all tests | ~6,925 currently identified engine lines, plus harness declarations and test-only dependency cleanup; explicit owner requirement | Production invariants and diagnostics |
| Delete unscheduled planetary-surface adapter | **1,433 Rust lines** before overlap with test deletion; high-confidence candidate after caller/export sweep | Any independently used public API must be resolved explicitly |
| Remove unused old portal shader | 36 lines; loader names `shader/portal.wgsl`, not the sibling old file | Active shader behavior |
| Remove CPU Transvoxel proof and dependency if no production consumer remains | Proof is test-only; `transvoxel-data` remains needed by GPU | Production table provider |
| Consolidate shader schemas/face bases and debug palettes | At least 104 matching shader lines identified; exact net savings depend on shared module plumbing | GPU layout and orientation semantics |
| Consolidate portal pair validation/contact bookkeeping | Repeated 18-line pair-resolution blocks; additional historical repetition | Distinct rigid-body and character solver behavior |
| Remove compatibility aliases and obsolete constructors | `UsfSpatialFrame`, scale0/metre aliases, materialization aliases, legacy presentation constructors | Migrate every caller, then delete aliases |
| Remove ineffective scripting state from live terrain or reconnect it deliberately | Policy revisions/snapshots travel through planning, while current descriptor construction takes only canonical field/block data | Explicit decision about whether live terrain scripting is a supported feature |
| Centralize projection, readiness and job lifecycle | Largest maintainability gain; no credible fixed line-saving estimate yet | Distinct authority, demand, readiness and view ownership |

These figures overlap. Do not sum them into a promised reduction. Deleting tests and the retired adapter will reduce size materially, but the central success criterion is fewer independent implementations and states—not an arbitrary target such as halving the repository.

## 9. Whole-engine disposition

| Area | Lines | Audit disposition |
|---|---:|---|
| Voxel | 28,513 | Primary consolidation target; isolate semantic field, demand, jobs, publication and backends |
| Game | 17,051 | Large adapters/state machines; chart migration must include tools, projectiles, spacecraft and camera |
| Devtools | 6,608 | Keep inspection/focus/widget separation; split workbench and make drawing chart-aware |
| Spatial | 6,585 | Preserve ownership vocabulary; consolidate projection and transition mechanisms |
| Physics | 4,545 | Preserve query/backend separation; unify chart membership and tolerances |
| Portal | 4,472 | Preserve rigid mapping/solver distinctions; eliminate repeated pair resolution and implicit chart assumptions |
| Console | 2,329 | Split parser, registry, log transport and UI; reduce repetitive runtime bindings with a narrow facility |
| Thermal | 2,135 | Pure coupling is a good reusable pattern; spatial ingress needs migration |
| USF | 1,900 | Keep canonical representation; complete bounded numeric contract and split formatter/algebra ownership |
| Geometry | 1,768 | Keep schema→compile→runtime staging; repair motion/rebase ownership |
| Worldgen | 1,453 | Sparse addressed store/keyed evaluation is a good foundation; avoid another spatial tree |
| Diagnostics | 801 | Retain as observation; consolidate repetitive terrain instrumentation |
| Procedural assets | 595 | Keep shared assets; centralize reusable diagnostic palette/grid primitives |
| Config | 467 | Extend typed policy ownership to recently added ad-hoc limits |
| Remaining roots/view/UI | 591 | Keep small shared contracts; strengthen chart-bearing view/ray APIs |

## 10. Execution order and evidence limits

Execute one coherent program with reviewable checkpoints: (1) remove all tests and establish a normal build baseline; (2) complete chart/unit boundaries; (3) migrate camera, player, gizmos and other spatial consumers; (4) separate field classification from view demand and establish conservative bounds; (5) split planner/publication/worker lifecycle; (6) consolidate GPU schema/backend facilities; (7) delete retired paths and revise steady-state documentation. The companion handoff provides concrete acceptance conditions.

This was a repository-wide static census and history review, with focused source tracing through the listed cross-domain paths. It is not formal verification of every arithmetic operation. No interactive scene, GPU execution, screenshot comparison, or profiler capture was performed. An offline test-target compile was initiated before the no-tests correction and cancelled during dependency compilation after that correction; it produced no engine build/test verdict. Source inspection found stale test imports/fixtures; they are deletion work, not repair recommendations. No production build pass is claimed. Git status/diff confirmed the cloned repository source was unchanged.
