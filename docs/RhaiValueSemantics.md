# Rhai Value Semantics

This document is the intent reference for the semantics variants declared in
`core_mod_api/src/rhai_binding/value_semantics/modes.rs`.

## Canonical variants

- `Clone`
- `Owned`
- `Ref`
- `Mut`
- `ScopedOwned`
- `ScopedRef`
- `ScopedMut`

## Core intent

These variants describe runtime access capability, not permanent storage class.
In particular, `Owned`/`Ref`/`Mut` are not "persistent container types" by
themselves. They are access semantics exposed to Rhai through the
`AccessCell`/`AccessCellProvider` model.

`Scoped*` means the same capability with lifetime erasure at the boundary.
Historically this was named "scoped" because the Rust lifetime is transmuted
away, then re-anchored by the provider lifecycle.

## Capability model

1. `Clone`
- Rhai-native trivial value transfer.
- No AccessCell lifecycle is required beyond normal Rhai value handling.

2. `Owned`
- Unique ownership-style capability in script-facing code.
- Usually represented with `AccessCell<..., T>` wrappers.

3. `Ref`
- Readonly access capability.
- Shared readonly handle behavior is acceptable for this mode.

4. `Mut`
- Mutable access capability.
- Treated as mutation authority, not generic shared readonly semantics.

5. `ScopedOwned`
- `Owned` with lifetime-erased boundary handling.
- Valid only inside the provider-managed access window.

6. `ScopedRef`
- `Ref` with lifetime-erased boundary handling.
- Valid only inside the provider-managed access window.

7. `ScopedMut`
- `Mut` with lifetime-erased boundary handling.
- Valid only inside the provider-managed access window.

## Handle liveness model

- Rhai clones aggressively and may retain handles longer than Rust lifetimes.
- Dangling script handles are allowed to exist.
- Safety is enforced at use-time: any access attempt on an invalidated/moved-out
  `AccessCell` must fail fast (`panic!`).
- This mirrors raw-pointer intuition: obtaining a handle is not the unsafe act;
  dereferencing/using it after invalidation is rejected.

## Soundness boundary

- `AccessCell` + `AccessCellProvider` are the intended backbone for non-trivial
  semantics and borrow-window enforcement.
- Access must follow `start_access -> use -> end_access` in the same
  synchronous Bevy system execution frame.
- Contract violations are fail-fast (`panic!`) by design.
- Entrypoints are expected to be Bevy schedule-hook systems; asynchronous or
  out-of-band access paths are outside this safety model.

## Why this file exists

Several placeholder files for semantics variants were removed during cleanup.
This file is the replacement intent record for those variants, while the actual
runtime implementation remains centralized in active modules.
