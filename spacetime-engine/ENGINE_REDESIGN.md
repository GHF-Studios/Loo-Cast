# Spacetime Engine — aggressive redesign / pruning plan

Status: **PLAN / AUDIT CHARTER ACCEPTED. Implementation has not started.**

This document is the durable hand-off point for the next Spacetime Engine cleanup and redesign effort. It **supersedes `DEVTOOLS_REDESIGN.md`** as the active migration/work plan. The completed developer-tools architecture is documented separately in `src/devtools/ARCHITECTURE.md`; it should not be reconstructed from the old migration history.

The purpose of this effort is not another compatibility-preserving cleanup. It is an aggressive code **and design** review driven by current game pressure. Existing abstractions are not presumed valuable merely because they already exist.

## Why this pass exists

The previous developer-tools migration successfully replaced the old observability subsystem, but it deliberately preserved most surrounding engine assumptions. The codebase still contains evidence from multiple architectural eras and several systems still solve the same reality independently.

The clearest recent example is portal topology:

- collision topology cuts real holes into Avian collision geometry;
- ordinary ray/shape queries still operate in one Euclidean physics scene;
- Developer Focus manually merges an Avian ray hit with a portal-aperture hit;
- Heat Ray, object placement and erasing use raw Avian queries and are portal-oblivious;
- third-person camera pushback would need its own portal traversal logic to behave correctly.

That is the wrong ownership model. The camera, Heat Ray, Focus, etc. should not each learn how portals work. They should consume one engine-level interpretation of **what space is like**.

The same pattern appears elsewhere: obsolete USF identity code, an entire component-conflict macro subsystem supporting that obsolete model, orphan old-game files, speculative World Draw features with no consumer, and duplicate source assets remain in the repository.

This pass exists to make the engine smaller, more truthful, and more systemic.

## Governing rules

These rules are the default unless real implementation pressure disproves them.

1. **Current pressure earns abstractions.** If a mechanism has no current consumer or exists only for a superseded design, delete it. Git is the archive.
2. **One consumer is suspicious, several different consumers can justify abstraction.** A one-off generic framework should normally be specialized or inlined unless its boundary is independently valuable.
3. **Reality is modeled once.** Consumers should not each be taught portal behavior, manifestation behavior, unit behavior, etc. The owning subsystem changes the reality they consume.
4. **Backend-local behavior is explicit.** Raw Avian/local-Euclidean queries may remain as an implementation escape hatch, but ordinary engine/game code should consume engine spatial semantics.
5. **Do not preserve parity by inertia.** Existing behavior, debug tools, types, settings and modules may be removed when they do not justify their maintenance cost.
6. **Prefer deletion over compatibility layers.** Temporary bridges need a named removal point. Permanent aliases for dead architecture are not a goal.
7. **Do not generalize hypothetical mechanics.** Generalize only where current independent consumers already demonstrate the shared concept.
8. **Keep patches pressure-tested and incremental.** One meaningful redesign/deletion batch at a time; user compiles/runs before stacking the next behavior-changing batch.
9. **Documentation follows current ownership.** Migration history is not architecture. Once a redesign lands, steady-state architecture docs describe what exists now.
10. **Accepted local work must be pushed before patch generation.** Before each implementation batch, refetch the exact pushed tree. Never generate a patch against remembered or assumed local state.

## Central architectural direction: topology is part of space

### Problem

Today, an ordinary `avian3d::SpatialQuery` answers questions only inside the backend's current Euclidean scene. Portals then require special handling in callers. That produces multiple incompatible spatial realities.

### Target

Spacetime Engine owns a topology-aware spatial-query layer. Ordinary engine/game systems ask spatial questions through that layer. Portal connections participate in the answer because they are part of space, not because the caller knows about portals.

Conceptually:

```text
consumer
   |
   v
Spacetime spatial query
   |
   +-- local collision backend (Avian)
   |
   +-- spatial/topological connections
   |
   v
connected-space result
```

A query path is piecewise:

```text
origin
  |
  | local segment
  +---- collision before connection -> hit / stop
  |
  +---- connection before collision
             |
             +-- record crossing
             +-- rigidly map query state
             +-- subtract travelled path length
             +-- continue in connected space
```

### Agreed semantics

