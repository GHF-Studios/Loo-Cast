# USF Spatial / World Generation Roadmap

Status: architectural working plan after the M0-M6 voxel prototype and the entity-split / portal prototype.

This document is intentionally narrower than a complete Universal Simulation Framework specification. Its job is to preserve the hard-won spatial and generation decisions that must survive the next implementation milestones.

## 1. Core spatial laws

### 1.1 Spatial scales

USF has **71 canonical spatial scales**, inclusive from **+35 through -35**.

- Scale 0 uses metres as its native spatial unit.
- Moving one scale upward multiplies native unit size by exactly 10.
- Moving one scale downward multiplies native unit size by exactly 0.1.
- The decimal progression is semantic. It is not changed to powers of two for implementation convenience.

Powers of two remain perfectly valid *inside* a scale for implementation structures such as voxel bricks, GPU workgroups, trees, caches, etc.

### 1.2 USF chunks / semantic regions

Every USF chunk spans:

```text
1000 x 1000 x 1000 native units of its own scale.
```

Because adjacent scales differ by exactly 10x, one chunk at scale `S` contains exactly:

```text
10 x 10 x 10 chunks at scale S-1.
```

This is not contradictory:

- `1000^3` describes a chunk in its **native spatial units**.
- `10^3` describes **hierarchical child chunks**.

Every child is itself `1000^3` in the smaller native units.

The +35 top level is also a balanced `10 x 10 x 10` grid rather than an unbounded integer plane. The current model therefore has a finite but deliberately absurd physical extent. The -35 scale is the currently intended minimum semantic detail scale.

### 1.3 Canonical position stack

The preferred semantic model is the legacy balanced decimal stack:

- one spatial digit per scale,
- digit coordinates in `[-5, 5)` on every axis,
- a bounded local offset in `[-500, 500)` native leaf-scale units,
- explicit carry / borrow between levels,
- explicit root overflow behavior.

The semantic model is more important than the physical Rust storage layout. A future implementation may pack or otherwise optimize the stack without changing its meaning.

A canonical USF position is **not a giant Vec3** and is not expected to support arbitrary vector algebra. It should expose deliberate spatial operations such as normalization, local translation, parent/child region access, relative projection, and eventually scale transitions.

## 2. Semantic space versus runtime space

The universe is not represented directly in Bevy/Avian coordinates.

```text
canonical USF position
        |
        | project through local chart
        v
bounded ordinary Vec3 / Transform / physics Position
```

Normal render and physics libraries should continue using ordinary `f32/f64` kernels in numerically comfortable ranges.

The semantic layer carries the information needed to interpret those local numbers as positions in a vastly larger (and eventually vastly smaller) universe.

Origin rebasing changes the **local chart**, not canonical world position.

## 3. Manifestations: two orthogonal concerns

Do not conflate these two splits.

### 3.1 Spatial/topological multiplicity

Portals and world wrapping can require one semantic entity to have multiple simultaneously functional spatial manifestations:

```text
UsfEntity
|- spatial manifestation A
`- spatial manifestation B
```

Both may participate in rendering, collision and other mechanics.

### 3.2 Logical / physical / presentation projection

A particular spatial manifestation may itself eventually have different local representations for logic/physics and presentation.

Therefore the long-term shape is conceptually:

```text
UsfEntity
|- Spatial manifestation A
|  |- logical / physical projection
|  `- presentation projection
`- Spatial manifestation B
   |- logical / physical projection
   `- presentation projection
```

M7 must not introduce assumptions that collapse these two dimensions into one relationship.

## 4. World generation is hierarchical causality, not one giant noise function

USF should **not** attempt to evaluate conventional floating-point noise over 71 orders of magnitude.

Instead:

```text
higher-scale state
      |
      | interpret / materialize downward
      v
lower-scale state
      |
      v
still lower-scale state
```

Noise is useful as a local ingredient or seed at a particular scale, not as the universal authority for every scale below it.

A child generator may consume:

- its parent phenomenon state,
- parent-scale Metrics,
- neighboring parent regions / metric samples,
- its exact hierarchical child address,
- world / phenomenon seed,
- scale-local procedural algorithms.

All expensive geometric and noise calculations can therefore operate in numerically sane bounded coordinates.

## 5. Phenomena own world meaning

Chunks are spatial support infrastructure. They do not define what the universe *is*.

The first real world-generation proof should use a minimal root phenomenon (working name: `MatterDistribution`) whose implementations at successive scales decide how parent state becomes child detail.

This directly exercises the intended USF model instead of building a temporary procedural-terrain framework that later has to be translated into phenomena.

## 6. Neighbor coherence

Child regions cannot simply generate unrelated local fields or visible seams will appear.

USF should expose enough context for each phenomenon to solve coherence using the algorithm appropriate to that domain. Possible techniques include:

