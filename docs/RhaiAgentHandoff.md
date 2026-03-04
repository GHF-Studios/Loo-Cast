# Rhai Agent Handoff

Purpose: give a fresh agent enough context to continue Rhai-binding work without
chat history.

## Hard Rules (Current Agreement)

- Global Rhai namespace is reserved for top-level modules and only truly central
  primitives.
- Do not add ad-hoc global helper functions for hook/testing orchestration.
- Schedule hook registration must go through
  `rhai_binding::schedule_hooks::add`.
- Testing gate must use config key `rhai_binding/testing_enabled`
  (not env vars).
- Keep `shop::divisions::sex` testing bridge content; do not delete it.
- Startup scripts are the integration-test entrypoint
  (`./build.sh dev` then `./run.sh dev`).
- `AccessCell` + provider pattern is the intended backbone for ECS/sysparam
  safety and value semantics.
- Arc/`rhai::Shared` is discouraged for mutable/scoped semantics; acceptable
  only for explicit persistent readonly-style semantics.

## Current Runtime Shape

- Boot registration:
  - `core_mod/assets/scripts/core/boot.rhai` loops hook names and calls
    `rhai_binding::schedule_hooks::add(hook)`.
- Engine runtime modules (non-bridge internals):
  - `rhai_binding::schedule_hooks::add`
  - `rhai_binding::testing::enabled`
  - both registered in
    `core_mod_api/src/rhai_binding/engine/bootstrap.rs`.
- Startup test split:
  - startup tests (including example-tests) run only when
    `rhai_binding::testing::enabled() == true`.
- Startup test helper functions are `private fn` to reduce script-global
  pollution.

## Where To Read First

1. `AGENT_TODO.md` (task sequencing + approval gates)
2. `docs/RhaiDialect.md` (architecture decisions)
3. `docs/RhaiValueSemantics.md` (semantics/lifecycle intent)
4. `docs/RhaiBridgeDevelopment.md` (extension workflow)
5. `docs/RhaiBindingRoadmap.md` (coverage backlog)

## Open Tasks (As Of This Handoff)

- Task 1: complete
- Task 2: complete
- Task 3: complete
- Task 4: complete
- Task 5: complete

## Known Directional Constraints

- Favor Rust/Bevy path mirroring in bridge domain/module naming.
- Keep generic behavior explicit: runtime descriptors + pre-registered
  monomorphized catalogs.
- Keep generic dispatch policy centralized in
  `rhai_binding::runtime::ecs::dispatch_policy`.
- Do not replace startup-hook integration testing with isolated test-only
  harnesses as the primary validation path.

## Recent Cleanup Notes

- Removed dead compatibility/export scaffolding:
  - `core_mod_api::access` shim module
  - empty `gpu::external` module tree
  - empty `logging::{functions,systems}` and `usf::{components,systems}` files
  - unused placeholder runtime submodules under
    `rhai_binding::runtime::{ecs,usf}`
- `reflection::{ids,traits}` remain as active reflection-facing alias modules,
  but are now documented as aliases (not legacy shims).
