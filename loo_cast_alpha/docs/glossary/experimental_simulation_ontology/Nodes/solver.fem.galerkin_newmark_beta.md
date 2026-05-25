---
canonical_name: Solver: Galerkin FEM with Newmark-Beta Time Integration
status: WIP-experimental
aliases:
  - FEM Newmark-Beta
source_of_truth: []
ontology_experimental: true
ontology_id: solver.fem.galerkin_newmark_beta
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
  S: continuum_structural
  M: weak_form_discretization
  D: deterministic
  C: model_dependent_conservation
  K: discrete_space_time
  L: element_local_with_global_solve
  R: irreversible_numerical
  T: mesh_topology
  I: high_dimensional
  O: fully_observable_numeric_state
  G: multiscale_constitutive_compatible
  Lambda: mesh_and_step_refinement
  Sigma: stable_parameter_regime_required
  Pi: mesh
---

## Identity

- `display_name`: Finite-element Galerkin structural solver with Newmark family time integration.
- `summary`: Common structural-dynamics method for linear and nonlinear transient analysis.

## Activated Metadata Modules

### `type.solver`

- `method_family`: finite-element weak-form discretization plus implicit/explicit Newmark updates.
- `order_of_accuracy`: second order in standard parameter choices.
- `stability_characteristics`: conditionally or unconditionally stable depending on Newmark parameters.
- `complexity_model`: sparse global linear or nonlinear solve per time step.

### `trait.multi_scale`

- `micro_scale`: element-level constitutive integration.
- `macro_scale`: global displacement and stress field response.
- `bridge_operator`: assembly and constitutive homogenization mapping.

## Declared Invariants

- Weak-form consistency and momentum-balance residual control.

## Admissible Representations

- finite_element_mesh
- sparse_system

## Edge Ledger (machine-parseable)

```yaml
edges:
  - rel: eng:recommended_for
    to: paradigm.continuum.solid_mechanics_pde
    direction: out
    confidence: 0.85
    when:
      transient_structural_response: true
```

## Related Nodes

- [paradigm.continuum.solid_mechanics_pde](paradigm.continuum.solid_mechanics_pde.md)

#tech_glossary
#experimental_ontology
