# Scripting — Rhai Dialect, Hooks, and Bridge Suites

## Overview

- The engine embeds Rhai and boots it through `core_mod_api::rhai_binding::engine`.
- Startup script entrypoint:
  - authoring path: `core_mod/assets/scripts/core/boot.rhai`
  - runtime asset path: `core_mod/scripts/core/boot.rhai` (resolved by `asset_root()`).
- `boot.rhai` registers schedule hooks through `add_hook_handler("<schedule_name>")`.

## Hook loading model

- Hook runner lives in `core_mod_api/src/rhai_binding/engine/hook.rs`.
- Each hook stage loads:
  1. all `.rhai` files under a same-name companion directory recursively (sorted by full path),
  2. then the stage's root file itself.
- Example:
  - `startup.rhai` pulls in everything under `startup/` first, then calls `main(world)`.

This makes `startup.rhai` a stable test/example entrypoint while companion files hold organized suites.

## Script layout

- Core scripts root: `core_mod/assets/scripts/core/`
- Schedule hooks: `core_mod/assets/scripts/core/schedule_hooks/`
- Startup suites: `core_mod/assets/scripts/core/schedule_hooks/startup/`
  - `reflection/` for reflection graph smoke checks.
  - `ecs/` for ECS bridge examples (World, Commands, Query, Messages, iterators).
  - `testing/` for testing-only bridge modules (e.g. `shop::divisions::sex`).
- Rhai-only utility namespace scaffold: `core_mod/assets/scripts/core/rhai_std/`

## Bridge model (high-level)

- Reflection/registration source of truth: `core_mod_api/src/rhai_binding/bridges/`
  - `domains/` for real runtime APIs (`ecs`, `player`, `rust`, etc.).
  - `testing/` for test-only modules.
- Runtime wrapper types currently live in `script/*` and are re-exported via:
  - `core_mod_api/src/rhai_binding/runtime/mod.rs`
- New bridge/runtime code should import through `rhai_binding::runtime::*`, not directly from `script::*`.

## Extension references

- Architecture and design rules: `docs/RhaiDialect.md`
- Step-by-step extension workflow: `docs/RhaiBridgeDevelopment.md`

## Validation

- Fast compile check: `cargo check -p core_mod_api`
- End-to-end startup script smoke:
  1. `./build.sh dev`
  2. `./run.sh dev`
