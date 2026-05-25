---
canonical_name: Solver: Fractional-Step Projection
status: WIP-experimental
aliases:
  - Projection Method
source_of_truth: []
ontology_experimental: true
ontology_id: solver.fluid.projection_fractional_step
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
  M: operator_splitting_pde
  D: deterministic
  C: mass_conserving_target
  K: discrete_grid
  L: local_stencil_plus_global_poisson
  R: irreversible_numerical
  T: fixed_or_moving_grid_topology
  I: high_dimensional
  O: fully_observable_numeric_state
  G: closure_optional
  Lambda: dx_dt_to_zero
  Sigma: cfl_and_poisson_quality_dependent
  Pi: grid
---

## Identity

- `display_name`: Incompressible flow update via advection-diffusion step plus pressure projection.
- `summary`: Core method family for incompressible Navier-Stokes discretizations.

## Activated Metadata Modules

### `type.solver`

- `method_family`: fractional-step operator splitting with Poisson pressure correction.
- `order_of_accuracy`: reconstruction and time integrator dependent.
- `stability_characteristics`: CFL and pressure-solve quality control mass conservation and robustness.
- `complexity_model`: local flux update plus global Poisson solve each step.

### `trait.structure_preserving`

- `preserved_invariants`: divergence-free constraint and global mass consistency.
- `monitor_strategy`: divergence norm and pressure-residual tracking.

## Declared Invariants

- Low divergence residual after projection.

## Admissible Representations

- grid
- staggered_grid

## Edge Ledger (machine-parseable)

```yaml
edges:
  - rel: eng:recommended_for
    to: paradigm.pde.incompressible_navier_stokes
    direction: out
    confidence: 0.9
    when:
      incompressible_constraint: true
```

#tech_glossary
#experimental_ontology
