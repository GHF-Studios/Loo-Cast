# Scripting — Rhai Dialect and Hook Tests

## Overview

- The engine embeds Rhai and boots it through `core_mod_api::rhai_binding::engine`.
- Startup script entrypoint:
  - authoring path: `core_mod/assets/scripts/core/boot.rhai`
  - runtime asset path: `core_mod/scripts/core/boot.rhai` (resolved by `asset_root()`).
- `boot.rhai` registers schedule hooks through `rhai_binding::schedule_hooks::add("<schedule_name>")`.
- Testing gate is exposed via `rhai_binding::testing::enabled()`, backed by config key
  `rhai_binding/testing_enabled`.
- Startup test scripts only execute when `rhai_binding/testing_enabled = true`.
- Policy: keep script helper orchestration out of global namespace; prefer namespaced modules and `private fn`.

## Hook loading model

- Hook runner lives in `core_mod_api/src/rhai_binding/engine/hook.rs`.
- Each hook stage loads:
  1. all `.rhai` files under a same-name companion directory recursively (sorted by full path),
  2. then the stage's root file itself.
- Example:
  - `startup.rhai` pulls in everything under `startup/` first, then calls `main(world)`.

This makes `startup.rhai` a stable startup-test entrypoint while companion files hold organized tests.

## Script layout

- Core scripts root: `core_mod/assets/scripts/core/`
- Schedule hooks: `core_mod/assets/scripts/core/schedule_hooks/`
- Startup test harness: `core_mod/assets/scripts/core/schedule_hooks/startup/`
  - `tests/reflection/` for reflection graph smoke checks.
  - `tests/ecs/` for ECS integration tests (World, Commands, Query, Messages, iterators).
  - `tests/examples/` for runnable example-tests, currently including `shop::divisions::sex`.
- Non-core module scripts: `*/assets/scripts/<module_name>/...` (for example
  `core_mod/assets/scripts/other_module/...`).

## `use` alias syntax

Supported script alias form:

- `use bevy::ecs::query::QueryData as QueryData;`
- `use core_mod_api::player::bundles::PlayerBundle as PlayerBundle;`

Current behavior:

- aliases are preprocessed before Rhai compilation,
- use declarations are preprocessed per script file before hook-source composition,
- alias substitution applies to:
  - path roots (`Alias::...`),
  - bare alias tokens (`Alias`) as canonical type-id string literals,
- strings/comments are not rewritten.
- alias declarations fail fast on:
  - duplicate alias names in one script,
  - reserved Rhai keywords,
  - collisions with known registered global symbol names when target path differs.

## Bridge model (high-level)

- Reflection/registration source of truth: `core_mod_api/src/rhai_binding/bridges/`
  - `domains/` for real runtime APIs (`ecs`, `player`, `rust`, etc.).
  - `testing/` for test-only modules.
- Runtime wrapper types live directly under:
  - `core_mod_api/src/rhai_binding/runtime/*`
- Bridge/runtime code should import through `rhai_binding::runtime::*`.

## Extension references

- Architecture and design rules: `docs/RhaiDialect.md`
- Step-by-step extension workflow: `docs/RhaiBridgeDevelopment.md`
- Coverage/TODO hierarchy: `docs/RhaiBindingRoadmap.md`
- Generic binding contract: `docs/RhaiGenericBindingPolicy.md`
- Script ergonomics design proposal: `docs/RhaiScriptErgonomics.md`

## Validation

- Fast compile check: `cargo check -p core_mod_api`
- End-to-end startup script smoke:
  1. `./build.sh dev`
  2. `./run.sh dev`