- **Topology-aware queries are the normal/default engine reality.** Raw backend queries are explicitly local/low-level.
- Query range is **total travelled path length** across every segment. A 100 m ray remains 100 m total after portal crossings.
- Topology crossings are normally transparent to hit-seeking queries, but the returned path can record crossings so Developer Focus/inspection or other callers can reason about them without performing a second custom portal raycast.
- Query state mapping includes whatever the primitive needs: origin/position, direction, orientation, and remaining distance.
- Traversal is bounded against pathological cycles. The distance budget is the primary termination rule; a conservative hop cap may exist as a safety invariant, not gameplay semantics.
- The first required primitives are **ray casts and swept shape casts**, because current real consumers already need both.
- Camera pushback becomes an ordinary topology-aware swept-sphere query. The camera may therefore cross before the player or remain behind after the player crosses without containing portal-specific logic.
- Heat Ray, Developer Focus, playground placement/erase and similar mechanics migrate to the same query reality.
- Do **not** immediately pretend full rigid-body simulation has been generalized by this query layer. Dynamic contacts and split-body solver coupling are a separate, harder problem and are reviewed later.

### Important non-goal

Do not build a giant abstract graph/wormhole framework first. Portals are the first real provider of connected-space topology. Extract only the mechanism demonstrated by current consumers; keep portal-specific policy portal-specific.

## Current worktree / recent prototype notes

The following recent work is relevant to the redesign checkpoint:

- Developer-tools redesign Stages 0–7 were locally validated. `src/devtools/ARCHITECTURE.md` is the steady-state devtools reference.
- A player/world Health-bar change was implemented locally after Stage 7, followed by two Bevy query-disjointness fixes. It may be unpushed; verify the repository before building on it.
- `portal-aware-third-person-camera.patch` demonstrated the desired camera behavior but is **not the desired final architecture**. It teaches `PlayerCamera` how portals work and is therefore superseded conceptually by the systemic spatial-query redesign. If it is locally applied, its caller-specific portal logic should be removed/replaced when the systemic query layer lands.
- Do not assume any of these local patches are on GitHub until explicitly verified.

## Review method

The first pass is a repository-wide audit, not an implementation spree.

For every meaningful module/type/abstraction, classify it as:

- **KEEP** — current responsibility is justified and boundary is healthy.
- **SIMPLIFY** — responsibility is justified, surface area is not.
- **REDESIGN** — current pressure is real but ownership/model is wrong.
- **DELETE** — dead, superseded, speculative, duplicate, or cheaper to recreate later.

Each classification should answer:

1. What current behavior depends on it?
2. How many genuinely different consumers does it have?
3. Is it modeling reality once or making callers adapt to a special case?
4. Is the abstraction more complex than the behavior it currently provides?
5. What becomes simpler if it disappears?
6. What concrete regression would deletion cause today?

Do not count comments, future plans, or hypothetical mods as consumers.

## Stage plan

The exact contents of later stages may change as the audit discovers more. Their **ordering and gates** are intentional.

### Stage 0 — Durable reset / charter

**Status: THIS DOCUMENT**

- Supersede the old `DEVTOOLS_REDESIGN.md` active plan.
- Record the aggressive-pruning rules and topology direction.
- Record recent local/prototype work so context survives chat rollover.
- No engine behavior change.

**Exit gate:** this document is in the repository and becomes the resume point.

### Stage 1 — Whole-engine audit

**Status: NEXT**

Scope:

- `spacetime-engine/src/**`
- `spacetime-engine-macros/**`
- relevant workspace dependencies / module wiring
- source assets that participate in engine behavior (e.g. duplicate shaders)

Work:

- map module/file ownership and current consumers;
- search for orphan files not in the live module graph;
- search for public exports with no real consumer;
- identify duplicate representations of the same concept;
- identify backend APIs leaking into gameplay where an engine semantic layer should own them;
- identify generic types/options with only one concrete branch in use;
- identify comments/docs that describe superseded architecture;
- produce/update a **KEEP / SIMPLIFY / REDESIGN / DELETE** table in this document before code changes.

No large redesign patch is produced during the audit itself.

**Exit gate:** user and assistant agree on the first deletion/redesign batches and disputed items are explicitly resolved.

### Stage 2 — Zero-ambiguity deletion batch

Delete things that are demonstrably dead/superseded before designing replacements.

Already-identified strong candidates, subject to Stage 1 consumer verification:

- root-level obsolete `UsfEntity::{Original, ProxyImmutable, ProxyMutable}` model;
- `OriginalUsfEntity`, `ProxyImmutableUsfEntity`, `ProxyMutableUsfEntity`;
- `ecs::component_conflict/**` if no surviving consumer remains;
- `spacetime-engine-macros` if conflict is its only surviving macro;
- now-unused macro/inventory workspace dependencies after that removal;
- `game/ui.rs` old Cube-HP test HUD;
- `game/target.rs` old singleton target prototype;
- duplicate/orphan portal shader source;
- other orphan files discovered by the audit.

This stage should be mostly deletion/module/Cargo cleanup, with no replacement framework.

**Exit gate:** fmt/check/test + ordinary gameplay smoke test. Repository should compile with visibly less historical architecture.

