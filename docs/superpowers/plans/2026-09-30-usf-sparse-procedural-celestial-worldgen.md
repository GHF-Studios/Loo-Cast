# Sparse Procedural Celestial Worldgen Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Replace eager world-space celestial voxel ladders with movable semantic body frames, authority-targeted sparse realization, planetary-scale terrain bands, adaptive whole-body presentation, and one canonically orbiting voxel-backed Moon.

**Architecture:** Semantic celestial entities own canonical pose/motion; gravity, voxel generation, travel boundaries, edits, and presentations consume that authority instead of copying position. Voxel demand is keyed by semantic authority + Scale before a `VoxelWorld` exists, allowing scale-local realizations to be created lazily. Whole-body presentation uses adaptive body-surface patches derived from the same terrain field as local dense voxel realizations.

**Tech Stack:** Rust, Bevy ECS 0.19.x, Avian 3D, existing USF S−35…S+35 canonical hierarchy, existing voxel materialization/Surface Nets pipeline.

**Spec:** `docs/superpowers/specs/2026-09-30-usf-sparse-procedural-celestial-worldgen-design.md`

## Global Constraints

- S+35 is the highest ordinary generative Scale; the #43 virtual world root is topology/container only.
- World generation is phenomenon-to-phenomenon; do not introduce a global world-generation manager.
- Semantic identity, residency, realization, capability readiness, physical interaction, presentation, and numerical charts remain distinct.
- Celestial bodies are movable semantic frames; voxel fields, gravity and edits must not own independently mutable world-space centers.
- Dense 10-native materializations remain one local backend, not canonical world ontology.
- Requested/relevant work must scale with demand rather than every theoretically supported Scale Slice.
- Representation refinement within a Scale must not invent finer semantic detail.
- Cross-Scale semantic refinement adds detail bands to the same terrain truth.
- Whole-body presentation must derive from the same canonical terrain semantics as local voxel terrain.
- #47 stays QUEUED; do not generalize a universal sparse-resolution framework in this pass.
- #49 collision episodes are not implemented here; this pass must leave a clean moving-body/swept-query input boundary for them.
- Patch/install scripts must never run Cargo/Vapor build, test, fmt, or run commands. They may use narrow source guards and `git diff --check`.
- Preserve dirty/unrelated owner changes. Each coherent implementation tranche is delivered as one guarded runnable Python installer containing its repo edits plus matching idempotent `gh` issue updates.
- No `cargo fmt --all`, no reset/checkout-based recovery.

## Review Focus

1. **A celestial body moves while coarse and dense realizations exist:** all projections must follow one canonical body pose; no stale copied center may remain authoritative. Task 1/2 tests pin this.
2. **Demand requests a Scale for which no `VoxelWorld` entity exists:** the request must survive long enough to create the realization rather than disappearing because the consumer was absent. Task 3 tests pin this.
3. **A body-local edit is recorded, the body translates, and the same local point is sampled afterward:** the edit must travel with the body. Task 2 tests pin this.
4. **Whole-body presentation and dense local terrain sample the same direction/boundary:** their surface position must agree to the declared approximation error; no fake sphere or unrelated noise surface. Task 5 tests pin this.
5. **Moon orbital motion changes canonical position while observer/view Scale changes:** presentation may reproject/compress, but semantic Moon identity and canonical trajectory must remain singular. Task 7 tests pin this.

---

## File Structure

### New focused units

- `spacetime-engine/src/spatial/semantic_frame.rs`
  - generic semantic local-frame orientation and local↔canonical transforms;
  - no voxel/celestial policy.

- `spacetime-engine/src/voxel/frame.rs`
  - voxel-authority-local canonical coordinate wrapper and projection helpers;
  - prevents body-local edit coordinates from being confused with universe-global voxel query coordinates.

- `spacetime-engine/src/voxel/celestial_realization.rs`
  - lazy semantic-authority + Scale → `VoxelWorld` lifecycle;
  - no demand policy and no presentation mesh generation.

- `spacetime-engine/src/voxel/planetary_surface/mod.rs`
  - regional body-surface realization identity, patch selection, coverage and lifecycle.

