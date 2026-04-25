# Rhai Value Semantics (Summary)

Semantics vocabulary:

- `Clone`
- `Owned`
- `Ref`
- `Mut`
- `ScopedOwned`
- `ScopedRef`
- `ScopedMut`

## Intent

- These are runtime access semantics, not persistent container categories.
- `AccessCell` + `AccessCellProvider` are the core soundness boundary.
- `Scoped*` variants rely on provider-managed frame-bound access windows.

## Failure Model

- Stale/invalid access attempts fail fast.
- Contract violations in access lifecycle are panic-fast by design.