### Stage 3 — Shrink live speculative abstractions

Remove generic surface that has current functionality but more design than current pressure warrants.

Known candidate:

- World Draw scalar fields currently have `ScalarFieldMode::{Heatmap, HeightField}` and `height_scale`, while the only real producer is Thermal heatmap. Remove dead HeightField semantics. Evaluate whether `WorldScalarField` should honestly become a narrower `WorldHeatmap` rather than pretending to be a broader field framework.

Audit other live abstractions under the same rule. Preserve genuinely useful common rendering/math helpers; remove unused modes/settings/type layers.

**Exit gate:** behavior remains equivalent for retained features; public/internal model describes only supported behavior.

### Stage 4 — Topology-aware spatial query foundation

Design and implement the smallest engine spatial-query layer justified by current consumers.

Required capabilities:

- topology-aware ray cast;
- topology-aware swept shape cast;
- total path-distance budget;
- nearest local collision vs nearest topology crossing arbitration;
- rigid mapping through a connection;
- path/crossing metadata in results;
- caller-provided collision filtering/exclusions without leaking portal policy;
- explicit access to backend-local queries for subsystem internals that truly require them;
- cycle/hop safety.

Portal code supplies the first real spatial connection provider. Query code must not live in `PlayerCamera` or other callers.

Naming is deliberately **not frozen yet**. Do not create misleading generic names until Stage 1/4 design review settles ownership between `physics`, `topology`, and portal domain code.

**Exit gate:** focused unit tests for path traversal + ray/shape behavior through arbitrary rotated portal mappings; no consumer migration required yet beyond test harnesses.

### Stage 5 — Migrate spatial-query consumers

Move current independent consumers onto the systemic query reality, deleting their portal/local-space workarounds as they migrate.

Expected consumers include:

- third-person camera collision/pushback;
- Developer Focus;
- Heat Ray;
- playground cube placement;
- playground erase/picking paths;
- Portal Gun query paths where appropriate;
- other raw Avian ray/shape casts discovered in Stage 1.

Desired outcome: these systems express only their own semantics. For example, camera code asks for a swept-sphere placement path; it contains no `Portal`, aperture or mapping knowledge.

**Exit gate:** portal-crossing camera scenario works in both directions; rays/tools work through portals consistently; caller-specific portal query code is gone.

### Stage 6 — Simple spatial traversal ownership

Review ordinary traveler teleportation after the query layer proves the shared topology concept.

Question to resolve:

> Should an ordinary point/rigid traveler move through a spatial connection because topology owns connected space, rather than because the entity carries a portal-specific `PortalTraveler` adapter?

Likely direction:

- extract only the shared connected-space traversal mechanism actually demonstrated by projectiles/simple travelers;
- keep portal-specific activation/aperture/pair policy in Portal;
- avoid pretending split rigid bodies are solved by the same mechanism.

`PortalTraveler` / `PortalVelocity` may disappear, shrink, or become topology-generic depending on the audit.

**Exit gate:** simple traversal ownership is singular and callers do not duplicate portal crossing/mapping logic.

### Stage 7 — Split-body / collision-topology review

Only after ordinary spatial queries/traversal have a clean model, revisit the heavier prototype machinery:

- `SpatialSplitPeer` / `SpatialSplitPeerActive`;
- `SpatialSplitBox`;
- `PortalSplitTraveler`;
- `KinematicQueryExclusions`;
- portal character split code;
- rigid split solver coupling;
- `collision_topology` stencil/CSG ownership;
- relationship between collision holes and connected-space topology.

Questions:

- Which pieces are genuinely mechanism-independent now?
- Which were generalized prematurely from a single Portal experiment?
- Is `physics::topology` one coherent subsystem or a bag of Portal-extracted helpers?
- Can query exclusions disappear or shrink once consumers use the correct spatial reality?
- Should collision topology and connected-space topology share an owner, or remain distinct layers with an explicit boundary?

Do not rewrite working split physics for aesthetic symmetry. Delete/generalize only where the systemic model materially simplifies it.

**Exit gate:** split prototype still works; remaining generic topology types each have a clear non-fictional responsibility.

### Stage 8 — Domain-by-domain design/pruning pass

Continue the audit beyond topology, prioritizing large/complex areas:

- geometry/authored-geometry pipeline;
- thermal domain/spatial coupling/presentation;
- character controller/frame/config split;
- playground catalog/action/input/item architecture;
- devtools/diagnostics/shared UI after real usage pressure;
- root plugin/module composition and public exports.

This is not a mandate to rewrite every subsystem. The goal is to challenge each abstraction and remove/restructure only where the cost/ownership is unjustified.

