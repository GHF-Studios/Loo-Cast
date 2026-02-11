# core_mod_api — Internal API surface

`core_mod_api` contains the typed API surface, plugin groups, workflows, and utility types used by the engine and first-party mods.

Key points

- **Code-only crate**: this crate should not contain any asset files. Engine-Level assets belong to `core_mod` (configs, rhai-scripts, models, shaders, etc.).
- **Intended audience**: engine and mod developers working inside this repository — internal-first.
- **Stability**: public items are *internal-first*. External stability is currently not a primary concern, and thus a heavy WIP!

See the crate doc in `src/lib.rs` for the canonical crate-level notes and `docs/Crates.md` for workspace-level context.