# TEMP Plan: Generic Sampler/Categorizer Pipeline

Date: 2026-04-03  
Status: active execution track (`partially_validated`)

## Goal

Make metric sampling and zone categorization generic, deterministic, and script-configured by scale contracts rather than hardcoded runtime ids.

## Target End State

1. `scale` binds `metric_set`, `metric_sampler_id`, `metric_categorizer_id`, and `zlm` explicitly.
2. Sampler/categorizer kernels are reusable algorithms with validated parameterization.
3. Runtime resolves sampler/categorizer by ids from scale contracts.
4. No fixed-id fallback authority in core runtime paths.
5. Runtime support contract is explicit:
   sampler ids must follow `metric_sampler.kernel.*`,
   categorizer ids must follow `metric_categorizer.kernel.*`.

## Core Runtime Contracts

1. `SamplerKernelContract`
   - kernel id
   - required input context
   - output metric vector shape constraints
2. `CategorizerKernelContract`
   - kernel id
   - compatible metric set/zlm constraints
   - deterministic classification guarantees
3. `ScaleSamplingContract`
   - scale id/index
   - metric set id
   - metric sampler id
   - metric categorizer id
   - zlm id

## Execution Steps

1. Kernel registration layer
   - define sampler/categorizer kernel registries in Rust.
   - expose script-facing ids with validation.
2. Scale binding hardening
   - require explicit `metric_sampler_id` + `metric_categorizer_id` on each scale contract.
   - validate metric-set and zlm compatibility at bootstrap.
3. Runtime resolution
   - route chunk sampling through resolved scale contracts.
   - remove hardcoded sampler/categorizer id assumptions.
4. Generic algorithm extraction
   - keep reusable sampler logic independent from concrete demo ids.
   - keep categorizer logic generic over metric set layout.
5. Determinism verification
   - test repeatability across restarts and load orders.
   - test compatibility rejection for malformed contracts.

## Acceptance Criteria

1. Two scales can select different sampler/categorizer ids without code branching.
2. Invalid scale-to-kernel binding fails startup with actionable diagnostics.
3. Deterministic outputs hold for equal input context and equal contracts.

## Risks and Mitigations

1. Risk: metric layout drift between metric sets and runtime vectors.
   - Mitigation: immutable metric-set index tables and validation gates.
2. Risk: accidental fallback to legacy default kernel ids.
   - Mitigation: remove fallback paths and require explicit binding.
3. Risk: heavy generic abstraction hurts performance.
   - Mitigation: keep kernel dispatch thin and data-oriented.

## Sequencing Notes

1. Keep this aligned with `UsfScaleDefinition { metric_sampler_id, metric_categorizer_id }`.
2. DPT terminology is retained only as historical aliasing context; runtime naming uses `metric_container`.
3. Prioritize correctness and deterministic diagnostics before optimization.