**Exit gate:** no known high-confidence DELETE/REDESIGN item remains merely because it was out of scope.

### Stage 9 — Steady-state architecture/document cleanup

- update subsystem architecture docs to describe the final ownership model;
- remove obsolete migration prose and temporary TODOs;
- decide whether this file becomes historical or is deleted after a concise current architecture index exists;
- verify public exports and Cargo dependencies one final time;
- perform a repository-wide orphan/stale-name sweep.

**Exit gate:** repository structure and docs tell the same story as the running engine.

## Initial audit findings already established

These are evidence-backed starting points, not a complete audit.

### High-confidence DELETE candidates

- Obsolete root-level USF Original/Proxy model in `src/lib.rs`.
- Component-conflict runtime/registration system if the obsolete model is its only live consumer.
- Conflict proc-macro and potentially the entire `spacetime-engine-macros` crate if nothing else survives.
- Orphan old test-game `game/ui.rs`.
- Orphan old singleton-target `game/target.rs`.
- Duplicate/orphan portal shader file once the actually loaded shader path is verified.
- `ScalarFieldMode::HeightField` and associated `height_scale` if the audit confirms no hidden producer.

### REDESIGN candidates

- Raw `avian3d::SpatialQuery` consumption across game/devtools: replace ordinary use with engine topology-aware spatial queries.
- Developer Focus's manual `spatial hit + portal aperture hit` merge: result should come from one spatial reality.
- Third-person camera portal handling: caller-specific solution is a prototype only; systemic spatial query should own it.
- Ordinary portal traversal (`PortalTraveler` family): revisit after systemic query semantics are proven.
- `physics::topology` vs `collision_topology` vs Portal topology ownership: review after the query foundation, not before.

### SIMPLIFY candidates

- `WorldScalarField` -> narrower heatmap representation if Thermal remains its only real producer.
- Any visualization/settings abstraction with only one current mode/consumer.
- Public exports and workspace dependencies left over from removed prototypes.

### Confirmed examples that currently earn themselves

Do not delete merely for symmetry:

- `DrawDepth::Overlay` has a real Portal topology visualization use for cross-space relationship lines.
- World Draw scalar/heatmap rendering has a real Thermal consumer even if its current generic surface is too broad.
- semantic `UsfEntity` + `UsfManifestationOf` / `UsfManifestations` are actively used by current game pressure and represent the current identity model.
- collision stencil/CSG code has real Portal host-hole behavior; ownership may change, but the behavior itself is not dead.

## Patch / validation discipline

For implementation stages:

1. User pushes accepted local work first.
2. Refetch exact pushed files/tree.
3. Generate one focused patch.
4. Validate patch structure and exact baseline as far as the environment allows (`git apply --check`, clean reconstruction, `git diff --check`).
5. User runs:

```bash
cargo fmt --all
cargo check -p spacetime-engine
cargo test -p spacetime-engine
```

6. User smoke-tests the mechanics touched by that stage.
7. Only then stack the next behavior-changing patch.

Do not use `--reject`, force application, or fuzzy conflict resolution as the normal workflow. If a patch baseline differs, refetch/regenerate.

## Resume checkpoint

If conversation context is lost, resume here:

1. Read this file.
2. Read `src/devtools/ARCHITECTURE.md` only if developer-tool ownership is relevant; the old devtools migration plan is superseded.
3. Verify current GitHub `main` and ask/confirm whether accepted local Health-bar work has been pushed before generating code patches.
4. Treat `portal-aware-third-person-camera.patch` as a behavioral prototype, **not** the target architecture.
5. Continue with **Stage 1 — Whole-engine audit**.
6. Before implementation, present the KEEP / SIMPLIFY / REDESIGN / DELETE findings for discussion. The user explicitly wants design review/back-and-forth before large structural changes.

The governing question for the entire effort is:

> **What current pressure earns this abstraction's existence, and is the owning subsystem modeling reality once?**

## 2026-09-23 USF intent audit checkpoint

This checkpoint was added after USF experiments 38–41 exposed a pattern of
bugs caused by incorrect ownership/intent rather than low-level implementation.

### Active execution spine — preserved verbatim

```text
NOW
│
├─ finish stabilizing tranche 3
│
├─ INTENT AUDIT of spatial/context/demand/coverage/field/voxel/locomotion
│    ├─ What reality/concept is represented?
│    ├─ Who owns truth?
│    ├─ What is merely derived/cache?
│    ├─ What invariants actually matter?
│    ├─ Which current concepts are implementation accidents?
│    └─ KEEP / REDEFINE / MERGE / DELETE
│
├─ correct the worst conceptual mistakes
│
├─ character/contact-state + hull-authority cleanup
│
└─ then advanced field representation / approximation
```

