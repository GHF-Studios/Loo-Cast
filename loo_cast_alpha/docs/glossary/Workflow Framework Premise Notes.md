# Workflow Framework Premise Notes

#tech_glossary

Related glossary terms:

- [Workflow Framework](Workflow%20Framework.md)
- [Concept Declaration Artifact](Concept%20Declaration%20Artifact.md)
- [Rhai Capability](Rhai%20Capability.md)
- [Rust Capability](Rust%20Capability.md)
- [Runtime Substrate](Runtime%20Substrate.md)
- [Modding Runtime](Modding%20Runtime.md)
- [Capability Runtime](Capability%20Runtime.md)
- [USF Runtime](USF%20Runtime.md)

Current premise slice (draft, intentionally not frozen):

1. Workflow framework scope is Rust-side orchestration semantics, not Rhai-side capability/domain exposure policy.
2. Stage execution should stay scheduler-visible as normal Bevy systems with typed `SystemParam` access.
3. Control-plane lifecycle handling and execution-plane stage logic are separate concerns.
4. `ECS`, `Render`, and `Async` are first-class distinct workflow domains.
5. `EcsWhile` and `RenderWhile` are core iterative variants for non-async contexts.
6. Workflow stages should orchestrate materialized Rust-side runtime artifacts, not raw Rhai engine internals.
7. If declaration/materialization progression is orchestrated by workflow, it should be mediated through explicit
   Rust-side capability contracts rather than ad hoc script engine calls.

Why this premise is currently high-signal:

1. Generated stage poll systems are normal Bevy systems (typed params), so scheduler conflict analysis and parallelism
   are preserved.
2. A significant part of current orchestration still uses exclusive `&mut World` systems, which is refactor debt rather
   than a useful premise.
3. The framework already separates domain stage families (`Ecs`/`Render`/`Async` + while variants), matching the
   intended mental model.

Near-term direction (still draft):

1. Keep stage execution in typed systems (do not collapse into exclusive global dispatch).
2. Incrementally move orchestration/control systems away from exclusive `&mut World` patterns where feasible.
3. Preserve first-class domain distinctions and while-stage semantics through refactor.
4. Refine and pressure-test run-model semantics (identity keys, concurrency policy, cancellation behavior).

Run identity + concurrency draft v0.1 (still draft, not frozen):

1. Workflow run identity is Rust-side only and must not be keyed by raw Rhai engine handles.
2. Each run has a stable `run_id` plus a semantic `concurrency_key`.
3. `concurrency_key` is derived from workflow kind + workflow domain (`ECS`/`Render`/`Async`) + target scope key.
4. Target scope keys point to runtime-owned targets/materialized artifacts, not declaration script file identities.
5. `EcsWhile`/`RenderWhile` iterations keep one `run_id`; iterations are lifecycle transitions, not new runs.
6. Admission policy: conflicting `concurrency_key` values serialize; non-conflicting keys can run in parallel.
7. Conflict handling policy can choose `reject`, `queue`, or `replace`; deterministic `queue` is the current default
   candidate.
8. Cancellation is cooperative and keyed by `run_id`; `completed`/`failed`/`cancelled` are terminal states.
9. Observability minimum per transition: `run_id`, `concurrency_key`, stage domain, lifecycle transition, outcome.
10. If workflow orchestrates declaration/materialization progression, it should do so via explicit Rust capability
    contracts, not ad hoc script-engine calls.

Legacy source pointers:

- `loo_cast_legacy/core_mod_api/src/backend/workflow/mod.rs`
- `loo_cast_legacy/core_mod_api/src/backend/workflow/systems.rs`
- `loo_cast_legacy/core_engine_macros/src/define_workflow_mod_OLD/mod.rs`
- `loo_cast_legacy/core_engine_macros/src/define_workflow_mod_OLD/core_function.rs`
- `loo_cast_legacy/core_engine_macros/src/define_workflow_mod_OLD/core_type.rs`
