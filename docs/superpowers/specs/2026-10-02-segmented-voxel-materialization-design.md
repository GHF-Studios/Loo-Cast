# Segmented Voxel Materialization and Capability-First Scheduling

Date: 2026-10-02  
Primary issues: #5, #50  
Related: #28, #37, #49  
Explicitly deferred: #47

## Status

Design approved in conversation on 2026-10-02. This document freezes the architecture before implementation.

The design is grounded in remote `GHF-Studios/Loo-Cast` HEAD
`fdc8fb26d8b9ce055cc7eb16ccc581433aed94a6`. Local uncommitted work may be
newer; implementation must re-read the live local tree before editing.

## Intent

Make voxel streaming/materialization genuinely sparse and capability-driven.

A canonical voxel materialization address must not imply:

- dense sampled voxel storage;
- a derived surface;
- a Bevy runtime entity;
- a render mesh;
- a collider;
- editing state.

Those are independently acquired, retained, invalidated, and retired
capabilities/representations.

The practical goal is that vast regions which are provably air cost only compact
metadata, while nearby useful terrain obtains collision, dense data, and
presentation ahead of irrelevant work.

## Why

The current dense backend already separates store-owned chunk data from ECS
manifestations, and known-empty dense chunks skip Surface Nets. However:

1. generation still constructs the full padded dense `VoxelChunk` before
   discovering uniform air/solid;
2. derived-current empty chunks deliberately retain one
   `VoxelMaterializationRuntime`;
3. manifestation rebuild creates a root plus opaque presentation child even when
   no mesh exists;
4. generic capability coverage is published through one
   `UsfCapabilityRealization` component per runtime manifestation;
5. generation ordering is primarily a per-world FIFO queue of demanded chunks,
   while global scheduling ranks worlds rather than the most useful capability
   work item.

As a result, an address that is useful only as the fact "this space is empty" can
still consume dense memory, ECS cardinality, publication work, and scheduling
opportunity.

## Ownership invariants

### Canonical / semantic

- Canonical USF topology and semantic authority remain unchanged.
- A voxel materialization key is an address in a disposable realization backend,
  not semantic identity.
- Demand remains intent; it does not itself create data or authority.
- Coverage remains realized fact.
- Collision readiness never grants canonical collision-response authority; #49
  continues to own collision episodes and exactly-once response.

### Runtime realization

- The store owns sparse realization state.
- ECS exists only for runtime consumers which actually require ECS participation.
- Presentation and collision consume store facts independently.
- Dense sampled data is a representation, not the definition of a resident
  materialization.
- Empty-space knowledge is valid realization/coverage without requiring a mesh
  or per-address ECS entity.

### Performance

- Work scales with useful changed capability state, not the volume of a nominal
  chunk window.
- Stable state approaches zero recurring work.
- Large legitimate demand remains allowed.
- Conservative classification may skip expensive work only when it can prove the
  result. Uncertain cases fall back to exact existing generation.

## Core data model

Replace the effective assumption:

```text
address -> pending/dense -> derived surface -> runtime entity
```

with segmented store state:

```text
VoxelMaterializationEntry
├── active demand metadata
├── occupancy / field classification
│   ├── Unknown
│   ├── UniformAir
│   ├── UniformSolid
│   └── Mixed
├── dense samples                 optional
├── derived surface               optional
├── derived revision/status
├── requested capability roles
├── fulfilled capability roles
└── in-flight segment revisions
```

Exact names may differ after local-tree inspection, but the ownership boundaries
must remain.

### Occupancy classification

`UniformAir` and `UniformSolid` are compact facts.

For the first tranche, classification is deliberately conservative and biased
toward easy wins:

- `VoxelBase::Empty` can prove air immediately;
- celestial fields may prove air when the complete materialization AABB lies
  outside a conservative maximum possible body/surface bound;
- similarly safe uniform-solid classification may be added only where a strict
  lower bound exists;