The ordering remains active. Tranche 3 is stabilized; this audit checkpoint is
the durable output of the second item. The next implementation batch corrects
the worst spatial/demand/field mistakes. Character/contact/hull ownership stays
next after that, followed by real field representation work.

### Audit rule

For this workstream the decision order is:

```text
intent -> authority/invariants -> architecture -> implementation -> optimization
```

Existing code is not evidence that an abstraction deserves to survive.

### Target-domain classification

| Concept | Classification | Current pressure / intent |
| --- | --- | --- |
| `SpatialScale`, `UsfPosition`, `UsfChunkAddress` | **KEEP** | Canonical semantic scale/address/position spine. Virtual identity is independent from allocation. |
| `UsfScaleLayer` / Scale Slice runtime partition | **KEEP** | Numerical chart identity. It is not semantic entity identity or LOD. |
| `SpatialDemandSource` | **REDEFINE** | One bounded source of generic spatial **interest**. It must not automatically choose a vertical stack of realization scales. |
| `SpatialDemandSnapshot` | **REDEFINE** | Snapshot of canonical interest scopes, not a realization plan. Capability planners consume it. |
| `SpatialRefinementDemand` | **REDEFINE** | Requested finest detail/fidelity for capability planners. It must not manufacture generic demand scopes. Naming can be revisited after more consumers exist. |
| `UsfContextTopology` experiment | **REDEFINE / RENAME** | Canonical topology always exists virtually. Runtime demand changes an ancestor-closed **resident context set**, not topology itself. |
| `UsfScaleCoverageSnapshot` | **KEEP** | Realized fact is correctly distinct from demand. Authority + scale + role coverage has multiple current consumers (handoff and physical-surface readiness). |
| generic `spatial::field` source/cache framework from experiment 41 | **DELETE** | One real field consumer does not justify a source-at-one-position generic framework. It incorrectly assumes all future fields decompose like radial point/body sources. |
| `GravityFieldQuery` semantic boundary | **KEEP / SIMPLIFY** | Consumers should ask gravity for physical acceleration. Current backend can evaluate authored sources exactly; representation stays private. |
| hierarchical exact gravity source partition | **DELETE FOR NOW** | It adds representation machinery without changing answers or satisfying a measured performance/error requirement. Reintroduce hierarchy only with an explicit approximation/error contract. |
| `VoxelRealizationDemandSnapshot` | **KEEP / REDEFINE INPUTS** | Correct subsystem-local realization plan. It should consume generic interest + voxel/detail policy instead of pretending generic demand already chose every Scale Slice. |
| `VoxelMaterializationChunkAddress` / store | **KEEP** | Capability-local sparse 10-native-unit cache identity is correctly distinct from USF Chunk identity. |
| `VoxelScaleDomain` | **REDESIGN LATER** | It currently mixes mechanism support masks with activation/patch realization heuristics. Real pressure exists, but these policy dimensions should not become semantic body identity. |
| `PrimaryBodyContext` | **KEEP NARROW** | Navigation-relative hard-body identity/geometry only. It must not become gravity or physical-surface authority. |
| `SurfaceContext` | **REDESIGN NEXT** | It currently uses navigation's selected body as physical-surface selection and calls a radial vector `up`. Surface relation must be independently physical and distinguish radial/outward, surface normal, contact normal, and gravity-up. |
| `DetailedInteractionScale` | **REDESIGN / SPLIT NEXT** | It currently stands in for authored hull scale, solver eligibility and procedural-surface sampling scale. Those are separate policies. |
| `ControlledSubjectHull` + `SpatialSplitBox` + stance dimensions | **MERGE NEXT** | They duplicate physical box truth. One engine-level physical hull description should feed collision realization, topology partitioning and support-radius queries. |
| `CharacterGroundState` | **REDESIGN NEXT** | `grounded` + optional entity + normal can contradict. Contact should be one coherent optional value; transitions derive from state changes. |
| `CharacterLocomotionFrame` + explicit gravity-alignment policy | **KEEP** | Correctly separates locomotion reference frame from gravity itself and from temporary topology/control orientation. |
| `LocomotionRegime` vs numerical `MotionKernel` | **KEEP CONCEPTS / REDESIGN RESOLUTION** | Semantic/control mode and numerical solver are legitimately distinct, but current resolver maps them too directly and also owns collision representation choices. |
| `CollisionPolicy` inside locomotion | **REDESIGN LATER** | Collision realization is physical representation policy and should ultimately be negotiated from subject/environment/coverage, not merely locomotion mode. |

### Concrete design bugs identified

1. **Generic interest became a realization plan.** `collect_spatial_demand`
   automatically emitted a coarser vertical spine and a finer refinement spine.
   That made a generic "I care about this region" component decide which
   representations/scales should exist.

