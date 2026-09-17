# USF Spatial / World Generation Roadmap

**Status:** continuity-grade architectural working plan<br>
**Current implementation baseline:** `GHF-Studios/Loo-Cast` commit `64e93af51410237d5b89b48a41c45dcfe554c1ea`<br>
**Current phase:** M7 complete; M7.1a complete; M7.1b Pass A decimal materialization core complete; Pass B canonical voxel queries/edits is next. Spatial demand remains deferred to M7.2 / Pass D.

This document is intentionally narrower than a complete Universal Simulation Framework specification. Its purpose is to preserve the spatial, realization, generation, and near-term implementation decisions that must survive context loss, handoff to another agent, or future refactoring.

Where a name is marked **working name**, preserve the concept even if the identifier changes.

---

## 0. Terminology discipline

Several concepts are easy to collapse accidentally. Do not.

Keep these distinct:

- full future USF `Scale` vs the current spatial decimal exponent,
- canonical USF space vs bounded runtime Bevy/Avian coordinates,
- USF Chunk vs representation-specific materialization chunks/blocks,
- chunk identity vs an allocated chunk object,
- semantic state vs realization/materialization policy,
- Phenomena vs Metrics vs realization/cache structures,
- spatial demand/interest vs generation,
- spatial manifestations vs logical/physics/presentation projections,
- origin rebasing vs zoom/scale transition,
- simulation authority vs materialization authority,
- semantic hierarchy vs GPU/SIMD/cache-local batching,
- top-down generation vs future upward reaggregation.

If a design makes two of these become the same thing merely for convenience, treat that as suspicious.

---

# 1. Canonical spatial laws

## 1.1 Spatial scale spine

USF currently has **71 canonical spatial scales**, inclusive from **+35 through -35**.

- Scale `0` uses metres as its native spatial unit.
- Moving one scale upward multiplies native unit size by exactly `10`.
- Moving one scale downward multiplies native unit size by exactly `0.1`.
- The decimal progression is semantic and must not be changed to powers of two for implementation convenience.
- `+35` and `-35` are intentional reality-driven bounds, not machine-word approximations.

Powers of two remain valid *inside representation implementations* where useful: GPU workgroups, SIMD batches, tree nodes, allocation slabs, etc. They do not redefine semantic space.

## 1.2 USF Chunk

A **USF Chunk** is the canonical first-level spatial partition at one spatial scale.

Every USF Chunk spans:

```text
1000 × 1000 × 1000 native units of its own scale
```

Because adjacent spatial scales differ by exactly `10×`, one USF Chunk at scale `S` contains exactly:

```text
10 × 10 × 10 child USF Chunks at scale S-1
```

There is no contradiction:

- `1000³` describes the chunk in its own native scale units.
- `10³` describes the number of child chunks one scale below.

Every child is itself `1000³` in the smaller native units.

The `+35` top level is itself a balanced `10 × 10 × 10` grid. The current semantic universe is therefore finite but deliberately absurdly large. `-35` is the current minimum semantic spatial resolution.

## 1.3 Canonical position stack

The preferred semantic model remains the legacy balanced decimal position stack:

- one spatial digit per scale,
- digit coordinates in `[-5, 5)` on each axis,
- a bounded local offset in `[-500, 500)` native leaf-scale units,
- explicit carry/borrow between levels,
- explicit root overflow behavior.

The semantic model matters more than the final Rust storage layout. It may later be packed or optimized without changing its meaning.

A canonical `UsfPosition` is **not a giant `Vec3`**. It should expose deliberate operations such as:

- normalize,
- translate in leaf/native units,
- get chunk/region address at a scale,
- move to parent/child address,
- compare/measure relative positions when representable in a bounded local chart,
- project into a `UsfSpatialFrame`,
- eventually transition the leaf scale.

Arbitrary global floating-point vector algebra is not the goal.

## 1.4 Chunk identity is virtual; allocation is sparse

A canonical USF Chunk **conceptually exists because its address exists**.

That does **not** imply:

- an ECS entity,
- a heap object,
- a `HashMap` entry,
- generated Phenomenon state,
- loaded geometry,
- a collider,
- a render mesh,
- or any other allocated runtime structure.

Critical invariant:

> **Conceptually addressable does not mean materialized.**

The hierarchy must remain sparse/virtual. Allocate a runtime object only when some subsystem has actual state, demand, or an attachment associated with that address/scope.

---

# 2. Semantic space versus runtime space

