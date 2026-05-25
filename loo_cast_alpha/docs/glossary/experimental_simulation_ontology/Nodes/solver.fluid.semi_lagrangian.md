---
canonical_name: Solver: Semi-Lagrangian Advection
status: WIP-experimental
aliases:
  - Semi-Lagrangian Scheme
source_of_truth: []
ontology_experimental: true
ontology_id: solver.fluid.semi_lagrangian
node_class: solver
node_types:
  - solver
traits: []
projection_tags:
  - book:engineering_decision
  - book:computational_representation
activated_modules:
  - core.identity
  - type.solver
coordinate_annotations:
  S: continuum_fluid
  M: characteristic_advection
  D: deterministic
  C: nonconservative_unless_corrected
  K: discrete_grid
  L: local_interpolation_with_backtracing
  R: irreversible_numerical
  T: fixed_grid_topology
  I: high_dimensional
  O: fully_observable_numeric_state
  G: subgrid_filtering_compatible
  Lambda: dx_dt_to_zero
  Sigma: robust_under_large_dt
  Pi: grid
---

## Identity

- `display_name`: Characteristic backtracing advection update.
- `summary`: Unconditionally stable advection core often used in graphics and practical transport solvers.

## Activated Metadata Modules

### `type.solver`

- `method_family`: backtraced characteristic interpolation scheme.
- `order_of_accuracy`: interpolation and reconstruction dependent.
- `stability_characteristics`: stable at large time steps but introduces numerical diffusion.
- `complexity_model`: local backtrace and interpolation per cell.

## Declared Invariants

- Boundedness under monotone interpolation variants.

## Admissible Representations

- grid

## Edge Ledger (machine-parseable)

```yaml
edges:
  - rel: eng:recommended_for
    to: paradigm.pde.incompressible_navier_stokes
    direction: out
    confidence: 0.73
    when:
      robustness_over_sharpness: true
```

#tech_glossary
#experimental_ontology
