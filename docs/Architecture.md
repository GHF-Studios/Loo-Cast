# Architecture

Short summary

The engine is a modular Rust-based runtime that composes a core engine (`core_engine`) and a set of crates (mods, APIs, macros) inside this workspace. The runtime separates core systems (rendering, simulation, input, resources) from game content implemented as crates and dynamically loaded mod libraries.

Workspace & build

- Workspace contains multiple crates (core, core_mod, base_mod, APIs, macros, tools) managed by Cargo and custom build scripts (`build.ps1` / `build.sh`).
- Build outputs are placed under `target/` (Cargo) and the `build/` directory (packaging/dev artifacts).

Runtime & dynamic loading

- Mods are compiled as dynamic libraries (cdylib) and loaded by the engine at runtime.
- The engine expects specific exported hooks (e.g. `init_api`) and metadata for discovery.
- Assets are conventionally placed in a mod's `assets/` folder and addressed by the engine asset system.

Features & notes

- Hot-reload / fastdev workflows supported via `fastdev` profile and dev artifacts.
- Cross-platform support: Windows and Linux build scripts are provided.

See also

- Concepts: [Concepts](./Concepts.md)
- README and `documents/` for deeper design notes and lore.

TODO:
- Describe the plugin discovery order and metadata format.
- Add an architecture diagram (refer to `documents/` if available).