The universe is not represented directly in Bevy/Avian coordinates.

```text
canonical USF position
        |
        | project through local spatial frame
        v
bounded ordinary Vec3 / Transform / physics Position
```

Render, physics, audio, particles, and similar libraries should continue using ordinary numeric kernels in comfortable bounded ranges.

The semantic layer supplies the information required to interpret those local numbers as positions in the larger USF space.

Origin rebasing changes the **local chart**, not canonical world identity.

At M7 the active leaf remains fixed at spatial scale `0`.

---

# 3. Representation hierarchy is not the USF scale hierarchy

This is a newly explicit rule.

The canonical USF hierarchy gives universal semantic addresses. Runtime systems are free to build representation-specific subdivisions and aggregate blocks over that address space.

For voxel/materialized dense data, the preferred structural skeleton is **decimal**, not the current `32³` legacy prototype.

A useful working hierarchy at one fixed sampling scale is:

```text
10 × 10 × 10      base materialization chunk
100 × 100 × 100   aggregate block / "Megachunk" (working name)
1000 × 1000 × 1000 aggregate block / "Hyperchunk" (working name)
```

The names `Megachunk` and `Hyperchunk` are provisional. The important property is decimal alignment and composability.

A larger block is an **aggregation/processing scope**, not necessarily a new universal semantic object.

Examples:

- individually address one `10³` materialization chunk,
- process `10³` neighboring base chunks together as a `100³` block,
- process `1000³` cells/samples as one larger aggregate,
- subdivide an aggregate internally for GPU workgroups,
- build a physics acceleration structure over a different extent,
- let several such structures overlap.

The runtime must not require the smallest addressable unit to also be the allocation, SIMD, meshing, collider, or scheduling unit.

## 3.1 Why the old `32³` should disappear

The current `32³` voxel chunk was a useful prototype but should no longer shape the architecture.

Problems:

- it is not decimal,
- it does not align cleanly with a `1000`-native-unit USF Chunk,
- it tempts code to treat one implementation `IVec3` lattice as universe-wide authority,
- it couples semantic addressing to a particular meshing/cache choice.

The replacement should preserve the useful dense-field machinery while moving the structural skeleton to decimal boundaries.

## 3.2 Current base materialization sample/cell convention

Pass A resolves the near-term convention without making padding semantic:

- one base materialization chunk owns the half-open logical cell extent `[0, 10)³`,
- the current dense Surface Nets working allocation copies one neighboring lattice sample on every side,
- the current private storage shape is therefore `12 × 12 × 12` samples,
- that `12³` allocation is an extraction/cache detail and is **not** materialization identity.

The canonical materialization address remains decimal `10³` regardless of padding. Future representations may choose different private working storage without changing that address.

---

# 4. Spatial attachments

The canonical chunk hierarchy is primarily an **address and attachment skeleton**.

A runtime subsystem may attach data/structures to a canonical spatial scope.

Conceptually:

```text
canonical spatial scope
        |
        +-- Phenomenon state
        +-- Metric data
        +-- voxel materialization
        +-- render acceleration
        +-- physics acceleration
        +-- navigation structure
        +-- generation cache
        +-- streaming bookkeeping
        `-- other subsystem-specific structures
```

Multiple attachments may coexist over the same or overlapping spatial scopes.

A useful conceptual shape is:

```text
SpatialAttachmentScope
    anchor: canonical USF chunk/position
    scale: spatial scale
    extent: aligned spatial extent

Attachment
    scope
    subsystem-specific payload
```

This is a concept, not yet a frozen Rust API.

An attachment point does **not** have to be represented by an ECS entity. Depending on the subsystem it might be:

- an ECS component/entity,
- an entry in a sparse map,
- a task-owned immutable snapshot,
- a GPU allocation,
- an acceleration structure node,
- a transient derived cache.

The universal contract is canonical spatial identity/scope, not storage mechanism.

---

# 5. Authority boundaries

## 5.1 Phenomena do not control loading/materialization

This is explicit.

A **Phenomenon** is a domain concept carrying or generating domain-specific world state/behavior at appropriate scales.

Examples might eventually include matter distributions, thermal phenomena, atmospheric structures, geology, etc.

A Phenomenon may:

- define semantic state,
- consume Metrics,
- generate/refine child-scale state,
- inspect permitted neighbor context,
- expose information needed by realizers.

A Phenomenon does **not** inherently decide:

- what regions are currently loaded,
- what chunks become ECS entities,
- what voxel blocks are materialized,
- what render/physics caches are allocated,
- which observer receives detail,
- global memory/CPU/GPU budgets,
- what runtime acceleration structures exist.

Do not grant Phenomena a universal scheduler/materializer authority merely because they describe important world state.

## 5.2 Realization/materialization policy is separate

A separate realization/materialization layer is responsible for turning semantic world state into currently-needed runtime representations.

Conceptually:

```text
semantic world state
        +
