# Rhai Bridge Tree

This directory is the executable reflection/registration surface for the Rhai dialect.

## Subtrees

- `domains/`
  - production bridge modules that mirror exposed runtime domains (`ecs`, `player`, `rust`, ...).
- `testing/`
  - testing-only bridge domains used by startup tests and bridge smoke checks.

## Design rule

Keep domain modules mirrored and decentralized. Avoid funneling unrelated bridge registrations into one giant module.

For extension workflow, see:

- `docs/RhaiDialect.md`
- `docs/RhaiBridgeDevelopment.md`
