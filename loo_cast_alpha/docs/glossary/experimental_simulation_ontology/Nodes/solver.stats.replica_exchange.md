---
canonical_name: Solver: Replica Exchange Sampling
status: WIP-experimental
aliases:
  - Parallel Tempering
source_of_truth: []
ontology_experimental: true
ontology_id: solver.stats.replica_exchange
node_class: solver
node_types:
  - solver
traits:
  - multi_scale
projection_tags:
  - book:engineering_decision
  - book:mathematical_structure
activated_modules:
  - core.identity
  - type.solver
  - trait.multi_scale
coordinate_annotations:
  S: information_scale
  M: multichain_stochastic_sampling
  D: ensemble
  C: exchange_balance_constraints
  K: discrete_or_continuous_state
  L: local_with_cross_replica_swaps
  R: irreversible_chain_paths
  T: replica_network_topology
  I: high_dimensional
  O: partially_observable
  G: coarse_ensemble_statistics
  Lambda: replicas_and_samples_to_infinity
  Sigma: swap_acceptance_dependent
  Pi: monte_carlo
---

## Identity

- `display_name`: Multi-replica MCMC with state exchanges across temperature or parameter ladders.
- `summary`: Improves mixing in multimodal landscapes.

## Activated Metadata Modules

### `type.solver`

- `method_family`: coupled Markov chains with exchange proposals.
- `order_of_accuracy`: asymptotic sampling convergence.
- `stability_characteristics`: swap schedule and ladder spacing strongly control efficiency.
- `complexity_model`: multiple chains plus communication/swap overhead.

### `trait.multi_scale`

- `micro_scale`: local chain transitions.
- `macro_scale`: cross-replica exploration and mode-hopping behavior.
- `bridge_operator`: replica-swap acceptance mapping.

## Declared Invariants

- Joint stationary distribution under detailed-balance-compatible swaps.

## Admissible Representations

- monte_carlo

## Edge Ledger (machine-parseable)

```yaml
edges:
  - rel: eng:recommended_for
    to: paradigm.statistical_mechanics.ensemble_inference
    direction: out
    confidence: 0.86
    when:
      multimodal_landscape: true
```

#tech_glossary
#experimental_ontology