spatial demand / interest
        +
budgets / policy
        |
        v
realization planner / materializer
        |
        +-- materialize required semantic state
        +-- create/update runtime attachments
        +-- batch work into useful aggregate blocks
        `-- retire disposable representations when no longer demanded
```

This layer may ask a Phenomenon how to generate the state required for a region, but that does not make the Phenomenon the global loading authority.

## 5.3 Simulation authority is separate again

The scale/entity that is authoritative for mutable simulation state is a separate concern from realization/materialization.

Future observer-relative and multi-scale simulation work must preserve this distinction.

---

# 6. Spatial demand / interest

We need a generic engine concept for things that request spatial realization.

Working name:

```text
SpatialInterestSource
```

or:

```text
SpatialDemandSource
```

The final name is open.

A source conceptually supplies some combination of:

- canonical center/anchor,
- relevant spatial scale(s),
- requested radius/extent,
- requested representation/capability classes,
- priority,
- enabled/disabled state,
- possibly budget hints.

It is a **request**, not ownership.

Multiple sources may overlap. Their demand should be merged/arbitrated rather than each source independently loading duplicate state.

Examples:

- player,
- camera/observer,
- AI simulation focus,
- portal destination,
- editor viewport,
- scripted world event,
- debug/test object,
- server/network interest region.

## 6.1 Player-toggleable chunkloading

The player should gain a toggleable spatial-demand capability for testing.

This is deliberately a test/debug-friendly first consumer of the generic demand model.

When enabled, the player's canonical position drives a configurable spatial demand scope.

When disabled, the player contributes no such demand except whatever minimum runtime requirements other systems independently impose.

Do not bake "the player is always the chunk loader" into the world architecture.

## 6.2 Chunkloading Cube

Duplicate/adapt the existing Damageable Cube playground item into a **Chunkloading Cube**.

The item should be physically placeable like the existing cube and carry a generic spatial-demand component.

Because it is a real movable spatial object:

- its demand follows its canonical semantic position,
- several cubes can coexist,
- overlapping demand can be tested,
- moving a cube tests demand migration,
- destroying/removing it tests demand retirement,
- portals/world wrapping can eventually pressure-test manifestation-aware demand.

The UI/item name may remain "Chunkloading Cube" even if the engine component is named more generally (`SpatialInterestSource`, etc.).

This is an important architecture test tool, not just a toy.

## 6.3 Initial demand acceptance test

The near-term test should allow:

1. toggle player chunkloading on/off,
2. place several Chunkloading Cubes,
3. visualize requested/materialized scopes,
4. walk away while cube-held regions remain demanded,
5. move/remove a cube and observe demand migrate/retire,
6. overlap two sources without duplicate semantic state,
7. ensure origin rebasing does not alter canonical demand locations.

---

# 7. World generation is hierarchical causality, not one giant noise function

USF should not evaluate one conventional floating-point noise function across 71 orders of magnitude.

Instead:

```text
higher-scale semantic state
        |
        | interpret/generate downward
        v
lower-scale semantic state
        |
        v
still lower-scale state
```

Noise is a local ingredient or seed at a particular scale, not the universal authority.

A scale-local generator may consume:

- direct parent state,
- parent-scale Metrics,
- permitted neighboring parent regions/metric samples,
- exact hierarchical child address,
- world/Phenomenon seed,
- scale-local procedural/rule-based algorithms.

All expensive geometric/noise work can therefore occur in bounded scale-local coordinates.

Generation and materialization are related but not identical:

- generation answers **what semantic state exists here?**
- materialization answers **what representations do we currently need here?**

---

# 8. Neighbor coherence

Child regions must not simply generate unrelated state if the domain requires continuity/coherence.

The framework should provide enough canonical neighbor/boundary context for the domain algorithm to solve this appropriately.

Possible mechanisms include:

