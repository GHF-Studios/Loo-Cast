---
canonical_name: Solver: Fast Multipole Method
status: WIP-experimental
aliases:
  - FMM
source_of_truth: []
ontology_experimental: true
ontology_id: solver.nbody.fast_multipole_method
node_class: solver
node_types:
  - solver
  - nbody_accelerator
traits:
  - long_range
projection_tags:
  - book:engineering_decision
  - book:computational_representation
  - book:transform_morphism
activated_modules:
  - core.identity
  - type.solver
  - trait.long_range
coordinate_annotations:
  S: mesoscopic_to_cosmological
  M: hierarchical_expansion_method
  D: deterministic
  C: conservative_approximation
  K: particle_or_hybrid
  L: long_range
  R: reversible_model_dependent
  T: adaptive_tree_topology
  I: high_dimensional
  O: fully_observable_numeric_state
  G: multipole_coarse_graining
  Lambda: truncation_order_to_infinity
  Sigma: stable_with_error_control
  Pi: tree_or_particle_mesh
---

## Identity

- `display_name`: Hierarchical multipole acceleration for long-range interactions.
- `summary`: Reduces all-pairs long-range interaction cost via controlled far-field approximation.

## Activated Metadata Modules

### `type.solver`

- `method_family`: hierarchical tree and multipole expansion accelerator.
- `order_of_accuracy`: expansion-order dependent.
- `stability_characteristics`: accuracy controlled by opening-angle and truncation policies.
- `complexity_model`: near `O(N)` to `O(N log N)` depending on implementation and dimension.

### `trait.long_range`

- `accelerator_family`: multipole expansion with near/far decomposition.
- `error_control`: expansion-order and acceptance-criterion tuning.
- `far_field_model`: cluster-level potential/force approximation.

## Declared Invariants

- Controlled error in far-field force/potential approximation.

## Admissible Representations

- particle
- tree
- particle_mesh_hybrid

## Edge Ledger (machine-parseable)

```yaml
edges:
  - rel: eng:recommended_for
    to: paradigm.particle.nbody_dynamics
    direction: out
    confidence: 0.88
    when:
      long_range_interaction: true
      particle_count_large: true
  - rel: math:connected_by_duality
    to: morphism.duality.particle_field
    direction: out
    confidence: 0.74
```

## Related Nodes

- [paradigm.particle.nbody_dynamics](paradigm.particle.nbody_dynamics.md)
- [morphism.duality.particle_field](morphism.duality.particle_field.md)

#tech_glossary
#experimental_ontology
