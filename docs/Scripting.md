# Scripting — Rhai Dialect and Typed Entrypoint Contracts

## Overview

- The engine embeds Rhai and boots it through `core_mod_api::rhai_binding::engine`.
- Startup script entrypoint:
  - authoring path: `core_mod/assets/scripts/core/boot.rhai`
  - runtime asset path: `core_mod/scripts/core/boot.rhai` (resolved by `asset_root()`).
- `boot.rhai` registers schedule entrypoints through `rhai_binding::schedule_entrypoints::add("<schedule_name>")`.
- USF contracts are loaded from typed scripts under `core_mod/assets/scripts/usf/*` by the engine bootstrap loader.
- Schedule entrypoint registration order is preserved (first `add(...)` call runs first for the same Bevy schedule phase).
- USF phase schedule entrypoints are wired into deterministic simulation subsets:
  - `substrate_pre_update` / `substrate_update` -> `UsfSubstrateSet::{Pre, Post}`
  - `realizer_pre_update` / `realizer_update` -> `UsfRealizerSet::{Pre, Post}`
  - `phenomenon_pre_update` / `phenomenon_update` -> `UsfPhenomenonSet::{Pre, Post}`
- Typed params expose shared metadata (`entrypoint_name`, `entrypoint_file`, `domain`, `stage`, timing metadata, bootstrap diagnostics, chunk realization metrics) plus domain-specific fields.
- Schedule entrypoint contracts are typed per domain and operate through typed ctx graphs with capability-channel endpoints.
- Capability channels are part of ctx graphs; script write paths emit intents only.
- Optional evaluator hooks may participate in declared reconciliation stages and are panic-fast on failure.

## Entrypoint Loading Model

- Schedule entrypoint runner lives in `core_mod_api/src/rhai_binding/engine/schedule_entrypoint.rs`.
- Each schedule entrypoint loads:
  1. all companion script files under a same-name companion directory recursively, ordered by file type then path:
     - `*.lib.rhai`
     - `*.entrypoint.rhai`
     - `*.substrate.rhai`
     - `*.realizer.rhai`
     - `*.phenomenon.rhai`
     - any other `*.rhai`
  2. then the entrypoint root file itself.
- Example:
  - `startup.rhai` pulls in everything under `startup/` first, then calls `main(world, params)`.
  - `realizer_update.rhai` can split responsibilities by file type:
    - `realizer_math.lib.rhai` for reusable helpers,
    - `realizer_pipeline.entrypoint.rhai` for stage orchestration,
    - `realizer_runtime.realizer.rhai` for realization-domain runtime logic.

This keeps root entrypoints stable while companion files hold organized domain logic/tests.

## Script Layout

- Core scripts root: `core_mod/assets/scripts/core/`
- ECS schedule entrypoints: `core_mod/assets/scripts/ecs/schedule_entrypoints/`
  - includes dedicated `substrate_*`, `realizer_*`, and `phenomenon_*` phase files for USF simulation staging.
- Startup test harness: `core_mod/assets/scripts/ecs/schedule_entrypoints/startup/`
  - `tests/reflection/` for reflection graph smoke checks.
  - `tests/ecs/` for ECS integration tests (World, Commands, Query, Messages, iterators).
  - `tests/examples/` for runnable example tests.
- Non-core module scripts: `*/assets/scripts/<module_name>/...` (for example `core_mod/assets/scripts/other_module/...`).

## `use` Alias Syntax

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

## Bridge Model (High-Level)

- Reflection/registration source of truth: `core_mod_api/src/rhai_binding/bridges/`
  - `domains/` for runtime APIs (`ecs`, `player`, `rust`, etc.).
  - `testing/` for test-only modules.
- Runtime wrapper types live directly under:
  - `core_mod_api/src/rhai_binding/runtime/*`
- Bridge/runtime code should import through `rhai_binding::runtime::*`.

## Extension References

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

## Status Note

The owner-direction scripting contract is the current target model defined in the scripting diagram atlas and canonical docs.  
Code may still contain legacy pre-target names while migration is in progress.
