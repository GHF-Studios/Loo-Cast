---
canonical_name: Paradigm: Lattice Boltzmann Method
status: WIP-experimental
aliases:
  - LBM
source_of_truth: []
ontology_experimental: true
ontology_id: paradigm.fluid.lattice_boltzmann
node_class: paradigm
node_types:
  - mesoscopic_lattice_method
traits:
  - multi_scale
projection_tags:
  - book:physical_scale
  - book:mathematical_structure
  - book:engineering_decision
  - book:computational_representation
activated_modules:
  - core.identity
  - trait.multi_scale
coordinate_annotations:
  S: mesoscopic_to_continuum
  M: kinetic_discrete_velocity
  D: deterministic_or_stochastic_extensions
  C: weakly_dissipative
  K: hybrid_discrete_continuous
  L: local_stream_collide
  R: irreversible_relaxation
  T: fixed_lattice_topology
  I: high_dimensional
  O: partially_observable
  G: hydrodynamic_limit
  Lambda: Knudsen_to_continuum_limit
  Sigma: stability_limited_by_relaxation
  Pi: grid_lattice
---

## Identity

- `display_name`: Mesoscopic kinetic-lattice formulation for fluid transport.
- `summary`: Uses discrete-velocity distribution functions with local collision and streaming updates.

## Activated Metadata Modules

### `trait.multi_scale`

- `micro_scale`: particle-distribution dynamics on velocity lattice.
- `macro_scale`: recovered hydrodynamic moments (density, velocity, stress).
- `bridge_operator`: moment closure and Chapman-Enskog asymptotic mapping.

## Declared Invariants

- Conserved low-order moments under collision-operator constraints.

## Admissible Representations

- grid_lattice
- distribution_function

## Edge Ledger (machine-parseable)

```yaml
edges:
  - rel: eng:recommended_solver
    to: solver.lbm.bgk_d3q19
    direction: out
    confidence: 0.83
    when:
      weakly_compressible_flow: true
      local_stencil_priority: true
```

## Related Nodes

- [solver.lbm.bgk_d3q19](solver.lbm.bgk_d3q19.md)

#tech_glossary
#experimental_ontology
