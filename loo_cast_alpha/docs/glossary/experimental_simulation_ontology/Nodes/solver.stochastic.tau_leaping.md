---
canonical_name: Solver: Tau-Leaping
status: WIP-experimental
aliases:
  - Tau Leaping
source_of_truth: []
ontology_experimental: true
ontology_id: solver.stochastic.tau_leaping
node_class: solver
node_types:
  - solver
traits:
  - multi_scale
projection_tags:
  - book:engineering_decision
  - book:computational_representation
  - book:mathematical_structure
activated_modules:
  - core.identity
  - type.solver
  - trait.multi_scale
coordinate_annotations:
  S: molecular_chemical
  M: stochastic_time_stepping
  D: stochastic
  C: weak_probability_conservation
  K: discrete_time_jump_approximation
  L: channel_local
  R: irreversible_sample_paths
  T: dynamic_state_graph
  I: moderate_to_high_dimensional
  O: fully_observable_numeric_state
  G: coarse_event_aggregation
  Lambda: tau_to_zero
  Sigma: step_size_limited
  Pi: event_batching
---

## Identity

- `display_name`: Batched stochastic update approximation for high-rate reaction systems.
- `summary`: Trades exact event ordering for computational throughput in stiff/high-frequency regimes.

## Activated Metadata Modules

### `type.solver`

- `method_family`: approximate jump-process time stepping.
- `order_of_accuracy`: weak-order, implementation-dependent.
- `stability_characteristics`: limited by leap-size validity and negativity-control policy.
- `complexity_model`: fewer update events than exact SSA at high event rates.

### `trait.multi_scale`

- `micro_scale`: individual reaction events.
- `macro_scale`: aggregate channel count evolution over leap windows.
- `bridge_operator`: Poisson/binomial event aggregation over adaptive `tau`.

## Declared Invariants

- Nonnegative count enforcement under guarded tau selection.

## Admissible Representations

- event_batching
- state_vector

## Edge Ledger (machine-parseable)

```yaml
edges:
  - rel: eng:recommended_for
    to: paradigm.chemical.gillespie_ssa
    direction: out
    confidence: 0.78
    when:
      high_event_rate: true
      exact_event_order_not_required: true
```

## Related Nodes

- [paradigm.chemical.gillespie_ssa](paradigm.chemical.gillespie_ssa.md)

#tech_glossary
#experimental_ontology