- `spacetime-engine/src/voxel/planetary_surface/mesh.rs`
  - cubed-sphere patch geometry only.

- `spacetime-engine/src/game/world/fixture/orbit.rs`
  - fixture-level analytic orbit propagation for authored celestial bodies;
  - writes ordinary canonical pose/motion; no render-specific orbit.

### Existing units with changed responsibility

- `spacetime-engine/src/voxel/authority.rs`
  - body-local celestial field definition and authoritative body-local voxel edits.

- `spacetime-engine/src/voxel/base/celestial/mod.rs`
  - scale/detail-band terrain evaluator; placement supplied by realization/frame context.

- `spacetime-engine/src/voxel/realization/mod.rs`
  - collect semantic-authority-targeted voxel realization demand before scale-local `VoxelWorld`s exist.

- `spacetime-engine/src/voxel/streaming/demand/mod.rs`
  - remains materialization planning for already-created scale-local voxel realizations.

- `spacetime-engine/src/voxel/manifestation/*`
  - dense materialization presentation/collision remains intact; does not absorb planetary surface representation.

- `spacetime-engine/src/game/world/fixture/scenery/celestial.rs`
  - constructs semantic celestial bodies and their authority partitions, not eager per-Scale voxel worlds.

- `spacetime-engine/src/game/world/fixture/definition.rs`
  - authored Earth + Moon semantic/body/orbit parameters.

- `spacetime-engine/src/physics/gravity/source.rs`
  - gravity parameters only; no copied center.

- `spacetime-engine/src/physics/gravity/query.rs`
  - resolves each gravity source's canonical position from the source entity.

- `spacetime-engine/src/spatial/navigation.rs`
  - travel influence/boundary evaluation receives semantic anchor/frame instead of storing duplicate anchors.

---

### Task 1: Establish one canonical movable semantic-frame authority

**Files:**
- Create: `spacetime-engine/src/spatial/semantic_frame.rs`
- Modify: `spacetime-engine/src/spatial/mod.rs`
- Modify: `spacetime-engine/src/physics/gravity/source.rs`
- Modify: `spacetime-engine/src/physics/gravity/query.rs`
- Modify: `spacetime-engine/src/spatial/navigation.rs`
- Modify: `spacetime-engine/src/voxel/authority.rs`
- Modify: `spacetime-engine/src/game/world/fixture/scenery/celestial.rs`
- Validation: production builds and direct spatial/gravity/voxel runtime inspection

**Interfaces:**
- Produces:
  - `UsfSemanticFrame::identity() -> Self`
  - `UsfSemanticFrame::local_direction_to_world(Vec3) -> Vec3`
  - `UsfSemanticFrame::world_direction_to_local(Vec3) -> Vec3`
  - `UsfSemanticFrame::local_metres_to_world(UsfPosition, DVec3) -> Result<UsfPosition, UsfPositionError>`
  - `UsfSemanticFrame::world_to_local_metres(&UsfPosition, &UsfPosition, SpatialScale, f64) -> Result<DVec3, UsfPositionError>`
  - `RadialGravitySource::new(radius_metres, field_scale, surface_gravity_metres_per_second2)`
  - `RadialGravitySource::acceleration_at(&UsfPosition /* source center */, &UsfPosition /* sample */)`
  - `CelestialVoxelField::new(radius_metres, coarsest_detail_scale, seed, profile)` with no center
  - `CelestialVoxelField::surface_position(&UsfPosition, UsfSemanticFrame, Vec3, SpatialScale)`
- Consumes: existing `UsfPosition`, `SpatialScale`, `UsfTravelBoundary`, `GravityFieldQuery`.

- [ ] **Step 1: Add failing semantic-frame transform tests**
  - Identity frame local→world→local round-trips metre offsets at S0 and a coarse Scale.
  - A rotated frame rotates directions without changing semantic origin.
  - Translating the semantic entity changes projected world position without mutating frame-local coordinates.

