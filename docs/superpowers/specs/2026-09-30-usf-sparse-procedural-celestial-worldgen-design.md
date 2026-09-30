# USF Sparse Procedural Celestial Worldgen — Architectural Megapass Design

**Date:** 2026-09-30  
**Program:** GHF Studios / Loo Cast / Spacetime Engine  
**Primary umbrella:** `GHF-Studios/Loo-Cast#28`  
**Direct pressure:** `#26`, `#42`, `#5`, `#37`, `#48`, later `#49`  
**Current inspected remote:** `main` at/after `879e197431c26734e30ffdbe1a6daab5e4e91a06`

## 1. Intent

Make the USF Scale Stack the actual generative and realization architecture of the universe rather than a coordinate system wrapped around an eager chunk pipeline.

The immediate end-to-end proving target is deliberately ambitious:

- one Earth-like semantic planet with kilometer-scale relief and mountain systems spanning hundreds to thousands of kilometres;
- adaptive whole-planet presentation derived from the same canonical terrain field as local voxel terrain;
- one semantic Moon moving overhead in a real canonical orbit;
- celestial bodies whose voxel/detail state moves with the body without physically translating millions of voxel cells;
- sparse procedural world construction that can begin at the S+35 universe/root scale and descend only where meaningful phenomena exist or demand requires more detail;
- lower-scale phenomena created/conditioned by other phenomena rather than by one global world-generation manager;
- no eager all-scale `VoxelWorld` carpet;
- architecture that naturally feeds future high-speed swept collision and high-precision/high-quantity simulation.

The visible proof is not merely “a sphere with noise.” From the surface and from orbit, terrain must visibly read as planetary geography rather than Minecraft-scale local noise.

## 2. Core model

### 2.1 S+35 is the highest ordinary generative Scale

S−35…S+35 remain the ordinary USF Scale Slices.

The virtual world-root from #43 may exist above S+35 as finite topology/container ownership only. It is not another numerical Scale Slice and does not own ordinary simulation/generation semantics.

For world-generation purposes, S+35 is therefore the effective root scale.

### 2.2 A Scale is a semantic/detail band, not a mandatory storage tessellation

For a phenomenon `P`, scale-local meaning can be viewed conceptually as a function:

`F(P, canonical_domain, scale, context, seed) -> semantic contribution / continuation / realization pressure`

This is a mental model, not a requirement for one universal `EverythingFunction` trait.

Each capability or phenomenon family retains its own generation semantics. The engine shares only the machinery that is genuinely universal:

- canonical addressing;
- semantic identity/ownership;
- sparse known/off-resident presence;
- deterministic/keyed generation context;
- refinement/materialization requests;
- bounded Scale-Slice charts;
- capability realization/coverage;
- causal/event application.

### 2.3 Phenomenon-to-phenomenon world generation

There is no global god-manager that “generates the planet.”

A semantic phenomenon may procedurally seed, condition or constrain other semantic phenomena.

Example conceptual chain:

`universe/root context`
`-> stellar-system phenomenon`
`-> star + orbital body phenomena`
`-> rocky planet`
`-> lithosphere / broad province structure`
`-> orogenic belts / basins / plateaus`
`-> drainage / erosion-scale landforms`
`-> local geology / caves / material regions`
`-> metre-scale terrain`
`-> finer material structure`

This chain is not required to branch on every Scale Slice. A phenomenon may continue through many scales as one compact semantic/procedural spine and branch only when new independent semantics become useful.

A node/phenomenon may effectively say:

> no new semantic branching is required until S+n; retain this continuation and evaluate there when demanded.

That is fundamentally different from recursively manufacturing chunks at each scale.

### 2.4 Worldgen constructs semantic reality; residency realizes it

World construction and runtime residency remain independent.

A generated/recoverable Moon exists semantically even while no local Moon voxel region is resident.

A mountain range can exist as a deterministic procedural/semantic feature before any dense metre-scale voxels beneath it exist.

Queries may therefore observe different evidence levels:

- committed semantic fact;
- deterministic/recoverable procedural continuation;
- sparse summary/shadow;
- resident coarse realization;
- resident detailed realization;
- exact active simulation.

Exact local interaction can request materialization without redefining existence.

## 3. Celestial-body ownership

### 3.1 A celestial body is a movable semantic frame

A planet, moon or asteroid is one semantic entity with:

