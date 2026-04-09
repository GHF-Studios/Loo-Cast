# Style & Patterns

Purpose: durable implementation and authoring patterns for this repository.

## Architecture Boundaries

1. USF-facing ontology and content are script-configured.
2. Rust owns execution kernels, capability backends, and runtime orchestration.
3. Bevy ECS is internal infrastructure, not the public USF language.
4. Canonical runtime persistence authority is entity-grounded:
   - `Phenomenon`
   - `PhenomenonModel`
   - `PartialPhenomenonModel`
5. Zones are derived but may be entity-backed runtime abstractions with agency.
6. Substrate and chunk realization artifacts are derived/runtime state.

## Rhai Authoring Patterns

1. Use typed script entrypoints only (`register_metric`, `register_zone`, `register_scale`, etc.).
2. Keep one-entity-per-file for entity-bearing USF script types.
3. Keep `boot.rhai` focused on schedule entrypoint registration.
4. Use script ctx APIs to declare content contracts; do not script around engine internals.
5. Keep startup test harness content under startup test script folders, separate from gameplay content.

## Runtime Behavior Patterns

1. Prefer deterministic behavior and fail-fast validation over silent fallback.
2. Reject invalid/missing script contracts at bootstrap.
3. Keep ownership boundaries explicit (definition vs runtime state vs output application).
4. Keep zone behavior parameterized over derived world data/policy, not ad-hoc canonical zone truth.
5. Keep capability application routed through registered execution channels, not hardcoded per-content branches.
6. Keep zone assignment exclusive per location/scale; map unclassified locations to a sentinel zone.
7. Keep phenomenon-to-zone feedback indirect through substrate/metric updates, never through direct canonical zone mutation.
8. Treat direct reconciliation versus intent queues as implementation strategy only; do not change lifecycle authority semantics.

## Workflow Framework Patterns

1. Treat workflows as explicit staged contracts (`Ecs`, `Render`, `Async`, `EcsWhile`, `RenderWhile`).
2. Keep stage names/domain names explicit and grep-friendly.
3. Preserve observable timeout and failure behavior.
4. Avoid ad-hoc side channels that bypass workflow/runtime registries.

## Naming Patterns

1. Prefer stable, explicit names over shorthand.
2. Use `lower_snake_case` for files and folders.
3. Keep capability/channel IDs explicit and domain-scoped.
4. Keep USF terminology consistent across scripts, Rust, and docs.

## Documentation Patterns

1. Keep canonical intent in `AGENTS.md` + the canonical docs in `docs/`.
2. Keep temporary planning/history in `documents/temp_stuff/`.
3. When code and docs diverge, document both:
   - current implementation,
   - target direction,
   and resolve with user direction before continuing.

## Validation Patterns

1. For engine/API edits: `cargo check -p core_mod_api`.
2. For integration-sensitive edits: `./build.sh dev` and `./run.sh dev`.
3. Prefer small, reproducible validation steps over broad assumptions.
