# USF Math

Purpose: focused math contract summary for cross-scale and local-runtime interoperation.

## Core Direction

1. USF has a global, scale-aware math space.
2. Runtime capability internals should use local numeric domains where valid (`f32/f64/i32/i64` style).
3. Global-to-local and local-to-global conversion boundaries must be explicit.
4. Expensive global math usage should be visible and deliberate.

## Contract Framing

- Cross-scale logic/data operations require scale-aware math paths.
- Same-scale operations should prefer local numeric kernels when bounded and correct.
- Capability channels are allowed to compute locally and reconcile canonically through explicit adapters.
- 71 scales are a structural backbone for math/context partitioning.

## Open Questions

1. Exact canonical numeric types and operator sets for global math core.
2. Formal precision/drift/error budget contracts across repeated adapter roundtrips.
3. What minimum global math surface every capability must consume directly.
4. Where global math remains mandatory even in otherwise local runtime logic.

## Diagram Pair

- `../intention_records/usf_records/60_usf_math_contracts.puml`