- canonical `UsfPosition`;
- canonical motion/orbit state;
- eventually canonical orientation/spin state;
- body shape/size parameters;
- gravity parameters;
- voxel/procedural generation capabilities;
- edit/modification authority;
- child/attached phenomena.

The body pose is the only world-space placement authority.

The voxel field must not own another copied world-space center.

The gravity model must not own another independently mutable center.

Travel influence/boundary data must not own another independently mutable anchor.

Those consumers resolve placement through the semantic body/frame.

### 3.2 Moving voxels means moving the semantic frame

The Moon is not “a pile of voxel chunks that all move.”

Moon-local voxel/material/detail coordinates remain stable in Moon-local semantic space.

Runtime representations are projected from:

`body-local semantic data + current body canonical pose -> current bounded realization`

Therefore moving the Moon costs approximately:

- update one canonical body pose/motion;
- invalidate/reproject only currently relevant runtime representations;
- preserve body-local edits and semantic children unchanged.

This model also applies to large movable voxel ships/asteroids later.

### 3.3 Body-local edits

Voxel edits on a movable body are expressed relative to the semantic body/frame, not ephemeral scale-local materialization addresses and not fixed world coordinates.

A death-ray scar on the Moon therefore:

1. resolves one semantic interaction against the Moon;
2. records a Moon-local semantic modification;
3. moves naturally with the Moon thereafter;
4. appears consistently in coarse and fine reconstructed representations;
5. remains meaningful after runtime realizations retire and regenerate.

The edit authority remains canonical; meshes/colliders/bricks are disposable consequences.

## 4. Sparse Scale realization

### 4.1 Delete eager all-scale voxel-world construction

The current Earth fixture eagerly creates one `VoxelWorld` entity for every supported Scale Slice from S−35 to its detail root.

That is transitional architecture and should be removed.

A celestial semantic body instead publishes:

- its supported semantic/detail scale range;
- its procedural continuation/generation capability;
- capability-specific realization rules.

Scale-local `VoxelWorld`/regional representations are created only when demand/refinement requires them.

No work should occur merely because a Scale Slice is theoretically supported.

### 4.2 Region-first demand

`VoxelRegionSpan` is the correct initial pressure: work is defined by a bounded region first; dense materialization leaves are merely one possible backend.

Demand conceptually asks:

`semantic phenomenon + canonical domain + scale/band + capability role + error/fidelity contract + priority/deadline`

The planner chooses an adaptive cover.

Possible realizations include:

- coarse surface patch;
- dense voxel brick;
- sparse volume/tree;
- query accelerator;
- collision proxy;
- edit index;
- later capability-specific forms.

Do not prematurely introduce a universal representation enum. Add concrete forms only when a real consumer requires them.

### 4.3 Components, not chunk ontology

A canonical region/realization entity may acquire independent ECS Components for the capabilities it currently needs.

Examples:

- coarse presentation surface;
- dense voxel samples;
- sparse-volume index;
- collision acceleration;
- collision realization;
- edit index;
- procedural continuation;
- coverage/readiness metadata.

The chunk/address object remains ignorant of the manager/planner and of unrelated capabilities.

Chunks are not the world.

## 5. Planetary terrain semantics

### 5.1 Required visual scale

An Earth-like rocky body should support geographic features in roughly these orders of magnitude:

- planetary figure: ~10,000 km;
- continent/oceanic/province structure: 1,000–10,000 km;
- major orogenic/rift chains and plateaus: 100–3,000+ km;
- individual mountain systems / basins: 10–500 km;
- valleys/ridges/local landforms: kilometres to tens of kilometres;
- hills/local terrain: metres to kilometres;
- fine material detail: metres downward.

Relief must comfortably include multi-kilometre elevation differences, including 5–10+ km class mountains where profile/planet semantics permit.

The current two-wave rocky macro relief is insufficient morphology even where its raw amplitude reaches kilometres.

### 5.2 Additive/residual semantic bands

Planetary terrain must not regenerate an unrelated surface at each Scale.

Conceptually:

`surface = base figure + R_coarse + ... + R_scale`

where each `R_scale` contributes detail appropriate to its semantic band.

Intra-scale representation refinement improves approximation of the same semantic truth.

Crossing into a finer semantic Scale may introduce genuinely new terrain information.

This distinction is mandatory:

- representation refinement != semantic refinement.

### 5.3 Phenomena may condition later terrain generation

Mountain systems should not be a single global noise function sampled everywhere.

