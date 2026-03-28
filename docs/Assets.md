# Assets — conventions & canonical locations

## Purpose

Short, concrete rules for where assets live and how we name, load, and test them.

## TL;DR

- Engine-owned: `core_mod/assets/...` (engine-related configs, scripts, shaders, models, textures, audio, etc.).
- Gameplay-owned: `base_mod/assets/...` (gameplay-related configs, scripts, etc.).
- Code-only crates (e.g., `core_mod_api`, `base_mod_api`) must not contain canonical assets.
- As of yet: **Proper modding capabilities are an unimplemented mess and should be fully ignored as a capability that this project may or may not have in the future. It's at best an afterthought right now, so some things may look like they support modding, but in reality there is no coherent framework for developing mods!**

## Canonical paths

- `core_mod/assets/configs/` — engine config files (e.g., `config.toml`). Use `core_mod_api::config`.
- `core_mod/assets/scripts/` — engine scripts (mainly just schedule hooks for now). These are typically very rarely changed as most scripting logic is gameplay logic; literally by design.
- `core_mod/assets/{shaders,models,textures,audio,etc.}/` — mandatory basic engine assets (think basic ui sprites, ui sounds, ui shaders, debug scripts, etc.).

- `base_mod/assets/scripts/` — gameplay scripts (.rhai) and data-driven logic.
- `base_mod/assets/configs/` — gameplay configs.
- `base_mod/assets/{shaders,models,textures,audio,etc.}/` — gameplay assets (think anything customizable outside of the game, like ui sprites, ui sounds, etc., but also *in-game* models, textures, shaders, audio, etc.). Avoid putting helper/util-like scripts here; these *may* belong in `core_mod/assets/scripts/`, but `base_mod/assets/scripts/utils` is often the right place.

## Example Locations (from this repo)

- `core_mod/assets/configs/config.toml`
- `core_mod/assets/scripts/core/boot.rhai`
- `core_mod/assets/scripts/ecs/schedule_hooks/*.rhai`
- `core_mod/assets/shaders/texture_generators/*.wgsl`

## Packaging & build

- Build scripts pack assets into `build/<profile>/` alongside crates.
- Each mod's `assets/` is isolated to avoid path conflicts in ready-to-use fully-merged builds.

## Guidelines

- Ownership: One authoritative owner per asset type: engine → `core_mod`, gameplay → `base_mod` (or other mod that owns the feature), but asset types are usable by everyone once they are owned by someone, exactly one someone to be precise.
- Loading: Prefer typed helpers in `core_mod_api` (no hard-coded manual asset loading all over the place; functions are a thing for a reason).
- Naming: All asset files & folders and rust source code files & folders must be in this format: `lower_snake_case`/`lower_snake_case.fileextension`.
- Document formats: Document new asset formats right here in `docs/Assets.md`.

## TODOs

- Versioning: Add a `version` field for asset formats and document migrations in `documents/`.
- Tests: Add small parsing/validation tests.
