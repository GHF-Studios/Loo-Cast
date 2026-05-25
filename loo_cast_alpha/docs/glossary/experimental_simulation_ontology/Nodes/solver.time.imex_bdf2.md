---
canonical_name: Solver: IMEX BDF2 Time Integrator
status: WIP-experimental
aliases:
  - IMEX-BDF2
source_of_truth: []
ontology_experimental: true
ontology_id: solver.time.imex_bdf2
node_class: solver
node_types:
  - solver
  - ODE_integrator
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
  S: multi_scale
  M: ODE_split_integrator
  D: deterministic
  C: weakly_dissipative
  K: discrete_time
  L: local_time_operator
  R: irreversible_numerical
  T: fixed_discretization_topology
  I: dimensionality_inherited
  O: fully_observable_numeric_state
  G: coarse_subcycling_compatible
  Lambda: dt_to_zero
  Sigma: A_stable_implicit_part
  Pi: grid_or_mesh
---

## Identity

- `display_name`: Second-order implicit-explicit backward differentiation integrator.
- `summary`: Handles stiff and non-stiff operator splits with implicit and explicit partitions.

## Activated Metadata Modules

### `type.solver`

- `method_family`: IMEX multistep time integration.
- `order_of_accuracy`: second order.
- `stability_characteristics`: stiffly stable for implicit partition; CFL-limited explicit partition.
- `complexity_model`: linear/nonlinear solve per step plus explicit flux evaluation.

### `trait.multi_scale`

- `micro_scale`: fast stiff source or diffusion modes.
- `macro_scale`: slower advective/transport modes.
- `bridge_operator`: operator splitting with partitioned update policy.

## Declared Invariants

- Stability envelope tied to split consistency and solver tolerance.

## Admissible Representations

- grid
- mesh
- finite_element_state

## Edge Ledger (machine-parseable)

```yaml
edges:
  - rel: eng:recommended_for
    to: paradigm.pde.incompressible_navier_stokes
    direction: out
    confidence: 0.84
    when:
      stiffness: high
      multiple_time_scales: true
  - rel: comp:implements_time_integration_for
    to: paradigm.pde.incompressible_navier_stokes
    direction: out
    confidence: 0.84
```

## Related Nodes

- [paradigm.pde.incompressible_navier_stokes](paradigm.pde.incompressible_navier_stokes.md)

#tech_glossary
#experimental_ontology