- edited/intersecting/uncertain regions remain `Unknown` and use exact dense
  generation.

Classification must account for all semantic edits relevant to the address.
An edit which could intersect an otherwise uniform address invalidates the
uniform shortcut.

### Dense state

Dense `VoxelChunk` allocation exists only for addresses that require exact local
samples and cannot be fulfilled by compact classification.

A uniform result discovered during exact dense generation should be eligible to
collapse back to compact `UniformAir` / `UniformSolid` state when doing so does
not discard required editing information.

### Derived surface

Surface derivation exists only for `Mixed` dense state with a surface
transition.

There is no surface cache object for uniform air/solid.

### Presentation

A presentation entity exists only when all of the following are true:

- PRESENTATION is requested;
- the current representation owns actual presentable triangles;
- that presentation is admitted by current presentation policy.

No empty root entity. No empty opaque child.

Presentation entity lifetime is therefore proportional to actual presented
geometry, not realization coverage cardinality.

### Collision

Collision continues to consume store-owned rigid derived surfaces independently
of presentation.

The existing aggregate collider direction is retained:

- empty/uniform-air materializations never enter collision aggregates;
- non-rigid surfaces never enter rigid collision;
- nearby requested rigid surfaces may be grouped as today;
- presentation existence is not a collision prerequisite.

### Editing

Editing demand may require dense/local mutable representation even if
presentation/collision do not.

The implementation must keep editing as an independently requested role instead
of making "editable" synonymous with "rendered."

## Compact capability coverage

The current generic contract expects `UsfCapabilityRealization` components on
concrete disposable runtime entities. That is too expensive for materialization-
granular empty-space facts.

Voxel should publish compact coverage from store state through a small number of
ECS publishers rather than one entity per address.

Initial implementation:

- one voxel-world-owned capability publisher component/resource contains compact
  realized coverage records;
- adjacent axis-aligned materialization cells with identical authority, scale,
  revision compatibility, and fulfilled role mask may be greedily coalesced into
  cuboids;
- `UniformAir` may publish REALIZATION/PRESENTATION readiness where semantically
  appropriate without inventing render entities;
- COLLISION is published only for current collider-backed rigid coverage;
- EDITING is published only where the editing representation is current.

The generic `UsfScaleCoverageSnapshot` should gain a producer path for batched
coverage records rather than forcing every fact through one component/entity.
Existing entity-backed producers remain supported.

This is a generic pressure earned by the voxel consumer, but the implementation
should be the minimum extension required; it must not become #47.

## Demand state

Current `VoxelStreaming` already has useful incremental changed-slab logic.
Preserve that property.

Replace queue authority based on one `VecDeque<DemandedChunk>` with keyed demand
state plus versioned scheduling tickets.

Per address, retain compact demand metadata:

```text
VoxelDemandState
- roles
- semantic priority
- distance / locality metric
- migration requirement
- revision/version
```

Changing demand updates the keyed state. It does not eagerly remove old heap
entries.

## Capability work tickets

Scheduling operates on work needed to fulfill a role, not merely "a chunk needs
loading."

Conceptual ticket:

```text
VoxelWorkTicket
- world
- materialization key
- segment/work kind
- demand revision
- role criticality
- semantic priority
- migration urgency
- reuse/readiness state
- distance/locality
- deterministic sequence/address tie break
```

Work kinds initially include:

- classify;
- dense-generate;
- surface-derive;
- presentation-publish;
- collision-publish/reconcile if it later benefits from the same scheduler.

Do not force collision aggregate rebuilding into the heap in tranche one if the
existing revision-driven reconciliation remains cheaper and cleaner.

## Scheduling order

The scheduler compares useful work items globally, not merely worlds.

Ordering, highest need first:

1. current interaction COLLISION / COLLISION_QUERY prerequisites;
2. EDITING/current direct interaction prerequisites;
3. make-before-break migration work required to retire an older committed state;
4. current PRESENTATION work;
5. ordinary REALIZATION/background work;
6. higher semantic demand priority;
7. reuse/readiness advantage;
8. lower distance / earlier encounter;
9. deterministic address/sequence tie-break.