2. **Approach refinement output did not actually govern celestial voxel detail.**
   `plan_approach_refinement` computes `realization_target_scale` and updates
   `SpatialRefinementDemand`, but celestial voxel realization selected the finest
   generic demand scope and then requested surface patches from every supported
   voxel world scale. The planner's central output was therefore effectively
   bypassed.

3. **Runtime residency was called topology.** Walking changes which canonical
   contexts are resident; it does not change the canonical parent/child topology
   of USF space.

4. **Experiment 41 generalized one field too soon.** A generic field source was
   required to have one `field_position()`, which already excludes extended,
   distributed, analytic and context-produced fields. The typed gravity query is
   the valuable boundary; the generic representation is not yet earned.

5. **Physical hull has several truths.** `Collider`, `ControlledSubjectHull`,
   `SpatialSplitBox` and character stance dimensions independently describe the
   same body geometry. This is the next ownership cleanup after the spatial
   correction batch.

6. **Surface relation is selected through navigation.** `SurfaceContext`
   currently accepts `PrimaryBodyContext` as the surface body. Navigation's
   useful reference body and the physically relevant support surface are
   correlated today, not identical by definition.

### Immediate correction batch

The first implementation batch after this audit is deliberately bounded:

1. Rename runtime context topology to ancestor-closed **context residency**.
2. Restore generic spatial demand to one bounded interest scope per source.
3. Keep refinement/detail demand separate and make voxel realization actually
   obey its requested finest scale.
4. Remove the speculative generic field framework and exact hierarchical gravity
   source partition.
5. Keep the typed gravity query and exact multi-source vector summation.
6. Keep realized coverage as independent fact.
7. Do not change character movement math in this batch.

After compile/runtime validation, resume the preserved execution spine at:

```text
character/contact-state + hull-authority cleanup
```

That next batch should also remove `SurfaceContext`'s dependency on navigation
body selection and split radial/outward direction from gravity/contact normals.

## 2026-09-24 character/contact/hull authority checkpoint

The active USF intent-audit execution spine remains:

```text
NOW
│
├─ finish stabilizing tranche 3
│
├─ INTENT AUDIT of spatial/context/demand/coverage/field/voxel/locomotion
│    ├─ What reality/concept is represented?
│    ├─ Who owns truth?
│    ├─ What is merely derived/cache?
│    ├─ What invariants actually matter?
│    ├─ Which current concepts are implementation accidents?
│    └─ KEEP / REDEFINE / MERGE / DELETE
│
├─ correct the worst conceptual mistakes
│
├─ character/contact-state + hull-authority cleanup
│
└─ then advanced field representation / approximation
```

This checkpoint implements the fourth item. The fifth item remains next after
compile/runtime validation.

### Body-shape authority

The previous runtime carried overlapping shape truths:

- `ControlledSubjectHull`,
- `SpatialSplitBox`,
- character dimension constants,
- backend `Collider`.

The corrected ownership is:

```text
character/vehicle authoring profile
             |
             v
      PhysicalBoxHull            canonical SI metres; authority
             |
       +-----+--------------------+
       |                          |
       v                          v
 detailed backend Collider   SpatialSplitBox
 (chart-local realization)   (plain chart-local topology value)
```

`ScaleInteractionProxy` remains intentionally separate. It is a coarse
Scale-Slice collision representation, not a resized detailed body.

`DetailedBodyCollision` is realized fact: it marks that the current backend
collider is the detailed collider derived from `PhysicalBoxHull`. Portal split
logic consumes this fact and therefore never partitions a coarse proxy as though
it were the detailed box.

`ControlledSubjectHull` is deleted. Its detailed size belonged to
`PhysicalBoxHull`; its proxy radius belonged to `ScaleInteractionProxy`.

`DetailedInteractionScale` is narrowed/renamed to `DetailedBodyScale`. It now
answers only which Scale Slice currently hosts the subject's detailed authored
body/solver representation. It no longer selects procedural-surface sampling.

### Character support/contact authority

`CharacterGroundState` previously encoded one fact redundantly as:

```text
grounded bool
+ optional ground entity
+ ground normal
```

Those values could disagree after jumps, recharting and portal transactions.

Groundedness is now defined exactly once:

```text
CharacterGroundState.contact: Option<CharacterGroundContact>
grounded := contact.is_some()
```

The contact records the collider actually hit and its walkable normal.
`just_landed`, `just_left_ground` and `just_jumped` remain transient outputs,
derived around contact changes rather than alternate contact authority.

Portal crossing invalidates the contact through the character-state API instead
of manually mutating multiple fields. Scale recharting clears the local contact
snapshot as one operation.

