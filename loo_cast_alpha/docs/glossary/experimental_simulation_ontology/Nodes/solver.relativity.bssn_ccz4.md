---
canonical_name: Solver: BSSN/CCZ4 Evolution Stack
status: WIP-experimental
aliases:
  - BSSN CCZ4 Solver
source_of_truth: []
ontology_experimental: true
ontology_id: solver.relativity.bssn_ccz4
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
  S: astrophysical_to_cosmological
  M: constrained_hyperbolic_time_stepping
  D: deterministic
  C: constraint_damped
  K: discrete_space_time
  L: local_stencil_with_global_constraints
  R: irreversible_numerical
  T: amr_or_uniform_grid_topology
  I: high_dimensional
  O: fully_observable_numeric_state
  G: gauge_and_constraint_controls
  Lambda: mesh_and_step_refinement
  Sigma: stable_under_cfl_and_constraint_controls
  Pi: grid_and_amr_grid
---

## Identity

- `display_name`: Numerical-relativity evolution stack for BSSN and CCZ4 equations.
- `summary`: Practical solver family for strong-field relativistic simulations.

## Activated Metadata Modules

### `type.solver`

- `method_family`: constrained hyperbolic PDE evolution with gauge drivers and damping controls.
- `order_of_accuracy`: discretization and reconstruction dependent.
- `stability_characteristics`: constrained by CFL, gauge, and constraint-damping parameter choices.
- `complexity_model`: high-dimensional stencil updates with optional AMR and elliptic solves.

### `trait.multi_scale`

- `micro_scale`: strong-field local curvature dynamics.
- `macro_scale`: global waveform and spacetime evolution.
- `bridge_operator`: AMR and scale-aware gauge policies.

## Declared Invariants

- Bounded constraint violations under tuned damping and resolution.

## Admissible Representations

- grid
- amr_grid

## Edge Ledger (machine-parseable)

```yaml
edges:
  - rel: eng:recommended_for
    to: paradigm.relativity.numerical_relativity_bssn
    direction: out
    confidence: 0.84
    when:
      full_spacetime_evolution_required: true
```

## Related Nodes

- [paradigm.relativity.numerical_relativity_bssn](paradigm.relativity.numerical_relativity_bssn.md)

#tech_glossary
#experimental_ontology