Exact role weights are implementation details; ordering semantics are not.

High-speed predictive demand supplies ordinary urgency/deadline metadata. The
voxel scheduler does not hard-code spacecraft behavior.

## Queue structure

Use a priority heap plus keyed current demand/revision state.

When state changes:

1. update the keyed record;
2. increment its version;
3. push new tickets as necessary.

When a ticket is popped:

- discard it if its version is stale;
- discard it if the requested segment is already fulfilled;
- discard it if demand disappeared;
- otherwise admit useful work.

This avoids O(queue) deletion and preserves the current incremental-planning
principle.

## Work pipeline

Target pipeline:

```text
demand delta
    ↓
keyed role/urgency state
    ↓
CLASSIFY ticket
    ├── proven UniformAir/Solid
    │      ↓
    │   compact coverage
    │      ↓
    │   DONE unless another role specifically needs dense state
    │
    └── Unknown
           ↓
      DENSE-GENERATE ticket
           ↓
      dense result classification
           ├── uniform → compact state
           └── mixed
                 ↓
            SURFACE-DERIVE ticket
                 ↓
          store-owned surface cache
             ├── PRESENTATION requested → actual mesh/entity publication
             └── COLLISION requested    → aggregate collider reconciliation
```

Every stage is versioned and stale-result safe.

## Role-specific materialization

A requested address should materialize only the minimum segments needed by its
roles.

Examples:

### Presentation-only air

```text
classification -> UniformAir -> compact PRESENTATION/REALIZATION coverage
```

No dense chunk, no surface, no ECS manifestation.

### Presentation-only mixed terrain

```text
classification -> dense -> surface -> render entity
```

No collider unless COLLISION is demanded.

### Collision-only mixed terrain

```text
classification -> dense -> surface -> collision aggregate
```

No render entity.

### Editing-only materialization

```text
classification/dense as needed -> editable dense state
```

No mesh/collider unless separately demanded.

### Presentation + collision

Shared dense/surface work feeds both downstream consumers; they remain separate
publication/lifetime owners.

## Classification API

Classification belongs to semantic/base-field preparation, not to streaming
policy.

Introduce a conservative domain query similar in spirit to:

```text
classify_materialization(address, semantic state) -> ProvenUniform | Unknown
```

It may use cheap analytic bounds, but it must not duplicate the full exact
sampler.

Celestial first-pass proof should use:

- body radius;
- maximum outward/inward relief bounds already defined or tightened in the
  celestial field;
- canonical materialization AABB/radial distance bounds;
- edit intersection.

The classifier must never label potentially mixed geometry as uniform.

## Store transitions

Required transitions:

```text
Absent
  -> PendingClassification
  -> UniformAir / UniformSolid
  -> PendingDense
  -> DenseMixed
  -> DerivedSurface
```

Not every implementation needs literal enum variants for every transient state;
in-flight tokens may remain orthogonal fields. The semantic transition behavior
must be explicit and testable.

Retirement is per segment:

- presentation may retire while dense remains warm;
- collision may retire while surface remains warm;
- dense may collapse/evict while compact uniform classification remains;
- all disposable state may retire when demand disappears.

Warm-cache policy should distinguish cheap compact entries from expensive dense
entries instead of counting both identically.

## Manifestation redesign

Delete the invariant documented today as:

> A derived-current empty result deliberately keeps a meshless runtime so
> capability coverage can represent known-empty presentation truth.

That responsibility moves to compact store-backed coverage.

`VoxelMaterializationRuntime` becomes presentation-specific (or is renamed to
make that fact explicit).

A runtime manifestation is created only for a surface that produces a
presentation mesh.

Translucent and opaque children remain optional products of actual geometry.

## Generic coverage extension