- deterministic values attached to shared canonical boundaries/corners,
- deterministic neighboring seeds,
- direct neighbor inspection,
- interpolation/blending,
- bespoke continuous algorithms,
- constraint propagation,
- Wave Function Collapse / rule-based propagation,
- relaxation/PDE-like methods,
- domain-specific topology/graph algorithms.

**WFC is one possible tool, not a framework mandate.**

Different Phenomena may define completely different coherence laws.

The framework's responsibility is to make the relevant canonical context/addressing available.

---

# 9. Downward generation now; reaggregation later

Near-term generation is top-down.

Eventually we likely want a compatibility law roughly like:

> A finer realization should remain compatible with the coarser semantic state that caused it, and significant fine-scale changes should eventually be capable of affecting higher-scale state.

But **upward reaggregation/reconciliation is deferred**.

It is a major architecture problem of its own and should not be smuggled into the current milestones.

---

# 10. Voxel role

Voxels are one materialization of matter/field state, not the definition of the universe.

The current useful invariant remains:

```text
semantic source state
        |
        v
voxel field/materialization
        |
        v
dense working data
        |
        +-- render mesh
        `-- collision cache
```

Derived mesh/collision data are disposable.

Voxel edits/base state may be authoritative for the current prototype, but the architecture must allow later Phenomenon-backed generation to replace the temporary procedural terrain base.

## 10.1 M7.1a current state

As of baseline commit `1bff71b`:

- `VoxelWorld` has a canonical `UsfPosition` origin,
- materialized voxel bricks receive canonical semantic origin identity,
- derived meshes/colliders are brick-local rather than baking old world coordinates into vertices,
- runtime entity transforms perform local projection,
- origin rebasing therefore no longer needs to rewrite generated mesh vertex coordinates.

This was a useful projection-boundary proof.

However, its current `32³` chunk/address machinery is now explicitly transitional and should be replaced rather than expanded.

## 10.2 Target voxel addressing

The target should not have one universe-wide `IVec3`.

Use canonical USF position/chunk addressing for semantic location.

Within a bounded materialization chunk/block, use small ordinary integer/local coordinates.

Conceptually:

```text
canonical USF spatial scope
        |
        +-- decimal materialization chunk addresses
                |
                +-- small local sample/cell coordinates
```

Sparse edit/query indexes should be keyed by canonical spatial scope/address rather than a giant flat integer lattice.

---

# 11. Milestones

## M7 — Fixed-scale USF positioning — COMPLETE / PROVING

Purpose: prove semantic position and local origin rebasing without zoom.

Implemented/proven direction:

- 71-scale `+35..-35` decimal position representation,
- runtime leaf fixed at scale `0`,
- balanced digit normalization/carry,
- semantic player position,
- bounded local runtime chart,
- quantized origin rebasing,
- Bevy Transform + Avian Position rebasing,
- explicit local-cache rebase notification,
- portal local-history rebasing,
- F4 `USF Spatial` diagnostics.

Non-goals remain:

- zoom/scale transitions,
- player resizing,
- divergent observer scales,
- cross-scale generation,
- final persistence.

## M7.1 — Canonical decimal materialization

Purpose: remove the remaining flat/local-coordinate assumptions from voxel materialization and establish the spatial attachment substrate.

### M7.1a — Projection boundary — COMPLETE

- canonical `VoxelWorld` origin,
- canonical materialized-brick identity,
- brick-local render/collision geometry,
- local entity Transform as runtime projection.

### M7.1b — Decimal voxel/materialization skeleton — IN PROGRESS (PASS A COMPLETE)

Ambitious pass:

- retire the `32³` structural assumption,
- introduce a decimal `10³` base materialization chunk,
- support aligned aggregation into `100³`, `1000³`, and later other scopes,
- keep padding/private extraction details separate from logical chunk extent,
- migrate canonical sample/query positions,
- migrate edit centers/bounds to semantic positions/scopes,
- remove authoritative dependence on one global `VoxelChunkCoord(IVec3)`,
- key sparse edit/materialization indexes canonically,
- preserve async generation and derived-cache pipelines,
- allow aggregate work items to batch many base chunks for cache/SIMD/GPU efficiency,
- keep meshes/colliders local to the representation scope,
- ensure frame rebases only change projection, never semantic addresses.

Acceptance:

- large fixed-scale travel does not grow authoritative float coordinates,
- edits remain at the same semantic location through arbitrary rebases,
- unloading/rematerializing reproduces the same edited state,
- base `10³` chunks are individually addressable without requiring one runtime object per possible address,
- aggregate blocks can process many chunks as one work unit,
- no `32`-based semantic assumption remains.

Pass A implementation state:

- `MATERIALIZATION_CHUNK_SIZE = 10` is the logical base extent; the old `CHUNK_SIZE` name remains only as a hidden compatibility alias,
- Surface Nets padding/storage is private (`1` copied sample each side, currently `12³` storage),
- `VoxelMaterializationChunkAddress` is canonical USF identity for one base materialization chunk,
- `VoxelWorld`'s sparse reserved/materialized registry is keyed by canonical materialization address rather than `VoxelChunkCoord`,
- canonical addresses are derived with exact whole-native-unit carry and do not round-trip large lattice displacements through one `f32 Vec3`,
- async field generation, dense chunks, Surface Nets, local render meshes, local colliders, and existing streaming machinery remain intact,
- `VoxelChunkCoord`, flat `VoxelBounds`, brush centers, edit indexing, and generation sampling are explicitly transitional `VoxelWorld`-local compatibility seams for Pass B,
- aggregate processing and spatial demand have **not** been pulled forward.

## M7.2 — Spatial demand / chunkloading test harness

Purpose: prove that realization is demand-driven and independent from Phenomenon authority.

Implement:

- generic `SpatialInterestSource` / `SpatialDemandSource`,
- demand merger/arbitration sufficient for the current single-player test,
- player-toggleable demand source,
- Chunkloading Cube playground item,
- canonical demand following moving sources,
- unload/retire behavior when demand disappears,
- F4 visualization for requested vs materialized scopes,
- overlapping-source tests.

Do not overbuild global scheduling/budget policy yet; create the minimum clean contract that future policies can extend.

## M8 — Manifestation / projection generalization

Generalize current entity split while preserving two orthogonal dimensions:

```text
semantic entity
    |
    +-- spatial manifestation A
    |       +-- logical/physics projection
    |       `-- presentation projection
    |
    `-- spatial manifestation B
            +-- logical/physics projection
            `-- presentation projection
```

