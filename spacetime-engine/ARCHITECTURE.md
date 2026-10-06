
# Spacetime Engine architecture and code-shape rules

These are ownership and maintainability rules, not formatting preferences.

## Semantic state, caches, and manifestations

A recurring engine rule is:

1. semantic state owns truth;
2. indexes/materializations are replaceable representations of that truth;
3. ECS/render/physics objects are runtime manifestations unless a domain
   explicitly says otherwise.

Disposable representations must not silently become semantic authority.

A runtime chart rebase changes representation only. Spatial preflight and the
physics backend resolve direct and attached-body Scale Slice membership through
the same rule before any origin shift is applied.

Canonical position construction, balanced carry arithmetic, bounded relative projection,
and decimal display live in separate USF position owners. A projected number is a
bounded adapter, never a replacement for the digit stack.

Runtime interactions must resolve Scale Slice membership before comparing
positions or converting physical distances. A split peer inherits its
authoritative realization's slice; unlayered runtime samples follow the chart
anchor. Authored motion bases follow rebases alongside their transforms.

Spatial transition requests and continuous interaction requirements carry the
same destination coverage contract. Admission reads realized coverage and
backend veto evidence; applying an admitted transition alone reanchors the
runtime chart and publishes the transition message. View context publishes a
coherent semantic/runtime/render observer snapshot without granting view state
authority over the interaction slice.

Canonical context residency is an ancestor-closed runtime responsibility
snapshot. Demand is normalized to discrete context ranges before rebuilding
that graph; movement inside the same ranges does not revise residency. Travel
influences are semantic navigation hints: boundary providers refine hard-body
distance, while the observer-local neighborhood cache selects candidates and
refreshes independently of collision or presentation authority.

## Terrain presentation and work

Celestial clipmap blocks are presentation requests over one canonical field.
The actual camera view remains in demand while predicted motion adds prefetch
interest. Camera exclusion does not prove field emptiness or remove collision
and editing residency.

The canonical field owns conservative radial bounds for shell and solid-core
rejection; cave inward support extends the lower bound. Sparse boundary samples
may guide refinement, but they cannot certify that a planned GPU block is
empty. A GPU dispatch acknowledgement records submitted work, not a geometry
readback or hardware completion fence. Publication still waits for the local
projection barrier.

GPU build admission rotates across authorities when frame limits are reached;
render-side processed build records are retired with their extracted blocks.
The admission limits bound work submission, not total GPU memory residency.
Clipmap publication advances in order: derive plan input, settle the plan,
admit bounded GPU work, receive a dispatch acknowledgment, project new shells,
then commit the complete frontier and retire old shells. The acknowledgment
alone never satisfies the projection barrier.

The live binary clipmap separates semantic-field planning, balanced frontier
transactions, GPU work admission/publication, and view projection. Its registry
and coverage snapshots are reconstructible presentation state. GPU descriptor
projection consumes one bounded block chart; render-world dispatch owns buffers,
pipelines and acknowledgments.

The resolution domain separates dyadic block topology, field classification,
and body-local view demand. The field cache is keyed by the canonical field and block; it does not
store a camera decision or moving surface focus. Broad
shell rejection is a conservative field proof. Fine probes distinguish an
observed boundary, a sample miss, and unavailable samples. A sample miss is
only a refinement priority signal, never a semantic empty certificate. The
planner retains a parent when a split admits no children, and rolls back a
split wave that cannot restore 2:1 balance.

Dense CPU voxels own editable samples and collision support; GPU Transvoxel
blocks own binary presentation over a bounded, quantized descriptor of the
same canonical field. The shared WGSL schema defines both GPU passes' buffer
layout. Numeric agreement is domain-limited by descriptor projection and f32
evaluation, and bitwise CPU/GPU parity is not assumed. Edited authorities
currently stay on the dense presentation path until an edit-aware GPU snapshot
exists.

Voxel streaming converts semantic demand into ranked materialization work.
Motion prediction biases a bounded sparse tube; it does not define canonical
residency. Region traversal, hot/warm reconciliation and work ranking have
separate owners. Local cached value noise and canonical lattice noise likewise
remain distinct; exact corner hashing, worker caches and diagnostics cannot
change field identity.

Voxel realization translates generic spatial demand into capability intent.
Celestial contact observation measures the semantic field and canonical motion;
its bounded scopes feed a sorted intent snapshot and canonical residency
requests. A separate resolver maps authority-and-Scale intent to disposable
voxel worlds. Snapshot ordering is owned by the snapshot, not by ECS systems.

The semantic `CelestialVoxelField` owns the authored radius, profile, seed and
terrain bandwidth. `CelestialFieldRealization` adapts that field to one bounded
Scale Slice for voxel sampling; its Scale does not select another planet.
Rocky morphology is composed of keyed features introduced at semantic Scales.
Those Scales describe when a feature enters refinement, while the feature's
recipe owns its shape and conservative bounds. Prepared presentation sampling
may cache Scale-derived parameters but uses the same residual noise law as
canonical field evaluation. The descriptive `worldgen` registry is separate:
it does not currently construct or replace the live celestial field.

Worker tickets distinguish pending output from terminal failure. A failed
generation reservation and a failed surface derivation remain unavailable;
neither may be published as an empty result. The worker pool keeps servicing
other jobs after an individual job panics. Dense and derived publication retain
their own revision checks and make-before-break coverage.