The generic spatial capability layer should support both:

- entity-backed `UsfCapabilityRealization`;
- batched producer-backed realized coverage.

One minimal design is a component on the voxel world containing a revision plus
`Vec<UsfScaleCoverageRecord>` consumed by the snapshot reconciler.

The snapshot remains the consumer-facing API.

Requirements:

- no consumer should care whether coverage came from an entity or a batch;
- snapshot revision changes only when effective coverage changes;
- batched records carry authority, scale, center/extent, roles, and revision;
- deterministic ordering before equality/reconciliation;
- producer removal retires its records automatically.

## Backpressure

#50 remains the owner of worker/publication backpressure.

This pass must plug segmented work into the existing durable voxel execution
domain rather than reintroducing one-future-per-chunk scheduling.

Classification should normally execute synchronously only when demonstrably
cheap and bounded; otherwise it belongs in the worker domain.

Main-thread publication budgets remain bounded.

Priority must affect admission before CPU-heavy dense work begins.

## Telemetry

Add counters/gauges sufficient to prove the architecture:

- demanded addresses;
- compact UniformAir count;
- compact UniformSolid count;
- dense resident count;
- mixed dense count;
- surface-bearing count;
- presentation entity count;
- collision aggregate/member count;
- queued work by kind and role class;
- stale tickets discarded;
- classification hits/misses;
- dense generations avoided by classification;
- dense uniform results collapsed;
- oldest queued age by criticality;
- useful completions by stage.

The important ratios:

```text
presentation entities / demanded addresses
dense chunks / demanded addresses
dense generations avoided / classifications
```

should make empty-volume savings obvious.

## Migration sequence

### Tranche A — store segmentation

- Introduce compact occupancy/materialization state.
- Add safe uniform classification.
- Preserve existing exact dense generation as fallback.
- Collapse exact uniform results when safe.
- Add tests for classification correctness and edit invalidation.

No scheduler rewrite is required before this compiles.

### Tranche B — remove empty ECS manifestations

- Extend generic coverage snapshot with batched producer records.
- Publish voxel compact coverage from store state.
- Stop creating runtime roots/presentation children for no-surface results.
- Make manifestation explicitly presentation-only.
- Keep collision aggregate path store-owned.

This should immediately reduce ECS cardinality.

### Tranche C — capability-first priority scheduler

- Replace pending FIFO scheduling authority with keyed demand state + versioned
  priority heap.
- Create classify/dense/derive work tickets.
- Rank tickets globally by capability need.
- Retain stale-ticket lazy rejection.
- Feed existing durable worker lanes and publication budgets.

### Tranche D — cleanup and tune

- Remove obsolete pending-queue fields/commentary.
- Separate warm limits for compact vs dense state if runtime evidence justifies
  it.
- Add diagnostics/HUD/console exposure for the new telemetry.
- Profile and tune priority constants from runtime evidence.

## Testing

### Unit tests

Classification:

- `VoxelBase::Empty` always classifies uniform air absent intersecting edits;
- conservative celestial exterior AABBs classify air;
- all boundary/near-relief cases remain Unknown unless strictly proven;
- intersecting edits invalidate the shortcut;
- same semantic input yields deterministic classification.

Store:

- uniform entries require no dense allocation;
- mixed entries retain exact old dense behavior;
- uniform dense result may collapse safely;
- segment retirement does not retire unrelated segments;
- stale worker output cannot resurrect retired/version-mismatched state.

Coverage:

- known-empty cells publish coverage without manifestation entities;
- coalescing preserves the exact union of cell coverage;
- differing roles/revisions/authority/scale never coalesce;
- batched producer removal retires snapshot coverage;
- snapshot consumer APIs behave identically for entity-backed and batched facts.

Manifestation:

- empty surface => zero presentation entities;
- opaque-only surface => only required presentation entity/entities;
- collision-only demand => zero presentation entities;
- presentation retirement leaves warm store data intact.

Scheduler:

