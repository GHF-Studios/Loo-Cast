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

- All compatibility/version/migration policy lives in `CONTRACTS.md`.
- Architecture may change freely unless it violates `CONTRACTS.md`.

Ownership + cadence:

- Engine internals
  - Owner: engine maintainers
  - Cadence: high

- Platform contracts
  - Owner: platform maintainers
  - Cadence: low

- First-party mods (`core_mod`, `base_mod`)
  - Owner: game/content maintainers
  - Cadence: medium

- Third-party mods
  - Owner: mod authors
  - Cadence: independent
