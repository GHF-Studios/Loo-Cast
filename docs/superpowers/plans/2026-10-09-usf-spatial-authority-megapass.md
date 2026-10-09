# USF Spatial Authority Megapass Implementation Plan

**Goal:** Make authored subject scale, canonical motion, bounded runtime charts, collision coverage, and observer presentation cooperate as one spatial system during ordinary flight, Lattice Cruise, and control transfer.

**Current base:** Loo-Cast `main` at `b77a397`, plus the existing dirty flight/camera/HUD/USF changes. Those changes are input to this pass, not a patch to discard or blindly accept. GitHub issue access was unavailable during planning; issue numbers and dependencies must be checked before issue edits. The owner will run compilation and runtime checks; this pass does not run or add tests.

**Architecture:** A semantic entity owns identity and canonical spatial state. A controlled manifestation has an authored interaction-scale affinity and one active physical motion executor. Spatial Scale Slices are bounded numerical/backend partitions; speed and proximity generate demand, never an implicit scale change. A fixed-step motion transaction prepares a canonical swept path, obtains collision evidence at any useful representation scale, resolves one physical result, commits semantic position/motion once, then projects into the active bounded chart. Presentation and developer observation read that committed state.

## Non-negotiable invariants

1. **Natural scale is authored.** Controlling player, ship, or another entity selects that entity's authored interaction policy. Cruise speed, distance, view zoom, and refinement do not independently change physical interaction scale. An explicit semantic/control transaction may request a scale handoff.
2. **One motion owner per subject and step.** Runtime physics or canonical kinematics may execute a step, never both. `UsfPosition` and `UsfCanonicalMotion` are the semantic result; Bevy `Transform` and Avian velocity are bounded projections or solver inputs. No later sync may overwrite a newer semantic commit.
3. **Collision authority follows the path.** Physical safety is decided from the proposed swept motion, available capability evidence, and an explicit uncertainty policy. A local `Collider` or current Scale Slice is not proof that the whole path is clear. Missing evidence cannot silently mean clear.
4. **A handoff is one admitted transaction.** Destination realization, relevant collision roles, backend readiness, pose, velocity interpretation, and chart projection are checked before changing active interaction state. A rejected handoff preserves the outgoing owner and motion continuity. Same-scale reanchors remain representation-only.
5. **Bound every runtime chart.** f32/f64 projection requires an explicit representable interval and error budget. Canonical state may advance beyond a chart; chart maintenance then reanchors before a stale runtime value can be interpreted as truth.
6. **Observation is truthful.** HUD, debug panels, and input help show actual command, motion, authority, coverage, and readiness. An inference from `layer == detailed` is not readiness evidence.

## Existing boundaries to retain or replace

| Existing boundary | Decision |
| --- | --- |
| `usf::UsfPosition` / `SpatialScale` | Retain canonical spatial algebra and 71 spatial slices. The exponent means metres per native spatial unit only in this pass. |
| `ecs::manifestation` ownership graph | Retain semantic entity → authority partition → logical realization → presentation projection. Keep semantic editor grouping separate from `ChildOf` transforms. |
| `game::control` transfer | Retain control transfer as the way the local player changes subject. Authored scale affinity belongs to the controlled manifestation. |
| `UsfPrimaryInteractionSlice` | Treat as the **current local focus adapter**, not a universe-wide scale oracle. Move capability/physics decisions to per-subject or per-demand evidence where callers currently infer from this resource. |
| `UsfRuntimeChartState` | Keep as the current local chart origin. Stop treating its existence as evidence that arbitrary concurrent manifestations share one physical chart. |
| `physics::collision_query` | Keep canonical sweep and candidate types; replace the observation-only PostUpdate journal as the safety decision path with a fixed-step prepare → query/refine → resolve → commit transaction. Diagnostics can still journal the result. |
| `game::locomotion::MotionExecution` | Keep one resolved executor but remove the `layer != detailed` shortcut as the sole reason for canonical authority/collision policy. |