- collision-critical near work outranks far presentation/background;
- migration-critical replacement outranks ordinary background;
- stale heap tickets are ignored;
- demand removal before execution avoids dense generation;
- deterministic ties produce deterministic ordering;
- one world may consume multiple worker admissions when it owns the highest-value
  work.

### Integration/runtime validation

Compilation is necessary but insufficient.

Required owner/runtime evidence:

1. Standing above planetary terrain with a large 3D demand volume produces many
   demanded addresses but only a small number of presentation entities.
2. Increasing empty-air demand volume does not grow render entities linearly.
3. Dense resident count grows near useful/uncertain terrain, not throughout the
   air volume.
4. Nearby collision-critical terrain becomes ready before farther presentation
   work under backlog.
5. Movement does not cause an O(resident-volume) queue rebuild.
6. Empty-space classification materially reduces dense generation throughput
   requirements.
7. Collision behavior remains unchanged for equivalent ready rigid surfaces.
8. Editing still forces the exact representation it requires.
9. Tracy shows no new unbounded main-thread publication burst.
10. Large demand backlog remains bounded and observable.

## Error handling / safety

- Classification failure means Unknown, never "probably air."
- Invalid canonical conversion leaves previous committed capability intact where
  make-before-break requires it and records diagnostics.
- Worker errors/stale outputs do not mutate semantic state.
- Coverage publication only claims roles whose required representation is
  current.
- No optimization may turn presentation readiness into collision authority.
- No broad repository reset/compatibility shim should be used during migration;
  adapt current local code explicitly.

## Non-goals

This design does not:

- restore or integrate Rhai terrain scripting;
- redesign the semantic terrain formula;
- implement #47's universal sparse resolution-domain backend;
- implement final velocity-cone geometry;
- change USF canonical topology;
- change the 71 semantic Scale Slice model;
- make binary presentation resolution own semantic scale;
- redesign collision episodes (#49);
- replace the Transvoxel/regional presentation architecture;
- reduce legitimate demand merely to hide throughput problems.

Terrain scripting integration is a separate follow-up: scripts should eventually
be a first-class policy/semantic generation layer feeding the same field and
classification contracts, not a presentation-only experiment.

## Files/boundaries expected to change

Exact paths must be revalidated against the live local tree before
implementation. Expected pressure points include:

- `spacetime-engine/src/voxel/store/*`
- `spacetime-engine/src/voxel/chunk/*`
- `spacetime-engine/src/voxel/world/recipe/*`
- `spacetime-engine/src/voxel/base/*`
- `spacetime-engine/src/voxel/streaming/*`
- `spacetime-engine/src/voxel/manifestation/*`
- `spacetime-engine/src/spatial/capability/*`
- voxel config/telemetry/devtools tests as required

## Issue ownership

### #5 — voxel capability/materialization truth

Owns:

- segmented materialization state;
- classification;
- role-specific realization;
- compact voxel coverage;
- removal of empty per-chunk manifestations.

### #50 — execution throughput/backpressure

Owns:

- capability-first ticket scheduling;
- worker admission priority;
- bounded queues/publication;
- telemetry and runtime throughput proof.

### #47 — remains QUEUED

This pass must not generalize the solution into a universal sparse-domain
framework. If another non-voxel consumer later needs the same orchestration, #47
can extract it from proven pressure.

## Done when

The voxel backend can represent large demanded 3D volumes without materializing
every address into dense memory and ECS.

Specifically:

- provably empty addresses remain compact;
- no empty address receives a render manifestation;
- dense/surface/presentation/collision/editing are independently materialized;
- generic capability coverage can represent compact store-backed voxel facts;
- useful capability work is globally prioritized ahead of irrelevant chunk work;
- existing incremental demand planning and stale-work rejection are preserved;
- runtime evidence shows ECS/dense work scaling with useful geometry rather than
  demanded air volume;
- #5/#50 retain their existing semantic ownership boundaries.