Portal/world-wrap multiplicity must not be collapsed into visual-vs-logical projection.

## M8.5 — First `10×` spatial scale transition

After fixed-scale addressing/materialization is robust:

- transition active leaf by exactly one decimal scale,
- remap local runtime chart,
- player scale may equal active simulation/view scale for this first proof,
- no multiplayer/divergent observer scales yet,
- preserve canonical identity through transition.

## M9 — Scale-local Phenomenon generation

Implement the first intentionally minimal root/domain Phenomenon, working example `MatterDistribution`.

Prove:

- parent semantic state -> child semantic state,
- exact hierarchical addresses,
- parent + neighbor context,
- bounded scale-local numeric algorithms,
- deterministic regeneration,
- coherence mechanisms,
- integration with the separate realization/demand layer.

Phenomena generate domain state. They do not become global loaders.

## M9.5 — 71-order synthetic-world torture test

Build a deliberately artificial matter/world structure that can be refined repeatedly through the scale stack.

From far away it should read as one coherent enormous structure; moving inward should reveal newly generated smaller-scale organization.

It does not need realistic galaxies, planets, geology, molecules, or atoms yet.

Its job is to prove:

- the 71-scale position hierarchy,
- top-down generation,
- spatial demand/materialization,
- scale transitions,
- local numeric kernels,
- neighbor coherence,
- representation aggregation.

Then stop and reassess before attempting the full universe simulation roadmap.

---

# 12. Immediate implementation order

If this conversation/context disappears, resume here.

**Baseline:** `64e93af51410237d5b89b48a41c45dcfe554c1ea`.

### Pass A — finish decimal materialization core — COMPLETE

Implemented in this pass:

1. Logical decimal `10³` base materialization extent.
2. Explicit materialization terminology distinct from USF Chunk identity.
3. Private Surface Nets padding/storage (`12³` currently).
4. Canonical `VoxelMaterializationChunkAddress` derived from USF spatial identity without a large-float round trip.
5. Sparse canonical materialization registry; no eager decimal hierarchy allocation.
6. Existing async generation / dense-field / Surface Nets / render / collider pipeline preserved.

### Pass B — canonical voxel queries/edits — NEXT

1. Introduce semantic/canonical voxel query position.
2. Make brushes/edit centers canonical + bounded local shape parameters.
3. Replace flat `VoxelBounds` authority with a canonical scoped/bounded representation.
4. Rebuild the sparse edit index around canonical chunk/scope keys.
5. Preserve global edit ordering semantics.
6. Ensure background generation snapshots contain semantic addresses, not frame-relative positions.

