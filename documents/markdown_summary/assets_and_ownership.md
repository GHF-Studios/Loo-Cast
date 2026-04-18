# Assets and Ownership

Current path and ownership conventions.

## Ownership Baseline

- Engine-owned assets: `core_mod/assets/...`
- Gameplay-owned assets: `base_mod/assets/...`
- Code-only crates should not contain canonical assets.

## Canonical Path Families

- `*/assets/configs/`
- `*/assets/scripts/`
- `*/assets/{shaders,models,textures,audio,...}/`

## Packaging

Build scripts copy per-mod asset trees into `build/<profile>/assets/<mod>/`.

## Naming

Use `lower_snake_case` for folders and files.

## Status

Modding/discovery semantics are still incomplete; use this as implementation guidance only.