The Source/Quake acceleration, friction, air acceleration, stepping and split
gravity math are intentionally unchanged.

### Surface telemetry

`SurfaceContext` survives because the flight HUD is a real current consumer, but
its authority is narrowed:

- it is read-only proximity/coverage telemetry;
- it independently selects the nearest authored celestial surface;
- it no longer inherits navigation's `PrimaryBodyContext` selection;
- it no longer uses `DetailedBodyScale` to choose procedural surface detail;
- its body-center direction is named `radial_outward`, not `up`;
- physical contact remains collision-query authority.

This preserves the distinction between:

```text
navigation reference body
gravity-derived locomotion up
body-center radial direction
procedural surface geometry
walkable/contact normal
actual collision contact
```

### Next preserved step

After this batch compiles/runs, resume the active spine at:

```text
advanced field representation / approximation
```

Do not resurrect the deleted generic field framework by default. Before adding
a hierarchical gravity representation, establish concrete pressure:

1. source-count/performance target,
2. required query types (acceleration, gradient/potential if actually needed),
3. acceptable spatial error,
4. update/currentness requirements,
5. how parent context and child residual/refinement compose,
6. which representation earns implementation (multipole, grid/brick, analytic
   summary, hybrid, etc.).

The typed `GravityFieldQuery` remains the consumer seam.

## 2026-09-24 field representation / approximation checkpoint

The original execution spine is now complete through its final item:

```text
NOW
│
├─ finish stabilizing tranche 3
│
├─ INTENT AUDIT of spatial/context/demand/coverage/field/voxel/locomotion
│    ├─ What reality/concept is represented?
│    ├─ Who owns truth?
│    ├─ What is merely derived/cache?
│    ├─ What invariants actually matter?
│    ├─ Which current concepts are implementation accidents?
│    └─ KEEP / REDEFINE / MERGE / DELETE
│
├─ correct the worst conceptual mistakes
│
├─ character/contact-state + hull-authority cleanup
│
└─ then advanced field representation / approximation
```

### Result: do not add an approximation backend yet

The field audit compared actual current domains rather than designing from the
word "field":

- gravity is a long-range additive vector field;
- voxel matter is a reconstructible signed-distance/material field with
  capability-local chunk samplers;
- thermal refinement is a finite-volume conserved-energy state attached to one
  semantic body;
- worldgen phenomena are sparse semantic contextual evaluation.

They share hierarchical/contextual *principles*, but they do not share one
runtime source/cache/query representation contract.

The experiment-41 generic `UsfFieldSourceLocation` /
`UsfHierarchicalFieldCache<S>` remains correctly deleted.

### Gravity contract retained

`GravityFieldQuery` remains the consumer seam.

Current production evaluation is deliberately the exact reference backend:

```text
RadialGravitySource semantic facts
           |
           v
GravityFieldQuery::sample()
           |
           +--> exact direct analytic superposition
```

`GravityFieldQuery::sample_exact()` is explicitly retained as the correctness
oracle for any future optimized backend.

`GravitySample` now records which evaluation class produced it and the number of
authored sources evaluated. This is diagnostic metadata, not semantic gravity
identity.

`RadialGravitySource` exposes gravitational parameter:

```text
μ = surface_gravity * radius²
```

For a spherical source this is the exterior-field quantity a future aggregate
representation should compose. Future approximation should not weight sources by
arbitrary ECS or authoring-scale metadata.

### Approximation acceptance contract

A hierarchical gravity backend is not earned until at least one concrete
pressure exists, such as measured fixed-tick cost or world generation producing
enough simultaneously relevant sources that exact direct evaluation is no
longer acceptable.

Before such a backend may replace exact sampling for a query class, it must
define:

1. **query semantics** — acceleration only, or also potential/gradient/tidal
   information if a real consumer needs them;
2. **spatial validity** — which canonical region/context one summary represents;
3. **opening/refinement rule** — when a coarse summary is acceptable and when a
   child/local refinement is required;
4. **error metric** — at minimum a bounded/estimated acceleration error useful
   to the consumer;
5. **currentness/version** — what semantic-source revision the representation
   reflects;
6. **composition** — no parent/child double counting; a fine query means inherited
   coarse contribution plus local residual/refinement, or an equivalent
   non-overlapping traversal;
7. **reference validation** — automated comparisons against
   `GravityFieldQuery::sample_exact()`.

The canonical USF hierarchy may be an efficient index/representation substrate,
but runtime `UsfContextResidency` must not define whether gravity physically
exists. Residency is runtime responsibility, not field truth.

### Important tranche-4 realization correction

The intent audit also exposed a concrete regression in tranche 4.