- shared deterministic values on canonical boundaries/corners,
- deterministic neighboring seeds,
- direct neighboring parent/child inspection,
- interpolation / blending,
- bespoke continuous generators,
- constraint solving,
- Wave Function Collapse or other rule-based propagation,
- relaxation / PDE-like methods where appropriate.

**WFC is an available technique, not a framework requirement.** The framework contract is coherent use of spatial context/boundary information.

## 7. Downward generation now; reaggregation later

The eventual desirable law is:

> Finer realizations should be compatible with the coarser state that caused them, and significant fine-scale state should eventually be capable of reaggregating upward.

But **upward reconciliation is deliberately deferred**. It is a major problem of its own.

The near-term implementation is top-down generation only.

## 8. Voxel role

Current `32^3` voxel chunks are implementation bricks / dense materialization caches. They are **not USF chunks**.

That work remains valuable:

```text
USF phenomenon + semantic region
        |
        v
active-scale field materialization
        |
        v
small dense voxel bricks
        |
        +-> render mesh
        `-> collision cache
```

The current power-of-two-ish brick sizing is allowed internally even though it does not divide the semantic `1000^3` chunk cleanly. Canonical addressing of those bricks across semantic-region boundaries needs an explicit design rather than pretending their current `IVec3` is a universe coordinate.

## 9. Milestones

### M7 - Fixed-scale USF positioning

**Purpose:** prove semantic position and local origin rebasing without introducing zoom.

Scope:

- 71-scale `+35..-35` decimal position representation,
- runtime leaf fixed at scale 0,
- balanced digit normalization and carry,
- semantic player position on the `UsfEntity`,
- bounded local runtime chart,
- quantized origin rebasing,
- Bevy Transform and Avian Position rebasing,
- explicit local-cache rebase notification,
- portal traversal cache rebasing,
- F4 `USF Spatial` diagnostics showing local and semantic coordinates, velocity, frame origin and rebase history.

Non-goals:

- zoom / scale transitions,
- player resizing,
- observer scale separate from player scale,
- cross-scale generation,
- LOD,
- persistence,
- final canonical voxel addressing.

Acceptance:

- ordinary gameplay coordinates remain bounded while the canonical player position continues accumulating,
- repeated positive/negative rebases preserve semantic position continuity,
- portal local history receives the same chart translation,
- the number stack demonstrates `1000` native-unit chunks and `10` child chunks per axis algebraically.

### M7.1 - Canonical voxel projection

The existing voxel prototype currently embeds local/world `Vec3` and `IVec3` addresses in its procedural base, edit log and brick index. That cannot be considered infinite-world authority.

M7.1 should migrate voxel interfaces to consume semantic spatial context without redesigning voxel storage itself:

- canonical semantic sample/query position,
- canonical voxel edit centers/bounds,
- a brick-address adapter that does not assume one global `IVec3`,
- streamed bricks projected into the current local frame,
- chunk-local derived meshes/colliders,
- generation tasks invalidated/reprojected safely across a frame rebase.

M7 includes a narrow compatibility bridge: current voxel queries/streaming are interpreted in the stable local coordinate system of the `VoxelWorld` root, so ordinary origin rebases do not immediately invalidate them. This is **not** the final huge-world voxel address model; large semantic distances still require this M7.1 migration.

Only after this milestone should procedural terrain be treated as a true 71-scale large-distance travel proof.

### M8 - General manifestation / projection architecture

Generalize the current entity-split prototype while preserving the orthogonality described in section 3.

Do not force portal/world-wrap multiplicity to be the same abstraction as visual-versus-logical projection.

### M8.5 - First 10x scale transition

After fixed-scale spatial identity and manifestation projection are stable:

- move active leaf scale by one decimal band,
- for now player scale == active simulation/view scale,
- remap local runtime chart so player/nearby geometry stay numerically comfortable,
- no multiplayer / divergent observer scales yet.

### M9 - Scale-local phenomenon generation

Implement the first root phenomenon and child-generation contract:

- parent state -> child detail,
- exact hierarchical addressing for deterministic generation,
- parent + neighbor context,
- local bounded noise/algorithms only,
- debug visualization of USF chunk boundaries and coordinates in F4 tooling.

### M9.5 - 71-order synthetic world torture test

Build one deliberately synthetic, irregular matter/terrain phenomenon that can be refined repeatedly across the scale stack.

It should look like one coherent enormous object from far away and reveal progressively generated structure while zooming inward. It does **not** need realistic galaxies, planets, geology, molecules and atoms yet.

This test exists to prove the architecture before broadening world content.

Then stop and reassess.

## 10. Deferred topics

These are intentionally not near-term milestones:

- persistence,
- conventional render-distance / LOD bands,
- upward reaggregation,
- multiple independent observer scales / multiplayer-scale projection,
- relativity,
- physically accurate full-universe phenomena,
- final voxel sparse-storage hierarchy.

They depend on the spatial/generative substrate above and should not dictate premature abstractions now.
