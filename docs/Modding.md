# Modding

## WIP Disclaimer
Modding support in the sense that the engine can load and run mods external to this workspace and outside of ``core_mod`` and ``base_mod`` is a work in progress. The engine currently supports loading the core and base mods, and can be extended to load additional mods with some additional work, but it's not there yet!

## Structure

- Mods are implemented as Rust crates and are typically built as `cdylib` dynamic libraries so the engine can load them at runtime.
- Typical layout:
  - `Cargo.toml` (set `crate-type = ["cdylib"]` for runtime mods)
  - `src/lib.rs` (export `init_api` and other expected hooks)
  - `assets/` for any art, scripts, or configs shipped with the mod

## Features & API

- The engine expects exported hooks such as `init_api` that register systems, assets, or event listeners.
- Use `core_mod_api` / `base_mod_api` types for safe interop with the engine/game respectively.

## Loading & discovery

- Engine loads the core mod statically, and dynamically and automatically the base mod, *and whatever other mods are present, I think? WIP!*

## TODO:
- Metadata files (TODO: define filename and fields) help the engine determine load order and dependencies.
- Document the exact hook signatures and metadata schema.
- Add example mod template and a step-by-step guide (link to `base_mod/`).