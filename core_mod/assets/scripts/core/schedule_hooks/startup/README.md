# Startup Hook Tests

`startup.rhai` is the startup-time test harness entrypoint.
It is not gameplay logic.

## Loading behavior

When startup hook runs, loader concatenates:

1. all `.rhai` files under this folder recursively (sorted by path),
2. then `../startup.rhai`.

This keeps startup tests categorized while preserving one executable entrypoint.

## Folder conventions

- `tests/reflection/`
  - reflection graph and metadata validation tests.
- `tests/ecs/`
  - Bevy ECS validation tests (World, Commands, Entity primitives, Query, Messages, iterators).
- `tests/examples/`
  - runnable working-example tests.
  - these currently depend on testing-only bridge modules.

Non-core scripting logic must live outside this harness tree (for example under
`scripts/<module_name>/...`).

## Testing gate

- Bridge registration and all startup test execution are gated by:
  - `rhai_binding/testing_enabled` (from `core_mod/assets/configs/config.toml`)
- Default behavior is disabled (`false`), so startup tests do not execute.

## Function naming convention

Startup helper functions are `private fn` by default and are orchestrated centrally.
Use explicit helper names, for example:

- `run_startup_test_*`
- `run_reflection_*`
- `run_ecs_*`
- `run_*_working_example_test_*`

Keep orchestration centralized in `00_startup_test_catalog.rhai`.
