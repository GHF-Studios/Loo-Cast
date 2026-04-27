# Assets and Ownership

Current path and ownership conventions.

## Ownership Baseline

- Engine-owned assets: `core_mod/assets/...`
- Gameplay-owned assets: `base_mod/assets/...`
- Code-only crates should not contain canonical assets.

## Canonical Path Families

- `*/assets/configs/`
- `*/assets/{modpack,mod,scale,metric,phenomenon_realizer,phenomenon,texture,sfx,music,shader,config,usf_texture,usf_sfx,usf_model,...}/`
- `*/assets/{shaders,models,textures,audio,...}/`

## Packaging

Build scripts copy per-mod asset trees into `build/<profile>/assets/<mod>/`.

## Naming

Use `lower_snake_case` for folders and files.

## Status

Modding/discovery semantics are still incomplete; use this as implementation guidance only.

## Current MVP Script Slice

- `core_mod/assets/scale/35.rhai`
- `core_mod/assets/metric/test_metric.rhai`
- `core_mod/assets/phenomenon/test_phenomenon.rhai`
- `core_mod/assets/phenomenon_realizer/35.rhai`
