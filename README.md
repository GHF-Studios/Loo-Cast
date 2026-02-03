# Loo Cast — Project Overview 🚀

**Loo Cast** is a work-in-progress repository that contains both an engine and a game mod built in Rust.

- **Engine**: a runtime for scale-aware simulation (ECS + modular runtime logic).
- **Gameplay (base mod)**: a mod built on top of the engine that provides scripts, configs, and content.

> Status: Active development. Docs and architecture are evolving — see `docs/` for curated references and `documents/` for long-form design notes.

---

## Quick links

- 📚 Docs: `docs/` (TOC in `docs/README.md`)
- 🧩 Crates: `docs/Crates.md` (differentiates code vs canonical assets)
- ⚙️ Build & Run: `./build.ps1` / `./build.sh` and `./run.ps1` / `./run.sh`
- 📝 Design notes: `documents/`

---

## Quick start 🛠️

Windows (PowerShell):

```powershell
./build.ps1 [dev|fastdev|release]
./run.ps1 [profile]
```

Linux / macOS:

```bash
./build.sh [dev|fastdev|release]
./run.sh [profile]
```

Build artifacts (mods & assets) are placed under `build/<profile>/`.

---

## High-level structure 🔧

- Repository is a Cargo workspace. Key responsibilities are intentionally split:
  - **`core_mod_api`** — code-only crate: typed APIs, plugin groups, workflows.
  - **`core_mod`** — canonical assets & initialization: configs, scripts-as-assets, models, shaders, and default data.
  - **`base_mod` / `base_mod_api`** — gameplay assets and scripting wrappers (rhai bindings).
  - **`core_mod_macros` / `base_mod_macros`** — procedural macros used internally.
- Engine loads built mods at runtime; see `core_engine` for the runtime composition.

---

## Canonical asset locations 📁

- `core_mod/assets/configs/` — engine-related configuration files.
- `core_mod/assets/scripts/` — engine-related scripts[^engine_vs_gameplay_assets_note].
- `core_mod/assets/*MY_ASSET_TYPE*/` - other engine-related assets of any kind, e.g.: shaders, models, textures, sound files, etc.
- `base_mod/assets/*MY_ASSET_TYPE*/` — gameplay-related configs, scripts, shaders, models, textures, sound files, etc..

For conventions and guidelines, see `docs/Assets.md`.

---

## Guidelines for docs 📖

- Prefer short, navigable docs in `docs/` and keep long-form history and design notes in `documents/`.

---

## Crate summary (short) 🧬

| Crate                      | Purpose & focus                             |
|----------------------------|----------------------------------------------|
| `core_engine`              | Runtime binary: composes Bevy plugins & runs app loop. |
| `core_mod`                 | Canonical assets & built-in data; provides init hook for statics. |
| `core_mod_api`             | Code-only API surface: plugins, types, workflows. |
| `core_mod_macros`          | Procedural macros used by core crates. |
| `base_mod`                 | Gameplay mod bundle (scripts, configs, assets). |
| `base_mod_api`             | Scripting bindings (rhai wrappers) for gameplay. |
| `base_mod_macros`          | Helpers for generating scripting wrappers. |
| `bevy_consumable_message`  | Small reusable Bevy message utility. |

See `docs/Crates.md` for more details.

---

[^engine_vs_gameplay_assets_note]: Note: Almost all scripts should likely be gameplay-related and thus belong in `base_mod/assets/scripts/` instead. 
