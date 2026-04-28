# Architecture

Layers:

1. Engine layer: runtime, rendering, ECS, IO, loader.
2. First-party mod layer: `core_mod`, `base_mod`.
3. Third-party mod layer: optional mod crates.
4. Packaging layer: produce runnable game bundle.

Boundary rules:

- Engine internals are private.
- Mod API is public and versioned.
- First-party mods use the same mod API shape as third-party mods.
- Mods may expose APIs for other mods.
- Integration mods are first-class.

Change governance:

- Frozen contract set per published game version:
  - `mod_api`
  - mod manifest/package/load schemas
  - save-data schema

- Global break rules:
  - No breaking changes inside one published game version.
  - Breaking changes require a new contract version (usually tied to a new game version).
  - Every breaking change needs a migration note (or explicit "no migration path").
  - Engine-internal changes are unrestricted unless they leak into the frozen contract set.

- Engine internals
  - Owner: engine maintainers
  - Cadence: high
  - Break scope: internal only

- Platform contracts (`mod_api` + schemas)
  - Owner: platform maintainers
  - Cadence: low
  - Break scope: only at contract-version boundary

- First-party mods (`core_mod`, `base_mod`)
  - Owner: game/content maintainers
  - Cadence: medium
  - Break scope: same rules as third-party mods when consumed as dependencies

- Third-party mods
  - Owner: mod authors
  - Cadence: independent
  - Break scope: author-defined by semver, constrained by platform contracts