## Implementation sequence

### 1. Stabilize the semantic motion boundary

**Files:** `spatial/motion.rs`, `spatial/systems.rs`, `game/locomotion/runtime/state/{policy,systems}.rs`, `game/locomotion/runtime/flight/{mod,commit}.rs`, `spatial/transition/{intent,admission,apply}.rs`.

- Trace every writer of `UsfPosition`, `UsfCanonicalMotion`, `Transform`, and `LinearVelocity` for the controlled subject, including landing, orbit, character movement, console relocation, and transitions. Assign one owner per execution regime and one handoff point between owners.
- Make motion authority eligibility use chart-relative precision over the proposed step, available collision representation, and required path resolution. The authored detailed scale remains a preferred collision capability, not a numerical truth predicate.
- Replace conflicting post-step synchronization with a commit protocol or explicit revision/ownership gate. Preserve already-authored dirty behavior only where it satisfies the invariant.
- Preserve velocity and orientation semantics across runtime/canonical handoffs. Remove duplicate projection or stale authority flags uncovered by the audit.

**Completion evidence:** For each motion regime, a single named writer commits semantic motion; a scale rechart or chart reanchor cannot duplicate movement, zero velocity, or resurrect the preceding pose. The owner should inspect compiler output and manually exercise player → ship → player and same-scale reanchor.

### 2. Put canonical collision resolution before motion commit

**Files:** `physics/collision_query/{contract,frame,runtime}.rs`, `voxel/collision_query/mod.rs`, `game/locomotion/runtime/flight/{mod,commit}.rs`, `physics/slice/mod.rs`, and the scheduler wiring in `physics/mod.rs` / `game/locomotion/mod.rs`.

- Change the current PostUpdate observation-only sweep into a fixed-step request based on the **proposed** trajectory, not the preceding velocity. Query providers through a generic physics-facing boundary; flight code must not directly depend on voxel implementation.
- Keep the voxel broad phase conservative over semantic field and edits, then add a bounded narrow phase for the requested error/contact policy. Candidate intervals alone cannot be interpreted as an impact; false positives need refinement. Other collision providers can join the same request without claiming independent impulse authority.
- Resolve one outcome for the subject: accepted displacement/velocity/contact, or an explicit blocked/uncertain state with a safe bounded fallback. Commit that result once. Keep Avian local collision when its chart and colliders are actually ready, and reconcile it with the same semantic contract.
- Separate lookahead/residency preparation from the exact current-step collision decision. Report which provider and bound justified the result.

**Completion evidence:** A high-speed trajectory cannot cross a known semantic solid body merely because local mesh/collider residency lags. Empty space remains traversable without broad-phase false-positive freezing. Unknown/unsupported regions have a visible, bounded safety response. The owner should use runtime diagnostics for coarse approach, voxel edit, local contact, and empty-space cruise.

### 3. Make demand and handoff readiness follow motion

**Files:** `spatial/demand/{mod,systems}.rs`, `spatial/navigation/travel/neighborhood.rs`, `spatial/refinement/demand.rs`, `game/navigation/runtime/{approach,context,travel}.rs`, `voxel/{realization,streaming/demand,streaming/generation,manifestation/collision}`.

- Derive required corridor/validity from canonical velocity, fixed-step path, stopping distance, build latency, and target resolution. Retain independent spatial and motion demand inputs so every velocity change does not rebuild topology needlessly.
- Make neighborhood validity depend on motion and nearest relevant boundary; a fixed 0.5 s expiry is only a ceiling. Ensure a newly published influence invalidates an empty cache.
- Admission must require destination roles and *physical backend* readiness over the arriving hull/path, then handle finer and coarser contact safely. Do not use successful materialization or `layer == detailed` as a proxy for a usable contact manifold.
- Prioritize generation by the subject's pending destination and trajectory evidence, rather than just distance in exponent from one global focus scale. Keep work bounded and sparse.

