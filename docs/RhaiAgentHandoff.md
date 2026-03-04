# Rhai Agent Policy

Purpose: permanent pre-instructions for AI agents working on Rhai bindings and
Rhai script runtime behavior in this repository.

## Scope

This document defines durable rules and expected workflow. It does not track
task status, temporary migrations, or historical cleanup notes.

## Non-Negotiable Rules

- Global Rhai namespace is reserved for truly central primitives and
  top-level modules.
- Do not add ad-hoc global helper functions for hook or test orchestration.
- Schedule hook registration must go through
  `rhai_binding::schedule_hooks::add`.
- Testing gate must use config key `rhai_binding/testing_enabled` from
  config, not env vars.
- Startup tests/examples must execute only when
  `rhai_binding::testing::enabled() == true`.
- Keep testing/example bridge content available for harness coverage (for
  example `shop::divisions::sex`).
- Startup hook scripts are the integration test entrypoint:
  `./build.sh dev` then `./run.sh dev`.
- `AccessCell` + provider patterns are the default semantics backbone for
  ECS/sysparam safety.
- `Arc`/`rhai::Shared` is not the default mutable/scoped model; only use it for
  explicit persistent/readonly semantics.

## Rhai Runtime Layout Policy

- `core_mod/assets/scripts/core/boot.rhai` is responsible for registering
  schedule hooks.
- Runtime utility modules exposed to scripts remain explicit and minimal:
  `rhai_binding::schedule_hooks`, `rhai_binding::testing`, etc.
- Startup harness code under
  `core_mod/assets/scripts/core/schedule_hooks/startup/` is tests/examples, not
  gameplay logic.
- `core` is a reserved script module name for core functionality.
  Non-core scripting logic should live under dedicated module paths
  (for example `scripts/<module_name>/...`).

## Generic Binding Policy

- Rhai generic-like behavior must stay explicit:
  reflected generic metadata + compile-time monomorphized catalogs + runtime
  resolver dispatch.
- Centralize generic dispatch validation/registration conventions in
  `core_mod_api/src/rhai_binding/runtime/ecs/dispatch_policy.rs`.
- Prefer policy macros and validators over ad-hoc per-catalog rules.
- Favor Rust/Bevy path mirroring in bridge domain/module naming.

## Startup Test Harness Policy

- Keep orchestration centralized in
  `core_mod/assets/scripts/core/schedule_hooks/startup/startup_test_catalog.rhai`.
- Use descriptive file names for test scripts; do not rely on numeric filename
  prefixes for semantics.
- Keep helper functions `private fn` unless broader visibility is intentional.

## Required Validation When Touching Rhai Bindings

- `cargo check -p core_mod_api`
- Focused unit tests for edited areas (for example dispatch policy and
  preprocess tests).
- Integration pass:
  `./build.sh dev` and a bounded `./run.sh dev` check.

## Primary Reference Docs

1. `AGENT_TODO.md`
2. `docs/RhaiDialect.md`
3. `docs/RhaiBridgeDevelopment.md`
4. `docs/RhaiGenericBindingPolicy.md`
5. `docs/RhaiValueSemantics.md`
