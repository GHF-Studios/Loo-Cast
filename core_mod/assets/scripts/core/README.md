# Core Rhai Scripts

This directory is the canonical Rhai asset root for core runtime scripting.

## Layout

- `boot.rhai`
  - Registers schedule entrypoints via `rhai_binding::schedule_entrypoints::add`.
- `../ecs/schedule_entrypoints/`
  - Schedule entrypoint scripts (`pre_startup.rhai`, `startup.rhai`, etc.).
  - Companion folders (same name as entrypoint file) contain categorized files loaded before the root entrypoint file.

Non-core scripting logic should live under its own top-level module path, for example:

- `scripts/other_module/...`

## Important references

- `docs/Scripting.md`
- `docs/RhaiDialect.md`
- `docs/RhaiBridgeDevelopment.md`