A rocky-planet generator may establish broad tectonic/province/orogenic phenomena that become context for finer generators.

Likewise later erosion, river, biome, settlement or resource phenomena may consume previously established facts without one global worldgen manager owning them all.

The first implementation need not simulate full geology. It must establish the ownership/dataflow boundary so increasingly realistic procedural mechanisms can replace simple generators without changing semantic identity.

## 6. Whole-planet adaptive presentation

### 6.1 Coarse surface representation

The first non-dense regional representation should be a body-surface presentation backend, not a giant volumetric shell.

Recommended proving representation: adaptive cubed-sphere surface patches.

Properties:

- derived from the same `CelestialVoxelField`/semantic terrain surface;
- body-local;
- disposable presentation only;
- adaptive subdivision based on projected/geometric error;
- suitable from whole-body views through regional views;
- no claim to represent caves/overhangs/interiors;
- local volumetric voxel realization takes over where interaction/editing/detail requires it.

This is not a fake separate planet mesh. It is one valid presentation realization of the canonical body field.

### 6.2 Composition with local volume

A coarse surface patch remains valid context until a finer realization covering that region is ready.

Refinement is make-before-break:

1. coarse patch remains visible;
2. finer patch/local volume is prepared;
3. coverage boundary is established;
4. finer realization replaces/occludes only the covered aperture;
5. redundant coarse detail is retired later.

No whole-scale disappearance.

Boundary sampling must derive from the same semantic surface so LOD transitions do not become terrain discontinuities.

## 7. Moon and celestial motion proof

### 7.1 Add one Moon as the first moving celestial voxel body

The fixture should become at least Earth + Moon.

The Moon owns:

- one semantic entity;
- canonical position/motion;
- Lunar terrain profile;
- voxel/procedural capability;
- gravity;
- body-local edit space;
- sparse realizations on demand.

### 7.2 Initial orbital model

The first authoritative orbital model may be a deliberately bounded analytic two-body/Keplerian propagation around Earth.

The important invariant is not model complexity. It is ownership:

`orbital propagation -> canonical UsfPosition + UsfCanonicalMotion`

Presentation, gravity queries, voxel projection, travel context and future trajectory prediction consume that canonical state.

There must not be a secret render-only Moon orbit.

### 7.3 Visible proof

From Earth:

- the Moon is visible at correct body-relative angular scale through USF presentation;
- it visibly moves overhead over accelerated/appropriate simulation time;
- zoom/outward travel reveals increasingly appropriate Moon representation;
- approaching the Moon refines the same semantic body rather than swapping to a second “local Moon” identity.

Future edits/impacts on the Moon remain attached while it continues orbiting.

## 8. Interaction with high-speed travel/collision

This megapass does not implement the final #49 collision-episode resolver, but it intentionally provides the hierarchy that resolver needs.

At extreme speed, correctness should not rely on instantaneous local raycasts/shape casts.

Future path:

`canonical swept motion`
`-> semantic moving-body candidates`
`-> coarse sparse domain rejection`
`-> body-local conservative intersection/TOI bracket`
`-> progressive refinement against available representations`
`-> one accepted collision episode`
`-> one canonical response`
`-> local backend reconciliation`

Predictive residency improves fidelity/latency but does not own response authority.

Moving-body transforms must be part of the swept problem; “voxel world is static because chunks are world-space” is not acceptable.

## 9. Determinism and generation context

Procedural semantic generation should use stable keyed randomness/context.

Candidate key material:

- universe/root seed;
- semantic phenomenon identity or deterministic construction key;
- generator/domain identifier;
- canonical region/address;
- semantic Scale/detail band;
- local sample/child index.

Unrelated generation order or worker completion order must not change semantic outcomes.

Async generation may finish out of order; authoritative semantic construction/commit ordering must remain deterministic where ordering matters.

## 10. Concrete first implementation megapass

The first large implementation pass should establish the architecture and produce a conspicuous visual proof, not attempt every future procedural domain.

### Tranche A — movable celestial semantic frame

- remove duplicated center ownership from `CelestialVoxelField`;
- migrate gravity/travel/boundary consumers toward semantic-body placement rather than copied center state;
- introduce body-local sampling/projection boundary;
- preserve static-Earth behavior during migration;
- establish body-local edit coordinates/transform boundary.

### Tranche B — sparse celestial realization

