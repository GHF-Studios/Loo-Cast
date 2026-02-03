# Loo Cast 🚀

**Loo Cast** is a work-in-progress project that is two things at once:
- **Engine** — a runtime intended to simulate a scale-aware, contextual world (ECS + modular runtime logic).
- **Game / Mod** — a gameplay layer built as a mod on top of the engine that explores narrative and simulation possibilities.

> Status: WIP — docs and systems are actively evolving. See the `docs/` folder for detailed reference.

---

## Quick links

- 📚 Docs: `docs/` (Architecture, Concepts, Building, Modding, Crates, Contributing)
- 🧩 Crates: see `docs/Crates.md` for a quick reference of workspace crates
- ⚙️ Build & Run: `./build.ps1` / `./build.sh` and `./run.ps1` / `./run.sh`
- 📝 Notes & design docs: `documents/`

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

## High-level overview 🔧

- The repository is a Cargo workspace. Key crates include the engine (`core_engine`), a core mod (`core_mod` + `core_mod_api`) and a gameplay mod (`base_mod` + `base_mod_api`).
- Mods are built as dynamic libraries (see `base_mod` crate) and loaded by the engine at runtime.
- Most architectural notes and design discussions live in `documents/` — the `docs/` pages are shorter, curated references.

---

## Crate summary (short) 🧬

| Crate                      | Purpose                                      |
|----------------------------|----------------------------------------------|
| `core_engine`              | Main engine binary (entry point)             |
| `core_mod`                 | Built-in core mod; bundles APIs & assets     |
| `core_mod_api`             | API surface for engine + mods                |
| `core_mod_macros`          | Macros used by core crates                   |
| `base_mod`                 | Gameplay mod (cdylib)                        |
| `base_mod_api`             | API for gameplay features & mods             |
| `base_mod_macros`          | Macros for base gameplay                     |
| `bevy_consumable_message`  | Custom bevy plugin used by the project       |

See `docs/Crates.md` for more details and pointers.