### Pass C — aggregate processing

1. Let work scheduling group many `10³` chunks into an aligned aggregate.
2. Working decimal aggregate sizes begin with `100³` and `1000³`.
3. Aggregation is a work/cache choice, not semantic identity.
4. Different subsystems may use different overlapping aggregation extents.

### Pass D — spatial demand

1. Add generic demand-source component/API.
2. Add player-toggleable demand.
3. Duplicate/adapt Damageable Cube into Chunkloading Cube.
4. Convert voxel streaming from hard-coded viewer logic toward merged demand.
5. Add F4 visualization of source scopes and materialized attachments.

### Pass E — fixed-scale torture

Test:

- repeated rebases,
- very large scale-0 translation,
- place voxel edits,
- leave/unload,
- return/rematerialize,
- multiple Chunkloading Cubes,
- player loading off while cube keeps a region alive,
- overlapping demand,
- portals where currently compatible.

Only then declare M7.1/M7.2 stable enough to approach M8.

---

# 13. Hard invariants / anti-goals

Do not:

- eagerly instantiate the full chunk hierarchy,
- let a Phenomenon become the global loader/materializer,
- treat the smallest addressable chunk as mandatory compute/allocation granularity,
- make one acceleration structure universal,
- prohibit overlapping spatial attachments,
- use one giant global `f32/f64 Vec3` as semantic position,
- replace that with one giant global `IVec3` and call the problem solved,
- let origin rebasing mutate semantic identity,
- let a power-of-two optimization redefine semantic decimal structure,
- conflate USF scale transition with render LOD,
- conflate spatial manifestations with presentation projections,
- implement upward reaggregation prematurely.

Prefer:

- canonical decimal addresses,
- sparse materialization,
- bounded local math,
- multiple coexisting spatial structures,
- explicit attachment scopes,
- independent demand/realization policy,
- domain-specific Phenomenon algorithms,
- aggressively disposable derived caches.

---

# 14. Open questions that remain genuinely open

These should be answered by implementation pressure, not speculative framework-building unless they become blockers.

1. Final Rust name/API for generic spatial attachment scope.
2. Final Rust name: `SpatialInterestSource` vs `SpatialDemandSource` or another term.
3. Whether future voxel representations need alternate private sampling/storage conventions beyond the current `10³` logical-cell + padded-lattice base representation.
4. Whether `Megachunk` / `Hyperchunk` are useful public concepts or merely debug/working names for generic aggregate extents.
5. How realization planners choose aggregation extent from CPU/GPU/memory/activity constraints.
6. How many representation classes the initial demand API should distinguish.
7. How portal/world-wrap manifestations should contribute demand when one semantic entity is simultaneously present in multiple spatial locations.
8. Persistence format for canonical attachments/edits.
9. Future upward reaggregation/reconciliation.
10. Divergent observer scales / multiplayer scale views.
11. Relativistic/causal simulation semantics.
12. Final full USF `Scale` model beyond the current spatial exponent.

Do not solve these merely to make the framework look complete.

---

# 15. Current conceptual picture

```text
                       CANONICAL USF SPACE
                              |
                    71 decimal spatial scales
                         +35 ... -35
                              |
                     canonical USF Chunks
                1000³ native units per scale
                  10³ child chunks per step
                              |
                   sparse address/attachment
                         skeleton only
                              |
          +-------------------+-------------------+
          |                   |                   |
      Phenomena            Metrics           Other semantic
     domain state         field/state            systems
          |                   |                   |
          +-------------------+-------------------+
                              |
                       generation/query
                              |
                              |   NOT loading authority
                              v
                   semantic state when needed

      Spatial demand sources                 budgets/policy
      player / cube / portal / ...                |
                 |                                |
                 +---------------+----------------+
                                 |
                                 v
                    realization/materializer
                                 |
             +-------------------+--------------------+
             |                   |                    |
         voxel data          physics accel        render/cache
             |                   |                    |
       decimal 10³ base      chosen extent       chosen extent
       chunks, optionally       independently        independently
       aggregated 100³,
       1000³, ...
             |
             v
       bounded local frame
             |
        Bevy / Avian / GPU
```

The important philosophical rule is:

> **Semantic space says what/where things are. Demand says what we currently need. Realization decides what runtime structures to build. Phenomena describe/generate domain state. None of those roles should silently absorb the others.**
