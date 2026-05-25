---
canonical_name: Solver: Particle-Particle Particle-Mesh Gravity
status: WIP-experimental
aliases:
  - P3M
source_of_truth: []
ontology_experimental: true
ontology_id: solver.gravity.p3m
node_class: solver
node_types:
  - solver
traits:
  - long_range
projection_tags:
  - book:physical_scale
  - book:engineering_decision
  - book:computational_representation
activated_modules:
  - core.identity
  - type.solver
  - trait.long_range
coordinate_annotations:
  S: astrophysical_to_cosmological
  M: hybrid_nbody_solver
  D: deterministic
  C: conservative_approximation
  K: particle_mesh_hybrid
  L: long_range_with_local_correction
  R: reversible_or_irreversible_model_dependent
  T: adaptive_mesh_or_tree_topology
  I: high_dimensional
  O: fully_observable_numeric_state
  G: coarse_far_field
  Lambda: mesh_and_particle_refinement
  Sigma: stable_with_force_splitting_controls
  Pi: particle_mesh_hybrid
---

## Identity

- `display_name`: Hybrid gravity solver combining near-field particle interaction and mesh-based far field.
- `summary`: Standard approach for large-scale gravitating systems with long-range coupling.

## Activated Metadata Modules

### `type.solver`

- `method_family`: particle-particle plus particle-mesh force splitting.
- `order_of_accuracy`: split and interpolation dependent.
- `stability_characteristics`: stable when force-split and time-step constraints are respected.
- `complexity_model`: near-field local interactions plus FFT or mesh Poisson solves.

### `trait.long_range`

- `accelerator_family`: mesh Poisson solve for far field with local particle correction.
- `error_control`: split-scale, interpolation, and force-softening error diagnostics.
- `far_field_model`: grid-based long-range potential approximation.

## Declared Invariants

- Controlled force-splitting error envelope.

## Admissible Representations

- particle_mesh_hybrid
- grid

## Edge Ledger (machine-parseable)

```yaml
edges:
  - rel: eng:recommended_for
    to: paradigm.astro.grmhd
    direction: out
    confidence: 0.72
    when:
      gravity_long_range_coupling: true
  - rel: eng:recommended_for
    to: paradigm.particle.nbody_dynamics
    direction: out
    confidence: 0.8
    when:
      very_large_particle_count: true
```

## Related Nodes

- [paradigm.astro.grmhd](paradigm.astro.grmhd.md)
- [paradigm.particle.nbody_dynamics](paradigm.particle.nbody_dynamics.md)

#tech_glossary
#experimental_ontology
