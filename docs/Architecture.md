# Architecture — Engine & Mod Composition 🏗️

## High-level summary

The runtime is a modular Rust engine that composes a core executable (`core_engine`) and a set of workspace crates that provide APIs, canonical assets, and mods. The engine separates core systems (rendering, input, resources, simulation) from game content; mods are delivered as dynamic libraries (`cdylib`) with clearly defined init hooks.

---

## Workspace & build

- The repo is a Cargo workspace.
- Builds are orchestrated by `build.ps1` / `build.sh` which:
- Built artifacts are placed under `target/` (Cargo) and the packaged final product ends up in `build/<profile>/`.
  1. *Clean* the output directory for the selected profile,
  2. *Build* the workspace executable and libraries (using `cargo +nightly`),
  3. *Build* mod crates separately with `--features init_api`,
  4. *Copy* the engine executable, mod libraries, and `assets/` into `build/<profile>/`.
- See `docs/Building.md` for more details.

---

## Runtime & dynamic loading

- Mods are compiled as `cdylib` and expose a well-known init entrypoint (symbol names use a fixed convention such as `__init_api__<crate_name>`).
- `core_engine` initializes *static* crates directly (e.g., `core_mod::__init_api__core_mod_api()`), and may dynamically load other mods by loading their dynamic libraries and invoking their exported init functions.
- Asset ownership is convention-based: `core_mod/assets/` for engine-owned assets and `base_mod/assets/` (or other mod `assets/`) for gameplay assets.

---

## Plugin & system composition

- The engine composes Bevy `Plugins`, and builds and registers a `CoreApiPluginGroup`, which in turn registers bevy systems, bevy resources, bevy schedules, etc. defined in `core_mod_api`. Bevy's ECS is used for game state management, and as the primary entrypoint for everything, although some init/update functionality may run outside the ECS context as needed.
- Mods expose initialization hooks and register bevy plugin groups, bevy systems, etc. to facilitate the "rust-side entrypoint". 
- The core mod specifically also exposes schedule hooks which hook into the bevy-powered engine's ecs lifecycle via rhai-scripting.

---

## Hot-reload & fastdev & final notes & whatever

- `fastdev` is a workflow geared for fast iterative development. The build script packages the engine, mods, and assets into `build/fastdev/` for convenient local runs.
- `run.ps1` / `run.sh` add dynamic library search paths for dev runs so locally-built mod libraries are discovered without installing into system locations.
- Do yourself a favour and have a look at `/docs/Building.md` for more details on building and running loo cast.

---

## Where to look in the code

- `core_engine/src/main.rs` — app composition, Bevy plugin configuration, and global init.
- `core_mod` — canonical engine assets and script hooks (`core_mod/assets/scripts/`).
- `core_mod_api` — typed APIs, plugin groups, and schedule hook registration.
- `base_mod` — gameplay assets and script bundles.
- `base_mod_api` — scripting bindings (rhai wrappers) for gameplay scripts.

---

> Note: Plugin discovery order and metadata format are intentionally simple for now (explicit init hooks and asset conventions); adding a discovery metadata manifest and a plugin registry is a reasonable future enhancement.