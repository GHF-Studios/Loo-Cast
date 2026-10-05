
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

Runtime interactions must resolve Scale Slice membership before comparing
positions or converting physical distances. A split peer inherits its
authoritative realization's slice; unlayered runtime samples follow the chart
anchor. Authored motion bases follow rebases alongside their transforms.

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
