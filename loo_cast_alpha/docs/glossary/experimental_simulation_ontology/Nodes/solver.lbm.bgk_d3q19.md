---
canonical_name: Solver: BGK D3Q19 LBM
status: WIP-experimental
aliases:
  - BGK D3Q19
source_of_truth: []
ontology_experimental: true
ontology_id: solver.lbm.bgk_d3q19
node_class: solver
node_types:
  - solver
traits: []
projection_tags:
  - book:engineering_decision
  - book:computational_representation
  - book:mathematical_structure
activated_modules:
  - core.identity
  - type.solver
coordinate_annotations:
  S: mesoscopic_to_continuum
  M: discrete_velocity_kinetics
  D: deterministic
  C: weakly_dissipative
  K: discrete_lattice
  L: local_collision_streaming
  R: irreversible_relaxation
  T: fixed_lattice_topology
  I: high_dimensional
  O: fully_observable_numeric_state
  G: hydrodynamic_limit
  Lambda: lattice_refinement
  Sigma: relaxation_parameter_limited
  Pi: grid_lattice
---

## Identity

- `display_name`: Single-relaxation-time lattice Boltzmann solver on D3Q19 stencil.
- `summary`: Efficient local update scheme for weakly compressible fluid simulation.

## Activated Metadata Modules

### `type.solver`

- `method_family`: stream-collide lattice kinetic solver.
- `order_of_accuracy`: typically second-order with standard lattice settings.
- `stability_characteristics`: controlled by relaxation-time and Mach-number constraints.
- `complexity_model`: linear scaling with lattice cells and stencil size.

## Declared Invariants

- Discrete mass and momentum consistency under collision constraints.

## Admissible Representations

- grid_lattice

## Edge Ledger (machine-parseable)

```yaml
edges:
  - rel: eng:recommended_for
    to: paradigm.fluid.lattice_boltzmann
    direction: out
    confidence: 0.83
    when:
      weakly_compressible_flow: true
      local_stencil_priority: true
```

## Related Nodes

- [paradigm.fluid.lattice_boltzmann](paradigm.fluid.lattice_boltzmann.md)

#tech_glossary
#experimental_ontology
