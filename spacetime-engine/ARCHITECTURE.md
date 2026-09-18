
# Spacetime Engine architecture and code-shape rules

These are ownership and maintainability rules, not formatting preferences.

## Semantic state, caches, and manifestations

A recurring engine rule is:

1. semantic state owns truth;
2. indexes/materializations are replaceable representations of that truth;
3. ECS/render/physics objects are runtime manifestations unless a domain
   explicitly says otherwise.

Disposable representations must not silently become semantic authority.

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

## Tests

Tests are intentional, not ceremonial. Prefer them for canonical addressing and
precision invariants, configuration boundaries, geometry/topology edge cases,
deterministic refinement behavior, and regressions that are difficult to
validate reliably through interactive use.
