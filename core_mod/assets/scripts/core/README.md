# Core Rhai Scripts

This directory is the canonical Rhai asset root for core runtime scripting.

## Layout

- `boot.rhai`
  - Registers schedule hooks via `rhai_binding::schedule_hooks::add`.
- `schedule_hooks/`
  - Hook entrypoint scripts (`pre_startup.rhai`, `startup.rhai`, etc.).
  - Companion folders (same name as hook file) contain categorized test files loaded before the root hook file.

Non-core scripting logic should live under its own top-level module path, for example:

- `scripts/other_module/...`

## Important references

- `docs/Scripting.md`
- `docs/RhaiDialect.md`
- `docs/RhaiBridgeDevelopment.md`
