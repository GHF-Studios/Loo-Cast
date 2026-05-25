---
canonical_name: Solver: Maxwell-Yee Grid Scheme
status: WIP-experimental
aliases:
  - Yee FDTD
source_of_truth: []
ontology_experimental: true
ontology_id: solver.quantum.maxwell_yee
node_class: solver
node_types:
  - solver
traits:
  - discontinuity_handling
projection_tags:
  - book:physical_scale
  - book:computational_representation
  - book:engineering_decision
activated_modules:
  - core.identity
  - type.solver
  - trait.discontinuity_handling
coordinate_annotations:
  S: quantum_to_plasma
  M: finite_difference_time_domain
  D: deterministic
  C: weakly_dissipative_numerical
  K: discrete_grid
  L: local_stencil
  R: irreversible_numerical
  T: fixed_grid_topology
  I: high_dimensional
  O: fully_observable_numeric_state
  G: coarse_grained_optional
  Lambda: dx_dt_to_zero
  Sigma: cfl_limited
  Pi: grid
---

## Identity

- `display_name`: Staggered-grid FDTD solver for Maxwell equations.
- `summary`: Standard electromagnetic wave propagation solver and a core component of PIC workflows.

## Activated Metadata Modules

### `type.solver`

- `method_family`: leapfrog time-stepped finite-difference FDTD.
- `order_of_accuracy`: second order in common stencil forms.
- `stability_characteristics`: CFL-limited and sensitive to material dispersion modeling.
- `complexity_model`: linear scaling with grid cells and time steps.

### `trait.discontinuity_handling`

- `shock_sensor`: field-jump detection and interface diagnostics.
- `limiter_family`: slope-limited variants in coupled hyperbolic updates.
- `entropy_fix_policy`: energy-compatible damping near discontinuities.

## Declared Invariants

- Discrete curl-consistency structure of staggered updates.

## Admissible Representations

- grid

## Edge Ledger (machine-parseable)

```yaml
edges:
  - rel: eng:recommended_for
    to: paradigm.quantum.relativistic_pic
    direction: out
    confidence: 0.82
    when:
      em_field_update_required: true
```

#tech_glossary
#experimental_ontology
