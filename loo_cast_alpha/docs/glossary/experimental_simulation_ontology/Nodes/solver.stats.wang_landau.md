---
canonical_name: Solver: Wang-Landau Density-of-States Sampling
status: WIP-experimental
aliases:
  - Wang-Landau
source_of_truth: []
ontology_experimental: true
ontology_id: solver.stats.wang_landau
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
  M: adaptive_stochastic_sampling
  D: ensemble
  C: adaptive_biasing
  K: discrete_or_continuous_state
  L: state_space_binning
  R: irreversible_adaptive_chain
  T: histogram_topology
  I: high_dimensional
  O: partially_observable
  G: coarse_density_of_states
  Lambda: iteration_to_flat_histogram_limit
  Sigma: adaptation_schedule_dependent
  Pi: monte_carlo
---

## Identity

- `display_name`: Adaptive density-of-states estimator and sampler.
- `summary`: Targets flat-histogram exploration for difficult free-energy landscapes.

## Activated Metadata Modules

### `type.solver`

- `method_family`: adaptive biasing Monte Carlo with density-of-states updates.
- `order_of_accuracy`: asymptotic convergence under schedule conditions.
- `stability_characteristics`: convergence sensitive to modification-factor schedules and binning design.
- `complexity_model`: long adaptive runs with histogram convergence checks.

## Declared Invariants

- Histogram-flattening convergence criteria under chosen update schedule.

## Admissible Representations

- monte_carlo

## Edge Ledger (machine-parseable)

```yaml
edges:
  - rel: eng:recommended_for
    to: paradigm.statistical_mechanics.ensemble_inference
    direction: out
    confidence: 0.82
    when:
      free_energy_landscape_estimation: true
```

#tech_glossary
#experimental_ontology
