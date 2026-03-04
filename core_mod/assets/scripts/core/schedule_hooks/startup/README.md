# Startup Hook Suites

`startup.rhai` is the integration-test-like entrypoint for bridge validation.

## Loading behavior

When startup hook runs, loader concatenates:

1. all `.rhai` files under this folder recursively (sorted by path),
2. then `../startup.rhai`.

This lets us split suites by concern while keeping one executable entrypoint.

## Folder conventions

- `reflection/`
  - reflection graph and metadata smoke coverage.
- `ecs/`
  - Bevy ECS bridge examples (World, Commands, Entity primitives, Query, Messages, iterators).
- `testing/`
  - testing-only bridges that are intentionally not production API.
  - loaded and executed only when `rhai_binding/testing/bridges_enabled = true` in `config.toml`.

## Testing gate

- Bridge registration and startup invocation for testing-only modules are both gated by:
  - `rhai_binding/testing/bridges_enabled` (from `core_mod/assets/configs/config.toml`)
- Default behavior is disabled (production-safe startup surface).

## Function naming convention

Use explicit suite function names, for example:

- `run_reflection_*`
- `run_ecs_*`
- `run_testing_*`

Keep orchestration centralized in `00_example_catalog.rhai`.
