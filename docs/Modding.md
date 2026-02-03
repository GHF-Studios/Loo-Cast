# Modding

Structure

- Mods are implemented as Rust crates and are typically built as `cdylib` dynamic libraries so the engine can load them at runtime.
- Typical layout:
  - `Cargo.toml` (set `crate-type = ["cdylib"]` for runtime mods)
  - `src/lib.rs` (export `init_api` and other expected hooks)
  - `assets/` for any art, scripts, or configs shipped with the mod

Features & API

- The engine expects exported hooks such as `init_api` that register systems, assets, or event listeners.
- Use `core_mod_api` / `base_mod_api` types for safe interop with the engine/game respectively.

Loading & discovery

- Engine loads mods from configured mod directories at startup and dynamically for whatever mods are present. (I think?)

Testing & tips

- Test a mod by running `cargo build -p <mod-crate>` and dropping the library into `build/dev/mods/` or using the fastdev workflow.
- Keep mod APIs small and stable; use feature flags for optional capabilities.

TODO:
- Metadata files (TODO: define filename and fields) help the engine determine load order and dependencies.
- Document the exact hook signatures and metadata schema.
- Add example mod template and a step-by-step guide (link to `base_mod/`).