`SpatialRefinementDemand::None` was interpreted by celestial voxel realization as
"realize no celestial surface at all". Navigation legitimately clears explicit
approach refinement outside an active approach, so ordinary local spatial
interest could lose every voxel realization while semantic celestial bodies,
gravity and gameplay state continued to exist.

That exactly permits symptoms such as:

```text
semantic body still exists
gravity still exists
"landed" / other gameplay state still exists
voxel surface/collision/presentation disappears
```

Correct semantics are:

- generic spatial interest still requests the capability's ordinary realization
  in the source's current Scale Slice;
- `SpatialRefinementDemand` only requests *additional multiscale refinement*;
- absence of explicit refinement must never mean absence of ordinary
  realization.

This is corrected in the same batch.

### Follow-on intent work

The next high-value intent audit should focus on **simulation-state coherence**:
states such as landed/grounded/inhibited/navigation-reference must be outputs of
or explicitly reconciled against the physical facts they claim to summarize.
The observed "landed while physically falling because collision realization is
gone" symptom is a useful case study, but it does not require abandoning the
completed roadmap or turning presentation/residency into physics authority.
## 2026-09-24 celestial surface-persistence blocker

Three independent handoff bugs could intentionally create a black/physics gap:

1. A coverage-gated spatial transition applied its requested view exponent before
   checking REALIZATION/COLLISION coverage.
2. `UsfScaleFallbackPresentation` hid the far celestial body from view scale
   alone, without checking whether the replacement local voxel presentation was
   actually ready.
3. The reference spacecraft spawned independently in `PostStartup`, took local
   control, and disabled the player's spatial demand before the procedural
   surface-arrival transaction necessarily had collision coverage.

Corrections:

- view mutation attached to a one-shot spatial transition commits only after its
  coverage gate succeeds;
- far celestial fallback stays visible until the active interaction slice has
  PRESENTATION coverage from the same semantic celestial authority around the
  observer;
- procedural spacecraft acquisition waits until the controlled player is in the
  detailed body slice and current local celestial collision coverage exists;
- spacecraft acquisition runs after generic spatial-demand collection, so the
  final player-demand frame and first ship-demand frame cannot create a
  zero-demand gap.

The broader state-claim coherence wave is intentionally postponed until the
physical/visible world is stable enough to validate it.
## 2026-09-24 voxel representation-space / floating-origin correction

The post-part-46 visual failure exposed three representation-contract bugs.

### Small square moon

The local voxel square is a bounded refinement patch. Its existence is normal.
The bug was treating that patch as a global replacement for the whole-body
presentation.

USF composition is restored to:

```text
persistent macro body / inherited context
                 +
bounded local voxel refinement aperture
```

The far/macro celestial body no longer receives
`UsfScaleFallbackPresentation`. Refinement does not delete ancestry.

### White terrain

Local celestial voxel terrain had been switched from the high-contrast
development grid to profile materials (`lunar_surface`, cracked clay, etc.).
That hid chunk lineage/seams precisely while the representation pipeline is
being debugged.

Local voxel terrain now uses `ProceduralAssetLibrary::debug_grid` again.
Profile materials remain on macro-body presentation.

### Massive wobble

The canonical position subtraction path was audited and is not performing
astronomical float subtraction: `UsfPosition::relative_at_scale_bounded`
subtracts/normalizes the hierarchical decimal representation before emitting a
bounded local `Vec3`. Bounded f32 remains the intended backend endpoint.

Two authority bugs were found instead:

1. both player and spacecraft could own `UsfSpatialAnchor` after control
   transfer; `rebase_local_frame` then arbitrarily chose `anchors.iter().next()`;
2. voxel manifestation roots are Avian static rigid bodies, but runtime
   reprojection wrote only Bevy `Transform`, while Avian also owns/synchronizes
   `Position`.

Corrections:

- `UsfSpatialAnchor` now follows the same local-control focus transaction as
  `UsfViewAnchor`, `LocalViewTarget` and `UsfInteractionProjection`;
- spacecraft does not pre-own the spatial anchor before control transfer;
- local-control audit requires exactly one spatial anchor on the controlled
  manifestation;
- floating-origin rebase fails closed instead of choosing arbitrarily if
  duplicate anchors ever reappear;
- voxel manifestation creation and reprojection write identical bounded local
  coordinates to both `Transform` and Avian `Position`;
- rebase threshold/quantum names now say `NATIVE`, because Transform values are
  Scale-Slice-native units, not universally metres.

This preserves the intended precision model:

```text
canonical hierarchical position = authority
        ↓ subtract canonically
bounded Scale-Slice-local value
        ↓
f32 Transform / Avian Position = disposable backend representation
```

The next voxel robustness work should audit manifestation retirement and
coverage publication, but only after this representation-space contract is
runtime-stable.