- stop eagerly spawning every Scale-Slice `VoxelWorld` for Earth;
- add demand/refinement-owned creation/retirement of scale-local celestial voxel realizations;
- use `VoxelRegionSpan` as region planning vocabulary;
- retain dense materialization as exact/local backend;
- ensure unsupported/unrequested Scale Slices are essentially free.

### Tranche C — planetary surface representation

- add first regional coarse-surface presentation capability using adaptive cubed-sphere patches;
- derive samples from the canonical celestial field;
- support whole-body and regional coverage without dense 3D shell materialization;
- integrate make-before-break coverage/refinement with local voxel presentation.

### Tranche D — planetary terrain morphology

- replace the rocky macro two-wave surface with hierarchical planetary terrain bands;
- produce broad provinces + long mountain/orogenic structures + kilometre-scale relief;
- preserve shared canonical sampling across representations;
- make Earth visually useful for LOD/seam evaluation from surface through orbit.

### Tranche E — Moon + canonical orbital motion

- author Moon semantic body through the same generic celestial construction path as Earth;
- give it a Lunar terrain field and sparse realizations;
- add canonical analytic Earth-relative orbit propagation;
- drive Moon presentation from canonical motion;
- confirm Earth and Moon do not require special separate render identities.

These are one architectural megapass with sequential dependencies, not five unrelated feature branches.

## 11. Earned deletions / migrations

As each replacement becomes authoritative, delete or retire:

- center/anchor copies inside celestial capability payloads that can diverge from semantic body pose;
- eager all-scale Earth voxel realization construction;
- assumptions that presentation manifestation identity is necessarily one dense materialization key;
- any fixture-specific whole-body fallback geometry if introduced during experimentation;
- storage/topology assumptions that make canonical regions synonymous with dense chunks.

Do not retain compatibility shims without a real remaining consumer.

## 12. Issue ownership

- **#28 ACTIVE umbrella:** universal multiscale semantic/runtime foundation.
- **#26 world construction/recovery:** phenomenon-to-phenomenon semantic construction and deterministic continuation pressure.
- **#42 off-resident presence:** sparse procedural/semantic spine may remain known without local residency.
- **#5 ACTIVE voxel proving ground:** demand, sparse realization, capability readiness and dense/local backend.
- **#37 ACTIVE rendering:** adaptive whole-body/coarse-fine presentation composition.
- **#48 QUEUED orbital north star:** Moon propagation provides immediate implementation pressure, but do not silently change workflow state merely because this megapass touches canonical orbital state.
- **#49 ACTIVE collision:** consumes the movable sparse hierarchy later for swept/TOI collision; not part of this first implementation response authority.
- **#47 remains QUEUED:** do not extract a universal sparse-resolution framework until presentation and swept collision independently prove the same machinery.

## 13. Validation evidence

Patch installers for this work must **not run Cargo/Vapor build, test, fmt or run commands**.

Installer-side validation is limited to narrow structural guards and `git diff --check` where useful.

Owner-run evidence is required after application.

The meaningful runtime checkpoints are:

1. application starts with no eager all-scale celestial realization explosion;
2. local Earth walking/collision still functions;
3. whole Earth can be viewed as one continuous mountainous body;
4. major relief visibly spans kilometre elevation and continental/orogenic distances;
5. adaptive refinement occurs without global coarse-scale disappearance;
6. Moon exists through the same generic celestial architecture;
7. Moon moves canonically overhead rather than via render-only animation;
8. travelling toward Earth/Moon refines the same semantic identity;
9. currently relevant realization count/work scales with demand, not with every supported Scale Slice;
10. later body-local edits remain attached as the body moves.

Compilation is necessary owner evidence but not sufficient for semantic completion.

## 14. Semantic completion condition for this megapass

This architectural pass is successful when the runtime demonstrates:

- a sparse S+35-rooted/procedural semantic construction model rather than eager descendant materialization;
- one Earth-like planet whose large-scale terrain visibly contains kilometre-high, hundreds/thousands-kilometre geography;
- whole-body adaptive presentation derived from the same terrain semantics as local voxel detail;
- one voxel-backed Moon whose canonical orbital motion is visible from Earth;
- celestial voxel/detail/edit state attached to movable semantic bodies rather than world-space chunk carpets;
- scale-local realization created only where capability demand requires it;
- no new global world-generation manager that owns phenomenon semantics;
- a clean path from this hierarchy into future swept high-speed collision without redefining world identity.
