# Scripting — Rhai Dialect and Schedule Entrypoint Tests

## Overview

- The engine embeds Rhai and boots it through `core_mod_api::rhai_binding::engine`.
- Startup script entrypoint:
  - authoring path: `core_mod/assets/scripts/core/boot.rhai`
  - runtime asset path: `core_mod/scripts/core/boot.rhai` (resolved by `asset_root()`).
- `boot.rhai` registers schedule entrypoints through `rhai_binding::schedule_entrypoints::add("<schedule_name>")`.
- USF substrate contracts are loaded from typed scripts under `core_mod/assets/scripts/usf/*`
  by the engine bootstrap loader (entrypoint-per-script-type).
- zone behavior (density + supported phenomena + selection policy + per-support `max_active` caps) is also authored in typed USF scripts,
  not via `boot.rhai`.
- Schedule entrypoint registration order is preserved (first `add(...)` call runs first for the same Bevy schedule phase).
- USF phase schedule entrypoints are wired into deterministic simulation subsets:
  - `substrate_pre_update` / `substrate_update` -> `UsfSubstrateSet::{Pre, Post}`
  - `zone_pre_update` / `zone_update` -> `UsfZoneSet::{Pre, Post}`
  - `phenomenon_pre_update` / `phenomenon_update` -> `UsfPhenomenonSet::{Pre, Post}`
- Schedule entrypoint contract supports:
  - `fn main(world, params)` only
- `params` is domain-typed instead of map-typed:
  - `SubstrateScheduleEntrypointParams` for `substrate_*` entrypoints
  - `ZoneScheduleEntrypointParams` for `zone_*` entrypoints
  - `PhenomenonScheduleEntrypointParams` for `phenomenon_*` entrypoints
  - `GlobalScheduleEntrypointParams` for non-domain entrypoints
- Typed params expose shared metadata (`entrypoint_name`, `entrypoint_file`, `domain`, `stage`,
  `delta_seconds`, `elapsed_seconds`, `has_virtual_time`,
  `bootstrap_global_script_count`, `bootstrap_package_script_count`, `bootstrap_selected_mod_count`,
  `bootstrap_executed_entrypoint_count`, `chunk_realization_mesh_instances`, `chunk_realization_material_instances`,
  `chunk_realization_collider_instances`, `chunk_realization_audio_emitters`, `chunk_realization_particle_emitters`,
  `chunk_realization_interaction_triggers`, `chunk_realization_simulation_services`) and domain-specific fields
  (for example `ZoneScheduleEntrypointParams.loaded_zone_count`).
- Testing gate is exposed via `rhai_binding::testing::enabled()`, backed by config key
  `rhai_binding/testing_enabled`.
- Bootstrap diagnostics are exposed via `rhai_binding::usf_bootstrap::*`
  (`selected_mod_ids`, `discovered_global_scripts`, `discovered_package_scripts`, `executed_entrypoint_count`).
- Startup test scripts only execute when `rhai_binding/testing_enabled = true`.
- Policy: keep script helper orchestration out of global namespace; prefer namespaced modules and `private fn`.

## Entrypoint loading model

- Schedule entrypoint runner lives in `core_mod_api/src/rhai_binding/engine/schedule_entrypoint.rs`.
- Each schedule entrypoint loads:
  1. all companion script files under a same-name companion directory recursively, ordered by file type then path:
     - `*.lib.rhai`
     - `*.entrypoint.rhai`
     - `*.substrate.rhai`
     - `*.zone.rhai`
     - `*.phenomenon.rhai`
     - any other `*.rhai`
  2. then the entrypoint root file itself.
- Example:
  - `startup.rhai` pulls in everything under `startup/` first, then calls `main(world, params)`.
  - `zone_update.rhai` can split responsibilities by file type:
    - `zone_math.lib.rhai` for reusable helpers,
    - `zone_pipeline.entrypoint.rhai` for stage orchestration,
    - `zone_runtime.zone.rhai` for zone-domain runtime logic.

This makes `startup.rhai` a stable startup-test entrypoint while companion files hold organized tests.

## Script layout

- Core scripts root: `core_mod/assets/scripts/core/`
- ECS schedule entrypoints: `core_mod/assets/scripts/ecs/schedule_entrypoints/`
  - includes dedicated `substrate_*`, `zone_*`, and `phenomenon_*` phase files for USF simulation staging.
- Startup test harness: `core_mod/assets/scripts/ecs/schedule_entrypoints/startup/`
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
- use declarations are preprocessed per script file before entrypoint-source composition,
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