- [ ] **Step 2: Implement `UsfSemanticFrame`**
  - Store orientation only (`DQuat` or the repository's equivalent double-precision quaternion).
  - Position remains the existing `UsfPosition` Component.
  - Do not create another position field or body transform authority.

- [ ] **Step 3: Add failing gravity-source movement test**
  - Spawn/source-evaluate the same `RadialGravitySource` parameters at two different `UsfPosition`s.
  - Assert acceleration follows the entity position without mutating/reconstructing source parameters.

- [ ] **Step 4: Remove `center: UsfPosition` from `RadialGravitySource`**
  - `GravityFieldQuery` becomes `Query<(Entity, &UsfPosition, &RadialGravitySource)>`.
  - Exact direct evaluation passes the entity position into `acceleration_at`.
  - Keep `field_scale`, radius, `μ`, and gravity values unchanged.

- [ ] **Step 5: Remove independently mutable anchor ownership from travel influence/boundary evaluation**
  - `UsfTravelInfluence` stores scale/extent/kind only.
  - Measurement functions receive the semantic anchor `&UsfPosition`.
  - `UsfTravelBoundary::sample_near` receives the owning semantic body's position/frame explicitly.
  - Update navigation queries to pair influence/resolver with the entity's canonical position.

- [ ] **Step 6: Remove `center` from `CelestialVoxelField`**
  - Surface evaluation receives semantic body position + frame.
  - `ProceduralCelestialBody` may carry a read-only realization snapshot of body placement only if clearly marked disposable; it must not become semantic authority.
  - Update Earth fixture construction to attach `UsfSemanticFrame::identity()`.

- [ ] **Step 7: Owner-run verification outside the installer**
  - Compile the engine/application.
  - Run the existing Earth fixture.
  - Expected: Earth gravity/navigation/terrain remain semantically unchanged before any Moon/sparse-realization work.

- [ ] **Step 8: Commit/issue checkpoint**
  - #28/#26 comment records that center ownership has been collapsed onto semantic body position/frame.
  - Do not mark the megapass complete.

---

### Task 2: Make semantic voxel edits body-local and movement-safe

**Files:**
- Create: `spacetime-engine/src/voxel/frame.rs`
- Modify: `spacetime-engine/src/voxel/mod.rs`
- Modify: `spacetime-engine/src/voxel/authority.rs`
- Modify: `spacetime-engine/src/voxel/edit/brush/mod.rs`
- Modify or split: `spacetime-engine/src/voxel/edit/operation/mod.rs`
- Modify: `spacetime-engine/src/voxel/world/mod.rs`
- Modify: `spacetime-engine/src/voxel/world/recipe/mod.rs`
- Tests: `spacetime-engine/src/voxel/edit/tests.rs`, `spacetime-engine/src/voxel/world/tests.rs`

**Interfaces:**
- Produces:
  - `VoxelFramePosition`: canonical-precision coordinate whose zero is the owning semantic frame, not universe zero.
  - `VoxelFrameBrush` and `VoxelFrameEdit` for `VoxelAuthority`.
  - `VoxelFrameSnapshot { origin: UsfPosition, frame: UsfSemanticFrame, scale: SpatialScale }`.
  - projection `VoxelFrameEdit::localized_for(snapshot, chunk_origin, extra_extent) -> Option<VoxelLocalEdit>`.
- Existing standalone `VoxelWorld` world-space inline edits may remain on `VoxelEdit`; semantic authority-backed celestial worlds use `VoxelFrameEdit`.
- Consumes: Task 1 `UsfSemanticFrame`.

- [ ] **Step 1: Add failing body-local edit movement test**
  - Record one remove/add edit in a frame at a known local position.
  - Sample it with body origin A.
  - Translate body to origin B without changing the edit.
  - Assert the edit affects the same frame-local point at B and no longer affects its old universe-global location.

- [ ] **Step 2: Introduce `VoxelFramePosition`**
  - Reuse USF canonical hierarchy/precision internally; do not store one giant `DVec3`.
  - Make world-global `VoxelQueryPosition` and frame-local `VoxelFramePosition` distinct Rust types.

- [ ] **Step 3: Introduce frame-local semantic edit shapes**
  - `VoxelAuthority` stores ordered `VoxelFrameEdit`s.
  - Keep order/persistence semantics from the existing authority log.
  - Do not key authority identity by `VoxelMaterializationChunkAddress`.

- [ ] **Step 4: Project frame edits into scale-local generation recipes**
  - `VoxelWorld::chunk_recipe_from_authority` receives the owning semantic frame snapshot.
  - Project only edits relevant to that scale-local chunk.
  - Derived indexes may be discarded/rebuilt when body pose or realization changes.

- [ ] **Step 5: Keep standalone voxel worlds backward-compatible without a semantic-frame lie**
  - Standalone `VoxelWorld::record_edit(VoxelEdit)` remains world-global.
  - Do not silently reinterpret existing world-global edits as body-local.

- [ ] **Step 6: Owner-run verification outside installer**
  - Existing standalone voxel edit tests still behave.
  - Earth semantic-authority terrain edits behave identically while Earth is stationary.
  - New movement test proves edits follow the body frame.

- [ ] **Step 7: Commit/issue checkpoint**
  - #26/#42 note: durable body-local edits are semantic facts independent of runtime realization residency.

---

### Task 3: Invert voxel demand from existing-world targeting to semantic-authority targeting

**Files:**
- Modify: `spacetime-engine/src/voxel/realization/mod.rs`
- Create: `spacetime-engine/src/voxel/celestial_realization.rs`
- Modify: `spacetime-engine/src/voxel/mod.rs`
- Modify: `spacetime-engine/src/game/world/fixture/scenery/celestial.rs`
- Modify: `spacetime-engine/src/voxel/manifestation/coverage.rs`
- Tests: `spacetime-engine/src/voxel/realization/mod.rs`

**Interfaces:**
- Replaces internal `VoxelRealizationDemand { target_world: Entity, ... }` for semantic celestial demand with:
  - `VoxelRealizationTarget { authority: Entity, scale: SpatialScale }`
  - demand identity: authority + Scale + source/view + roles + scope.
- Produces:
  - `CelestialVoxelRealization` component containing `authority: Entity`, `scale: SpatialScale`.
  - `CelestialVoxelRealizationRegistry` mapping `(authority, scale)` to the current logical realization entity.
  - `sync_celestial_voxel_realizations(...)` creates missing scale-local `VoxelWorld`s for demanded targets and retires unused ones under existing make-before-break/coverage constraints.
  - `VoxelRealizationDemandSnapshot::requests_for_target(authority, scale)` and `requests_for_world(world)` adapter after registry resolution.
- Consumes: Task 1 body-local field/frame, existing `VoxelScaleDomain`, `UsfAuthorityPartitionOf`, `UsfLogicalRealizationOf`.

- [ ] **Step 1: Add failing demand-without-world test**
  - Semantic Earth authority exists with `CelestialVoxelField` + `VoxelScaleDomain`.
  - No scale-local `VoxelWorld` exists.
  - A spatial/view refinement request targets a supported Scale.
  - Assert demand snapshot contains `(authority, requested_scale)`.

- [ ] **Step 2: Change celestial demand collection to iterate semantic celestial authorities, not existing `VoxelWorld`s**
  - Standalone voxel worlds keep their current world-targeted path.
  - Parent readiness remains authority + canonical-context coverage.
  - Do not create all supported scales merely to query their policy.

- [ ] **Step 3: Implement lazy celestial realization registry/system**
  - For each demanded `(authority, scale)`, create or reuse one `VoxelWorld` logical realization.
  - Construct its `VoxelBase::CelestialBody` from the current body frame snapshot + scale.
  - Add `UsfScaleLayer`, `UsfLogicalRealizationOf`, streaming, material and collision/editing policy exactly as the old fixture loop did.
  - The fixture no longer owns creation of those per-scale entities.

- [ ] **Step 4: Define retirement**
  - A realization with no demand may retire only after its replacement/child/parent coverage dependencies no longer require it.
  - Preserve existing materialization make-before-break behavior.
  - Semantic body/authority never despawns merely because its voxel realization does.

- [ ] **Step 5: Delete the eager `for raw in CELESTIAL_VOXEL_MIN_SCALE..=detail_root` loop from the fixture**
  - Replace permanent coarse bootstrap with an authority-level bootstrap demand/seed that can instantiate only the requested coarse representation.
  - No hidden array/vector of 71 dormant `VoxelWorld`s.

- [ ] **Step 6: Add scaling test**
  - With one celestial authority and one demanded Scale, realization registry contains one scale-local world, not `domain.realization_slices().count()` worlds.
  - Add a second requested Scale and assert exactly two.
  - Drop demand and assert retirement policy can return to one/zero realization without deleting semantic authority.

- [ ] **Step 7: Owner-run verification outside installer**
  - Start Earth fixture and inspect diagnostics/entity counts.
  - Expected: no eager S−35…detail-root `VoxelWorld` ladder.
  - Local requested interaction terrain still materializes through ordinary streaming.

- [ ] **Step 8: Commit/issue checkpoint**
  - #5/#28 current truth updated with authority-targeted demand and lazy realization.

---

### Task 4: Replace rocky macro noise with hierarchical planetary terrain semantics

**Files:**
- Modify/split: `spacetime-engine/src/voxel/base/celestial/mod.rs`
- Create: `spacetime-engine/src/voxel/base/celestial/rocky.rs`
- Create: `spacetime-engine/src/voxel/base/celestial/bands.rs`
- Modify tests in: `spacetime-engine/src/voxel/base/celestial/mod.rs`

**Interfaces:**
- Produces:
  - `PlanetaryTerrainBand` or equivalent internal band descriptor with semantic Scale range, amplitude and keyed-domain identity.
  - `rocky_surface_displacement_metres(direction, through_scale, seed) -> f64`.
  - deterministic coarse province/orogenic functions that are independent of runtime materialization/chunk layout.
- Consumes: `CelestialBodyProfile`, `SpatialScale`, existing keyed/semantic noise helpers.

- [ ] **Step 1: Write deterministic morphology tests**
  - Same seed/direction/Scale is bitwise/deterministically stable.
  - Crossing to a finer semantic Scale adds residual detail rather than replacing coarser displacement.
  - Different materialization origins sampling the same canonical direction return the same surface.

- [ ] **Step 2: Add planetary-scale relief contract tests**
  - Earth-radius Rocky profile samples a broad deterministic direction set.
  - Assert relief envelope permits at least 5 km positive/negative class variation.
  - Assert broad-band neighboring directions demonstrate structures whose characteristic span is hundreds to thousands of kilometres.
  - Tests should check scale/magnitude invariants, not one brittle screenshot/noise value.

- [ ] **Step 3: Implement broad rocky bands**
  - Planet/province-scale low-frequency structure.
  - Long coherent orogenic/rift belt fields.
  - Basin/plateau modulation.
  - Existing finer detail bands remain residual layers below them.
  - Use keyed deterministic domains; no mutable global RNG.

- [ ] **Step 4: Preserve the single semantic surface resolver**
  - `surface_position` and dense voxel signed-distance sampling must consume the same accumulated terrain displacement.
  - Remove obsolete `rocky_macro_relative_relief` two-wave-only authority once no caller needs it.

- [ ] **Step 5: Owner-run visual verification outside installer**
  - On Earth surface: terrain is visibly not globally flat.
  - At regional/planetary distance once Task 5 lands: coherent mountain systems read at planetary scale.

- [ ] **Step 6: Commit/issue checkpoint**
  - #1/#5 record that Earth terrain semantics now contain explicit planetary-scale bands.

---

### Task 5: Add adaptive whole-body surface presentation as a regional representation

**Files:**
- Create: `spacetime-engine/src/voxel/planetary_surface/mod.rs`
- Create: `spacetime-engine/src/voxel/planetary_surface/mesh.rs`
- Modify: `spacetime-engine/src/voxel/mod.rs`
- Modify: `spacetime-engine/src/spatial/view/mod.rs` only if a small mutable-anchor/projection adapter is needed
- Modify: `spacetime-engine/src/spatial/view/systems.rs` only for generic semantic-frame projection support
- Modify: `spacetime-engine/src/game/world/fixture/scenery/celestial.rs`
- Tests: new module tests + existing view tests

**Interfaces:**
- Produces:
  - `PlanetarySurfacePatchId { face, level, x, y }`.
  - `PlanetarySurfaceDemand`/internal patch selection from observer projected/geometric error.
  - `PlanetarySurfaceRealization { authority, patch, scale, revision }`.
  - mesh builder `build_planetary_surface_patch(field, body_origin, frame, patch, sample_scale) -> Mesh`.
- Uses existing `UsfSceneryPresentation` / `UsfScalePresentation` projection semantics; does not introduce a parallel camera coordinate system.
- Consumes: Task 3 authority-targeted demand and Task 4 terrain resolver.

- [ ] **Step 1: Add cubed-sphere topology tests**
  - Six root faces cover all directions.
  - Child patches exactly cover parent angular domain.
  - Shared patch edges generate the same normalized directions at equivalent tessellation points.

- [ ] **Step 2: Add semantic-surface agreement test**
  - A patch vertex at direction `d` must equal `CelestialVoxelField::surface_position(body_origin, frame, d, sample_scale)` within declared numeric tolerance.
  - Dense celestial voxel surface query at the same direction must agree semantically.

- [ ] **Step 3: Implement root patches and adaptive subdivision**
  - Start with bounded fixed tessellation per patch.
  - Subdivide based on projected/geometric error and view significance.
  - Do not tie patch subdivision level one-to-one to USF Scale.
  - Patch hierarchy is representation refinement inside the selected semantic/detail band.

- [ ] **Step 4: Attach patch realization to semantic authority**
  - Patch entities carry `UsfPresentationProjectionOf(authority)` and appropriate USF presentation component.
  - Recompute/reproject from current semantic body pose.
  - No patch position becomes semantic body authority.

- [ ] **Step 5: Implement coarse/fine make-before-break presentation composition**
  - Parent patch stays until all replacement children needed for the visible aperture are ready.
  - Dense/local voxel presentation occludes/replaces only its covered aperture; it does not globally hide the planet.
  - If exact geometric clipping against dense aperture is too large for this task, use patch-level conservative suppression only where the dense coverage fully contains a patch; never hide the whole coarse body.

- [ ] **Step 6: Add bounded-work tests**
  - Whole-body distant view requires O(root/visible adaptive patches), not dense shell materializations.
  - Refining one geographic aperture does not subdivide the entire planet to that depth.

- [ ] **Step 7: Owner-run visual verification outside installer**
  - Whole Earth is visible through the normal USF far-field projection.
  - Mountains/valleys distort the actual silhouette/surface.
  - Approaching Earth refines locally without a planet-wide mesh explosion or whole-scale pop-out.

- [ ] **Step 8: Commit/issue checkpoint**
  - #37/#5 record the first direct `VoxelRegionSpan`-era non-dense regional representation proof.
  - #47 remains QUEUED.

---

### Task 6: Generalize fixture celestial construction to phenomenon-spawned Earth and Moon

**Files:**
- Modify: `spacetime-engine/src/game/world/fixture/definition.rs`
- Modify: `spacetime-engine/src/game/world/fixture/scenery/celestial.rs`
- Modify: `spacetime-engine/src/game/world/fixture/scenery/mod.rs`
- Potential create: `spacetime-engine/src/game/world/fixture/scenery/body.rs` if `celestial.rs` becomes responsibility-heavy
- Tests: fixture construction/audit tests where practical

**Interfaces:**
- `BodyDefinition` gains body-generation/orbit fields without exposing voxel runtime representation details.
- `spawn_body(...) -> Entity` returns semantic body entity so parent/child orbital definitions can refer to prior semantic phenomena.
- Earth and Moon both travel through the same generic body constructor.
- The fixture is authored seed/input only; it does not become a global procedural worldgen manager.

- [ ] **Step 1: Extend authored body definition**
  - Earth remains radius 6,371,000 m, Rocky profile, existing gravity.
  - Add Moon with approximately 1,737,400 m radius, Lunar profile, realistic-ish surface gravity (~1.62 m/s²), and an Earth-relative orbit definition.
  - Keep exact first-proof orbital values centralized in `definition.rs`.

- [ ] **Step 2: Make construction phenomenon-to-phenomenon**
  - Construct Earth semantic phenomenon.
  - Use Earth semantic entity as the parent/reference phenomenon for Moon orbital construction.
  - Do not create one manager entity that owns all future generation rules.

- [ ] **Step 3: Ensure both bodies expose identical capability shape**
  - `UsfPosition`, `UsfSemanticFrame`, `CelestialVoxelField`, `VoxelAuthority`, `VoxelScaleDomain`, gravity, travel/refinement and authority partition.
  - Neither body eagerly creates per-Scale `VoxelWorld`s.

- [ ] **Step 4: Update world authority audit**
  - Audit N celestial bodies generically.
  - Validate semantic body capabilities without requiring “all scale realizations already exist.”
  - Earth arrival-site policy remains an authored bootstrap concern, not generic body identity.

- [ ] **Step 5: Owner-run verification outside installer**
  - Runtime audit reports both semantic Earth and Moon healthy before demanding detailed Moon terrain.

- [ ] **Step 6: Commit/issue checkpoint**
  - #26 current truth records first phenomenon-to-phenomenon celestial construction proof.

---

### Task 7: Give the Moon canonical orbital motion and moving voxel/presentation projection

**Files:**
- Create: `spacetime-engine/src/game/world/fixture/orbit.rs`
- Modify: `spacetime-engine/src/game/world/fixture/mod.rs`
- Modify: `spacetime-engine/src/game/world/fixture/definition.rs`
- Modify: `spacetime-engine/src/game/world/fixture/scenery/celestial.rs`
- Modify as needed: `spacetime-engine/src/spatial/systems.rs` or a dedicated semantic-motion application system
- Tests: `fixture/orbit.rs`

**Interfaces:**
- Produces:
  - `CelestialOrbit` semantic component describing parent body, epoch, semi-major axis, eccentricity, inclination/phase and gravitational parameter/model.
  - `propagate_orbit(orbit, parent_position, simulation_time) -> (UsfPosition, DVec3 /* m/s */)`.
  - fixed/update system writes Moon `UsfPosition` + `UsfCanonicalMotion`.
- Consumes: Task 1 semantic frame, Task 6 Earth/Moon semantic entities.
- Reuses existing `SpacecraftOrbit` only as diagnostic inspiration; do not make spacecraft UI state the celestial orbit authority.

- [ ] **Step 1: Add analytic orbit tests**
  - Circular special case preserves orbital radius.
  - Velocity magnitude is finite and consistent with chosen `μ/a`.
  - One period returns near the starting canonical position within numerical tolerance.
  - Parent translation moves the entire orbit without changing child body-local state.

- [ ] **Step 2: Implement bounded two-body propagator**
  - Use simulation time/epoch, not wall clock.
  - Keep model explicitly identified as analytic two-body first proof.
  - Emit canonical SI velocity through `UsfCanonicalMotion`.

- [ ] **Step 3: Add Moon orbit component at construction**
  - Parent is semantic Earth.
  - Initial phase chosen so Moon is visibly above/near the authored Earth arrival region during practical test acceleration where possible.
  - Do not hard-code rendering coordinates.

- [ ] **Step 4: Make all current Moon realizations follow canonical body pose**
  - Planetary surface patches reproject from semantic Moon position.
  - Any active dense Moon realization refreshes its frame snapshot/projection rather than leaving materialization geometry at the old world coordinate.
  - Body-local edits remain unchanged while world projection changes.

- [ ] **Step 5: Add moving-realization test**
  - Build/sample a Moon patch/edit at time T0.
  - Propagate to T1.
  - Assert semantic identity and body-local terrain/edit coordinate are unchanged while canonical projected position differs.

- [ ] **Step 6: Owner-run visual verification outside installer**
  - From Earth, Moon is visible through ordinary USF presentation.
  - With suitable time progression, Moon visibly moves overhead.
  - Flying/zooming toward it refines the same Moon semantic entity.
  - No render-only second Moon appears.

- [ ] **Step 7: Commit/issue checkpoint**
  - #48 gets a scoped comment: canonical orbital propagation now has a real voxel-backed celestial consumer; Map View/maneuver-node work remains QUEUED.
  - Do not change #48 workflow state unless explicitly authorized separately.

---

### Task 8: Megapass integration, telemetry, deletions, and runtime proof

**Files:**
- Modify only files touched above as required by integration evidence.
- Modify diagnostics/devtools only if needed to expose counts:
  - semantic celestial bodies;
  - active celestial scale realizations;
  - active planetary surface patches;
  - active dense materializations.
- GitHub updates: #28, #26, #5, #37, #1, #48 as supported by actual evidence.

**Interfaces:**
- No new architecture unless runtime evidence proves a missing boundary.
- Output is the end-to-end behavior from the approved spec.

- [ ] **Step 1: Delete earned transitional assumptions**
  - No copied semantic center in `CelestialVoxelField`.
  - No copied semantic center in `RadialGravitySource`.
  - No eager celestial `for scale in domain` world construction.
  - No fake whole-body planet sphere/fallback if experimentation created one.
  - No global worldgen manager.

- [ ] **Step 2: Add/verify bounded-work diagnostics**
  - One Earth + Moon should not imply 71×2 scale-local voxel worlds.
  - Far whole-body view should be dominated by adaptive surface patches, not dense materialization count.
  - Local interaction should create dense work only in demanded apertures.

- [ ] **Step 3: Owner-run compile/runtime validation outside installers**
  - Build/run under profiling as owner chooses.
  - Record compiler errors first; fix only evidenced issues.
  - Inspect Tracy/diagnostics for accidental planet-wide work.

- [ ] **Step 4: Runtime scenario A — Earth surface**
  - Player/ship local collision still works.
  - Terrain visibly contains kilometre-class relief.

- [ ] **Step 5: Runtime scenario B — surface to orbit**
  - Whole Earth becomes visible as the same continuous terrain body.
  - Coarse/fine presentation composes without global disappearance.

- [ ] **Step 6: Runtime scenario C — Moon**
  - Moon is visible, moves canonically, and can be approached/refined as the same semantic entity.
  - No eager Moon dense terrain until demanded.

- [ ] **Step 7: Runtime scenario D — movable edit sanity**
  - Apply or test one body-local voxel edit on a movable body.
  - Advance orbital motion.
  - Confirm the modification remains attached to the body.

- [ ] **Step 8: Record high-speed collision follow-up evidence without implementing #49 here**
  - Document that future high-speed queries consume moving semantic body/frame + sparse regional hierarchy + body-local terrain resolver.
  - Instantaneous ray/shape casts remain local backend tools, not canonical high-speed correctness.

- [ ] **Step 9: Update issue memory idempotently**
  - #28: current semantic/worldgen architecture.
  - #26: phenomenon-to-phenomenon construction proof.
  - #5: lazy authority-targeted voxel realization and regional representation.
  - #37: whole-body adaptive presentation/coarse-fine composition.
  - #1: Earth runtime proof.
  - #48: canonical Moon orbital consumer.
  - #49: only evidence/interface note if high-speed behavior was observed; do not claim collision episodes implemented.
  - #47 remains QUEUED.

- [ ] **Step 10: Semantic completion check**
  - Megapass is complete only if the runtime demonstrates sparse celestial realization, planetary-scale terrain, adaptive whole-body Earth, canonical moving Moon, body-local movable voxel semantics, and no eager all-scale chunk carpet.

---

## Execution packaging

Implementation should be delivered as coherent guarded Python installers, not giant unreviewable raw diffs in chat.

Recommended packaging:

1. `install_usf_semantic_celestial_frames.py` — Tasks 1–2.
2. `install_usf_sparse_celestial_realization.py` — Task 3.
3. `install_usf_planetary_terrain_surface.py` — Tasks 4–5.
4. `install_usf_earth_moon_orbit.py` — Tasks 6–7.
5. `install_usf_celestial_megapass_integration.py` — Task 8 evidence-driven corrections/deletions only.

Each installer:
- preflights narrow source structures;
- stacks on dirty state;
- restores only touched files on failure;
- performs matching idempotent `gh` updates;
- may run `git diff --check`;
- does **not** run Cargo/Vapor build/test/fmt/run.

Owner compile/runtime evidence is collected between installers, so failures constrain the next coherent tranche rather than being buried under five architectural migrations at once.
