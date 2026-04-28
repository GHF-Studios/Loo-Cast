# Workflows

Daily developer loop:

1. `cargo xtask build`
2. `cargo xtask package`
3. `cargo xtask run`

Support tools:

- `cargo xtask cloc`
- `cargo xtask gource`

Mod author loop (current):

1. Add a crate under `crates/`.
2. Mark mod crate with `.loo_cast_mod`.
3. Implement mod code against public mod API.
4. Build/package/run through xtask.

Composition loop:

1. Add two mods with separate APIs.
2. Add one integration mod that depends on both.
3. Verify build order and runtime compatibility.

Contract-safe change flow:

1. Change engine/mod code without breaking `CONTRACTS.md`.
2. Run `cargo xtask build`, `cargo xtask package`, `cargo xtask run`.
3. Update docs only if behavior/workflow changed.

Breaking change flow:

1. Bump target published version per `CONTRACTS.md`.
2. Update affected contract definitions.
3. Add migration guide at `docs/migrations/<from>-to-<to>.md`.
4. Record decision in `DECISIONS.md`.
5. Validate build/package/run before release.
