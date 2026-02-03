# Crates

Overview

This workspace uses multiple crates each with a focused responsibility. Below is a concise listing; see workspace `Cargo.toml` and each crate's README for details.

Key crates

- `core_engine` — The executable entrypoint and main runtime loop.
- `core_mod` — Core game logic and shared gameplay utilities.
- `core_mod_api` — API types and interfaces exposed to mods.
- `core_mod_macros` — Procedural macros used across core crates.
- `base_mod` — Example or base content mod that ships with the project.
- `base_mod_api` — API for base mods and utilities.
- `bevy_consumable_message` — Example app/lib using Bevy; tooling for message flows.

Workspace dependencies

- `serde`, `bevy`, `anyhow`, and similar common crates are used across crates (check `Cargo.toml` files).

Adding a new crate

- Create the crate: `cargo new my_crate --lib` or `cargo new my_mod --lib`.
- Add it to the workspace `Cargo.toml` under `members`.
- If publishing as a mod: make it a `cdylib` with suitable `crate-type` in `Cargo.toml` and export the expected hooks.

TODO:
- Add a full table with paths and license info.
- Document the exact semantic boundaries and API stability guarantees.

See also: `README.md` and `docs/Modding.md` for mod packaging details.