## Module boundaries

A module should have one coherent reason to change. Split a file when it mixes
independent concerns such as:

- domain model and file I/O;
- scheduling and expensive domain algorithms;
- semantic state and rendering/physics manifestations;
- policy/configuration and mechanism;
- generic engine primitives and game-specific adapters.

`mod.rs` should primarily document, compose, and re-export a subsystem. It
should not become the subsystem implementation.

The game adapters keep their model, runtime transaction, and presentation
owners distinct. Local control transfers semantic ownership before its view
focus adapter requests a chart handoff. Navigation travel profiles belong to
the subject; automatic presentation state belongs to the view. Player input
bindings translate devices to actions, while the sampled frame is the only
hardware snapshot consumed by gameplay. Flight telemetry observes resolved
locomotion, navigation, surface, and safety state.
The authored binding registry supplies defaults through the same mutable
binding API used by the console; command dispatch alone reads console binds
after focus arbitration. Debug freecam settings, local motion, and the
reversible view-only observer override are separate owners. Console
observation commands report position, presentation readiness, or runtime
history without mutating simulation authority.
Camera profiles are target-owned intent; local pose, contextual projection,
self visibility and physical FOV resolve that intent in presentation. The
script workbench owns document actions and committed revisions; its explorer
and editor issue actions and display current state.
Controlled flight resolves intent to a canonical SI velocity in its policy
module. Cruise owns throttle/steering policy; the commit adapter alone crosses
between semantic USF position, bounded runtime pose and local collision.

Spacecraft boarding proves physical SI reach across runtime Scale Slices before
requesting a semantic control transfer. Disembark proves a walkable standing
pose before mutating player position, constituency, or control. The flight HUD
only formats resolved telemetry and owns its presentation refresh cadence.

Developer scripts use a draft → compiled candidate → committed revision
lifecycle. The document owns that transition; the workspace owns live/default
storage and open documents; console and editor code are adapters. A compiled
candidate is never runtime authority until committed. Canonical collision
query contracts and their frame journal are separate from ECS collection and
provider scheduling; a candidate is evidence, not a collision response.
Collision topology keeps authored stencil state and a change-driven host index;
the reconciliation system alone replaces Avian colliders. Rectangular stencil
fitting corrects placement against a cuboid face, while the strict support
predicate only validates an unchanged placement. Both use the same face-axis
geometry without moving portal policy into the collision layer.

Developer Lab presets compose typed runtime-variable adapters. A preset
transaction captures previous effective values, rebuilds from baselines and
active assignments, and commits active state only after setters succeed.
Clearing a preset restores all captured baselines before dropping paths no
longer controlled by any active preset.

Meaningful domain/concept modules get explicit named module boundaries: for
example `physics`, `worldgen`, `player`, `portal`, or `voxel`. Generic role
names such as `types`, `systems`, `components`, `resources`, or `functions`
are support files inside a meaningful domain, not architectural domains of
their own and not dumping grounds for unrelated code.

Every semantically meaningful Rust module is represented by a directory with
a `mod.rs`, even when it currently contains only one source file. Role/support
files such as `components.rs`, `types.rs`, `systems.rs`, `resources.rs`,
`functions.rs`, `model.rs`, `state.rs`, `config.rs`, `plugin.rs`,
`registration.rs`, `input.rs`, `math.rs`, and `source.rs` may
remain plain files inside that semantic module. These names describe
implementation roles, not standalone architectural domains.

## ECS systems

Bevy systems are orchestration boundaries. A system should normally read its ECS
inputs, delegate domain work to focused Rust functions/types, then publish ECS
changes. Large algorithms, geometry, policy decisions, and collection
transforms should not live inside several-hundred-line systems.

There is no hard line-count rule, but a system above roughly 80-100 lines is a
strong signal that responsibilities should be extracted.

## Dependencies

Reusable domains (`spatial`, `physics`, `voxel`, `worldgen`, `ecs`, `geometry`)
must not depend on `game` implementation modules. `game` may adapt reusable
engine APIs to Loo Cast mechanics.

Developer tooling and diagnostics observe runtime state; they do not own
simulation truth.

## Configuration

Tunable policy belongs in the typed config layer, not as scattered constants in
implementation files. Configuration is grouped by semantic owner and validated
where external data becomes trusted runtime policy.

Changing runtime config must not change semantic identity unless that config is
explicitly part of persisted semantic state.

## Public API

Public items exist because another module/crate needs the abstraction, not
because `pub` is convenient. Prefer private or `pub(crate)` helpers until a
stable external contract exists.

Public types and non-obvious invariants should be documented at their ownership
boundary. Comments should explain why/invariants rather than narrating syntax.

## Errors and invariants

Use `Result` for recoverable boundary failures. Use `expect` only for invariants
established structurally whose violation is a programmer error; its message
should name the invariant.

Avoid catch-all fallback behavior that hides invalid state.

## Validation

This repository does not carry automated test targets or executable doctests.
Validate production changes with workspace builds, focused static checks,
shader and pipeline creation, and direct runtime inspection. Keep assertions
that enforce production invariants and diagnostics that explain runtime state.
