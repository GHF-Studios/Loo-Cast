# RhaiDialect — Architecture and Design Decisions

## Intent

This project treats Rhai as a customizable runtime dialect for Bevy/ECS workflows:

- prioritize flexibility and iteration speed over compile-time ergonomics,
- keep unsafe internals explicit and boxed behind strict access patterns,
- accept manual monomorphization for generic-heavy APIs.

## Core architecture

### 1) Reflection and bridge declarations

- Canonical bridge declarations live in:
  - `core_mod_api/src/rhai_binding/bridges/domains/*`
  - `core_mod_api/src/rhai_binding/bridges/testing/*` (test-only surface)
- These modules are the reflection graph that `EngineExt::register_binding_graph` registers into Rhai.

### 2) Runtime wrappers and implementation layer

- Runtime wrappers live in:
  - `core_mod_api/src/rhai_binding/runtime/*`
- Canonical import path for bridge/runtime code is:
  - `crate::rhai_binding::runtime::*`

### 3) Access backbone (soundness boundary)

- `AccessCell` + `AccessCellProvider<T>` are the central borrowing/ownership abstraction.
- `start_access`/`end_access` pairs are required and must complete inside one synchronous system frame.
- Violations panic intentionally (fail-fast model).

### 4) Hook/runtime execution

- Hook runner: `core_mod_api/src/rhai_binding/engine/hook.rs`
- For each schedule hook, loader composes:
  1. recursively discovered companion `.rhai` files,
  2. then the hook root file.
- Startup entrypoint is kept at `startup.rhai`, with categorized suites under `startup/`.

## Value semantics model

Supported semantics vocabulary:

- `Clone`
- `Owned`
- `Ref`
- `Mut`
- `ScopedOwned`
- `ScopedRef`
- `ScopedMut`

Current policy direction:

- use `AccessCell` as the primary mechanism for runtime ownership/access semantics,
- avoid broad `Arc`-style sharing for mutable/owned semantics,
- allow dangling handles to exist and enforce safety at use-time (invalid access panics),
- treat `Scoped*` as lifetime-erased variants anchored by provider-managed system-frame access.

Detailed intent/lifecycle matrix: `docs/RhaiValueSemantics.md`.

## Generic support model

Rhai cannot create new Rust monomorphizations at runtime, so generic support is modeled as:

1. runtime descriptors (e.g. query data/filter descriptors),
2. dispatch-key resolution,
3. compile-time registered signature catalogs (`inventory`).

This gives dynamic selection with static safety boundaries.

## Query/Commands/Messages direction

- Sysparam-like features are exposed via the same AccessCellProvider pattern.
- Query/message behavior is signature-driven and catalog-backed.
- Bridge APIs should mirror Rust/Bevy concepts where practical (iterators/cursors instead of ad-hoc batch facades).

## Module organization rules

- `bridges/domains/*`: production runtime API surface mirrored by domain path.
- `bridges/testing/*`: explicit testing-only bridge space.
- `bridges/domains/bevy/ecs/catalog/*`: compile-time registries for signatures and providers.
- `core_mod/assets/scripts/core/schedule_hooks/startup/*`: runtime integration examples and smoke coverage.

Macro surface assessment and unification plan: `docs/RhaiMacroSurface.md`.

## Failure model

- Panic-on-contract-violation is accepted by design for unsafe boundary misuse.
- No panic recovery layer is required for this scripting boundary right now.