**Completion evidence:** Fast approach requests useful detail before contact; a delayed build holds or slows physical handoff without losing semantic motion; moving away releases obsolete work; multiple demand sources do not steal each other's authority.

### 4. Separate local focus from semantic capability

**Files:** `spatial/interaction/mod.rs`, `spatial/view/{context,systems}.rs`, `voxel/realization/intent.rs`, `voxel/streaming/generation/mod.rs`, `voxel/manifestation/material/systems.rs`, `game/control/runtime.rs`.

- Audit each `UsfPrimaryInteractionSlice` consumer. Keep it where the primary local observer genuinely selects a view/tool focus; replace it where it determines what a semantic body can simulate, collide with, or realize.
- Preserve per-view zoom and presentation projections independently from physical interaction. Allow an authored ship at its own scale to remain one semantic subject while its pilot, local camera, distant body presentation, and refinement demand use different projections.
- Keep semantic editor grouping non-transforming; any editor group component that changes `ChildOf` or physics transforms violates this pass.

**Completion evidence:** Camera mode/zoom does not move or rechart the ship; a pending interaction handoff does not globally erase unrelated capability work; pilot and ship identity remain distinct across transfer.

### 5. Reconcile navigation, cockpit truth, and developer diagnosis

**Files:** `game/flight/{model,runtime}.rs`, `game/navigation/{policy,runtime}.rs`, `game/player/{hud,camera,controls,input}.rs`, `game/playground/ui/hud`, `spatial/devtools/panel/mod.rs`, relevant console observation commands.

- Make Lattice Cruise entry, emergency dropout, cooldown, reentry, and high-speed flight report the transaction's actual readiness and accepted motion. A dropout must not claim safety solely because speed was clamped after an unresolved path.
- Make HUD velocity derive from `UsfCanonicalMotion` while runtime/native values are labeled as projections. Display motion authority, interaction slice, demanded slice, collision state, and reason for a blocked handoff in developer views.
- Keep persistent signed throttle without presets, context-accurate bindings, flight-assist on/off, and selectable camera modes aligned with actual control state. Remove any stale or duplicate UI path discovered during integration.

**Completion evidence:** HUD and developer panel agree with canonical motion during coarse flight; input help matches executable actions; ship rotation does not implicitly rotate the chosen camera mode; emergency state is visible and explains why travel stopped or dropped out.

## Manual verification and handoff

The owner requested to handle compilation and to skip automated tests. Do not run or add tests in this pass. Before handoff, perform source-level API and scheduling review, formatting/parsing where useful, whitespace/diff review, and instrumentation inspection without asserting runtime success. Give the owner a short scenario checklist: player/ship control transfer, high-speed empty-space travel, approach/contact with a procedural body, an edited voxel obstacle, dropout and reentry, chart reanchor, camera mode changes, and stale or absent collision data. Report exact compiler/runtime evidence only after the owner supplies it.

The pass is complete only when the implemented motion/collision/transition path satisfies the invariants above in the owner's runtime observations. A large diff or successfully compiling code alone is not completion.

## Future constraints, not current implementation

- **Coupled spacetime scale:** One authored scale index, with baseline spatial and temporal units coupled by ×10 per step. Scale-local clock-rate adjustments may be needed for gameplay. Do not add an independent 71 × 71 grid or automatic velocity-based rechart. Canonical world time and event order must remain explicit before temporal slices are implemented.
- **Relativistic movement and time travel:** Require an explicit causal/time model, including proper time, event ordering, and interactions across different histories. A temporal tag or a scaled `delta_seconds` is insufficient. Keep these as future design work; no claim of easy support.
- **Authored physical bodies:** Earth/Moon-style overrides should enter as semantic body/field providers using the same gravity, travel, collision, and presentation contracts as procedural bodies, without special cases in flight control.
- **Developer entity and god controls:** Typed, inspectable semantic ingress should permit deliberate overrides and live diagnosis. A privileged developer subject can compose capabilities, but must not silently bypass authority, collision, and causality rules. Full tooling is future work.
