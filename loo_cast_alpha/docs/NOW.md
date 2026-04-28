# NOW

Date: 2026-04-28

Current direction:

- Keep one repo.
- Keep xtask small and focused.
- Treat `core_mod` and `base_mod` as first-party mods, not engine internals.
- Build a composable mod ecosystem (mods can depend on mods).
- Use lockstep game/contract versioning.

Current scope:

- Primary tasks: `build`, `package`, `run`, `cloc`, `gource`.
- No SDK/toolchain redistribution layer right now.
- No dual-repo automation right now.
- Compatibility policy is defined in `CONTRACTS.md`.

Success for this phase:

- Clean developer workflow with low cognitive load.
- Clear boundary between engine internals and public mod API.
- Mod composition proven with at least one integration-mod example.
