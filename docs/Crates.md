# Crates

This workspace is organized into a set of focused crates. The table below explains each crate's *responsibility* and clarifies whether it is primarily **code** or **canonical assets**.

| Crate | Primary focus | Notes |
|---|---:|---|
| `core_engine` | Code (binary) | Engine entrypoint — composes Bevy plugins, registers `CoreApiPluginGroup`, initializes `core_mod` and runs the app loop. See `core_engine/README.md`.
| `core_mod` | Code (Entrypoint/Init) + Assets (Engine) | Engine-related data & assets (primarily configs, shaders, and occasionally rhai-scripts). Exposes an init hook used to register statics. See `core_mod/README.md`.
| `core_mod_api` | Code (EngineAPI) | Internal API surface: types, plugin groups, workflows, ecs systems, etc. Code-only; No assets of any kind. See `core_mod_api/README.md`.
| `core_mod_macros` | Code (macros) | Procedural macros: workflows, global statics helpers, USF utilities, etc..
| `base_mod` | Code (Entrypoint/Init) + Assets (Gameplay) | Gameplay content: scripts, configs, models, etc.; and initialization glue (cdylib). See `base_mod/README.md`.
| `base_mod_api` | Code (Rhai-Binds) | Rhai-Scripting wrappers/bindings for gameplay scripts to utilize; No assets of any kind. See `base_mod_api/README.md`.
| `base_mod_macros` | Code (macros) | Macros to reduce boilerplate for script-bindings on the rust-side.
| `bevy_consumable_message` | Code (utils) | Small Bevy utility crate used across the workspace.


See also: `README.md`, `docs/Assets.md` and `docs/Modding.md`.