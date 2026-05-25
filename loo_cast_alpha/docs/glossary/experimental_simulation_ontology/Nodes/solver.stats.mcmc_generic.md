---
canonical_name: Solver: Generic MCMC Engine
status: WIP-experimental
aliases:
  - MCMC
source_of_truth: []
ontology_experimental: true
ontology_id: solver.stats.mcmc_generic
node_class: solver
node_types:
  - solver
traits: []
projection_tags:
  - book:engineering_decision
  - book:mathematical_structure
activated_modules:
  - core.identity
  - type.solver
coordinate_annotations:
  S: information_scale
  M: stochastic_sampling
  D: ensemble
  C: detailed_balance_or_nonreversible_variant
  K: discrete_or_continuous_state
  L: kernel_local_or_global
  R: irreversible_chain_paths
  T: state_graph_topology
  I: high_dimensional
  O: partially_observable
  G: coarse_statistics
  Lambda: sample_count_to_infinity
  Sigma: mixing_rate_dependent
  Pi: monte_carlo
---

## Identity

- `display_name`: General-purpose Markov chain sampling engine.
- `summary`: Baseline posterior/ensemble sampling solver across statistical-mechanics and Bayesian domains.

## Activated Metadata Modules

### `type.solver`

- `method_family`: Markov transition kernels with acceptance or direct transitions.
- `order_of_accuracy`: not applicable in classical deterministic sense; convergence is asymptotic.
- `stability_characteristics`: determined by ergodicity, mixing, and proposal quality.
- `complexity_model`: per-step kernel cost times effective sample size requirements.

## Declared Invariants

- Target stationary distribution under valid kernel construction.

## Admissible Representations

- monte_carlo
- graph

## Edge Ledger (machine-parseable)

```yaml
edges:
  - rel: eng:recommended_for
    to: paradigm.statistical_mechanics.ensemble_inference
    direction: out
    confidence: 0.9
  - rel: eng:recommended_for
    to: paradigm.bayesian_inference.posterior_dynamics
    direction: out
    confidence: 0.88
```

#tech_glossary
#experimental_ontology
