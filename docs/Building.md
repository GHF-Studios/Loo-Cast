# Building & Running 🔧

## Quick commands

- Windows (PowerShell):
  - Build (default fastdev profile): `.\build.ps1` (defaults to `fastdev`).
  - Build a specific profile: `.\build.ps1 dev|fastdev|release`.
  - Run: `.\run.ps1 [dev|fastdev|release]` (defaults to `fastdev`).

- Linux / macOS (bash):
  - Build: `./build.sh [dev|fastdev|release]` (defaults to `fastdev`).
  - Run: `./run.sh [dev|fastdev|release]` (defaults to `fastdev`).

> Note: The build scripts invoke `cargo +nightly build` and build mod crates separately with the `init_api` feature. They then copy the engine executable, mod dynamic libraries, and `assets/` into `build/<profile>/`.

---

## Profiles & artifact layout ✅

- `dev` — Cargo's debug build. Used for standard local debugging.
- `fastdev` — A custom fast iteration profile.
- `release` — Optimized release build.

The build scripts do the following steps:
1. Build the workspace excluding mod crates (so the engine and libraries are built together).
2. Build mod crates (`base_mod`, etc.) separately using `--features init_api` so they export required init symbols.
3. Copy the engine executable (and debug symbols when available) into `build/<profile>/`.
4. Copy mod dynamic libraries (`.dll`, `.so`) into `build/<profile>/`.
5. Copy assets from each mod's `assets/` into `build/<profile>/assets/<mod>/`.

Additional notes:
- If a build is started, no matter the outcome, the selected profile's output directory is immediately nuked into oblivion to ensure no remaining artifacts from the last build clobber the new build.

---

## Running the game

- `run.ps1` / `run.sh` launch `build/<profile>/core_engine` and set up library search paths for dev builds (adds `target/debug/deps` and Rust's target lib dir to loader paths) so the executable can find dynamically-built mod libraries when running locally.