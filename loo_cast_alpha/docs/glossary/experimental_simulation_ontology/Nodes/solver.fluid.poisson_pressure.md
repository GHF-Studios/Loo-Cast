---
canonical_name: Solver: Pressure Poisson Solver Stack
status: WIP-experimental
aliases:
  - Pressure Projection Poisson
source_of_truth: []
ontology_experimental: true
ontology_id: solver.fluid.poisson_pressure
node_class: solver
node_types:
  - solver
traits:
  - structure_preserving
projection_tags:
  - book:engineering_decision
  - book:computational_representation
activated_modules:
  - core.identity
  - type.solver
  - trait.structure_preserving
coordinate_annotations:
  S: continuum_fluid
  M: elliptic_linear_solve
  D: deterministic
  C: mass_constraint_enforcement
  K: discrete_grid
  L: global_elliptic_coupling
  R: irreversible_numerical
  T: mesh_or_grid_topology
  I: high_dimensional
  O: fully_observable_numeric_state
  G: multigrid_coarsening
  Lambda: mesh_refinement
  Sigma: solver_tolerance_dependent
  Pi: grid_or_sparse_system
---

## Identity

- `display_name`: Elliptic pressure solve component for incompressible projection methods.
- `summary`: Enforces divergence constraints through global pressure correction.

## Activated Metadata Modules

### `type.solver`

- `method_family`: sparse elliptic linear system solvers (CG, multigrid, FFT variants).
- `order_of_accuracy`: discretization dependent.
- `stability_characteristics`: divergence control depends on residual tolerance and boundary consistency.
- `complexity_model`: global sparse solve dominates for large 3D domains.

### `trait.structure_preserving`

- `preserved_invariants`: low divergence and global mass consistency.
- `monitor_strategy`: residual norms and divergence diagnostics.

## Declared Invariants

- Pressure-corrected velocity with controlled divergence residual.

## Admissible Representations

- grid
- sparse_system

## Edge Ledger (machine-parseable)

```yaml
edges:
  - rel: comp:implements_time_integration_for
    to: paradigm.pde.incompressible_navier_stokes
    direction: out
    confidence: 0.88
```

#tech_glossary
#experimental_ontology
