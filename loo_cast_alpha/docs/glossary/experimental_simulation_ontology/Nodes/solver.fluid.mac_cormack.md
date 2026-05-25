---
canonical_name: Solver: MacCormack Advection Scheme
status: WIP-experimental
aliases:
  - MacCormack
source_of_truth: []
ontology_experimental: true
ontology_id: solver.fluid.mac_cormack
node_class: solver
node_types:
  - solver
traits:
  - discontinuity_handling
projection_tags:
  - book:engineering_decision
  - book:computational_representation
activated_modules:
  - core.identity
  - type.solver
  - trait.discontinuity_handling
coordinate_annotations:
  S: continuum_fluid
  M: predictor_corrector
  D: deterministic
  C: weakly_dissipative_or_dispersive
  K: discrete_grid
  L: local_stencil
  R: irreversible_numerical
  T: fixed_grid_topology
  I: high_dimensional
  O: fully_observable_numeric_state
  G: filter_compatible
  Lambda: dx_dt_to_zero
  Sigma: cfl_limited
  Pi: grid
---

## Identity

- `display_name`: Two-step predictor-corrector advection scheme.
- `summary`: Higher-fidelity advection option relative to basic first-order transport updates.

## Activated Metadata Modules

### `type.solver`

- `method_family`: explicit predictor-corrector finite-difference/volume advection.
- `order_of_accuracy`: second order in smooth regions.
- `stability_characteristics`: CFL-limited and oscillation-prone near steep gradients without limiting.
- `complexity_model`: two transport updates per step.

### `trait.discontinuity_handling`

- `shock_sensor`: gradient and smoothness indicators.
- `limiter_family`: flux limiters and monotonicity constraints.
- `entropy_fix_policy`: selective dissipation near discontinuities.

## Declared Invariants

- Conservative consistency when used in conservative flux form.

## Admissible Representations

- grid

## Edge Ledger (machine-parseable)

```yaml
edges:
  - rel: eng:recommended_for
    to: paradigm.pde.compressible_flow_with_shocks
    direction: out
    confidence: 0.71
    when:
      moderate_shock_strength: true
```

#tech_glossary
#experimental_ontology
