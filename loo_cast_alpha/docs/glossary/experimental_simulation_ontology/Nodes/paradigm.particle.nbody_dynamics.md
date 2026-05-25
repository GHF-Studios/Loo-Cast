---
canonical_name: Paradigm: Particle N-Body Dynamics
status: WIP-experimental
aliases:
  - N-Body Particle Dynamics
source_of_truth: []
ontology_experimental: true
ontology_id: paradigm.particle.nbody_dynamics
node_class: paradigm
node_types:
  - particle_system
traits:
  - nonlocal
projection_tags:
  - book:physical_scale
  - book:mathematical_structure
  - book:engineering_decision
  - book:transform_morphism
activated_modules:
  - core.identity
  - trait.nonlocal
coordinate_annotations:
  S: mesoscopic_to_cosmological
  M: ODE_system
  D: deterministic_or_stochastic
  C: Hamiltonian_or_dissipative
  K: particle
  L: long_range
  R: reversible_or_irreversible
  T: dynamic_connectivity
  I: high_dimensional
  O: partially_observable
  G: mean_field_or_statistical
  Lambda: N_to_infinity
  Sigma: chaotic
  Pi: particle_or_tree
---

## Identity

- `display_name`: Interacting particle system with pairwise or field-mediated forces.
- `summary`: Canonical particle representation for gravitational, electrostatic, and related many-body dynamics.

## Activated Metadata Modules

### `trait.nonlocal`

- `interaction_kernel`: inverse-power and screened-kernel families.
- `support_radius`: effectively global unless screened or truncated.
- `decay_behavior`: algebraic or exponential depending on physics.

## Declared Invariants

- Energy and momentum under conservative closed-system assumptions.

## Admissible Representations

- particle
- tree
- particle_mesh_hybrid

## Edge Ledger (machine-parseable)

```yaml
edges:
  - rel: eng:recommended_solver
    to: solver.nbody.fast_multipole_method
    direction: out
    confidence: 0.88
    when:
      long_range_interaction: true
      particle_count_large: true
  - rel: math:duality_instance
    to: morphism.duality.particle_field
    direction: out
    confidence: 0.8
```

## Related Nodes

- [solver.nbody.fast_multipole_method](solver.nbody.fast_multipole_method.md)
- [morphism.duality.particle_field](morphism.duality.particle_field.md)

#tech_glossary
#experimental_ontology
