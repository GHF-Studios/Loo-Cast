# Scripting — Rhai-based scripting & schedule hooks 🛠️

**Overview**
- This project uses Rhai as the embedded scripting language for gameplay logic and light orchestration.
- The main script engine is created at startup (see `core_mod_api::reflection::internals::functions::new_main_script_engine`) and runs a `boot.rhai` script located under `core_mod/assets/scripts/core/boot.rhai`.

**Schedule hooks**
- Scripts can register hook handlers which are then mapped into Bevy schedule stages (e.g., `pre_startup`, `startup`, `update`, `post_update`, `last`).
- The engine's hook runner compiles and executes hook files from `core_mod/scripts/core/schedule_hooks/` and runs them in the appropriate stage via a system wrapper.
- Use the engine-exposed helper `add_hook_handler("<schedule_name>")` from scripts to register a hook that gets executed by the runner.

**Bindings exposed to scripts**
- `World` — read/write access to the Bevy world (safe, scoped handle wrapping raw Bevy access).
- `Commands` / `EntityCommands` — spawn and manipulate entities.
- `Component` / `Bundle` — convenience constructors for components and bundles.
- `Entity` — entity identity and helper getters.
- TODO: This is hiiiiiighly incomplete and needs to be expanded into a full tree of docs for the base_mod_api crate, basically.

**Authoring tips**
- Keep boot scripts idempotent and fast; use scheduled hooks for per-frame or periodic logic.
- Prefer manipulating data through `Commands` and small scripts that call into typed runtime APIs instead of re-creating complex logic in scripts.

**Where to look**
- `core_mod_api/src/reflection/internals/functions.rs` — script engine, bindings, and hook runner.
- `core_mod/assets/scripts/core/` — boot scripts and default hooks.
- `base_mod/assets/scripts/` — gameplay scripts for the base mod.
