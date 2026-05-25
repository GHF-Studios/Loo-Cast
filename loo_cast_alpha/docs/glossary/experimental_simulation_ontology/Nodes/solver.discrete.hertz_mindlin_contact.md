---
canonical_name: Solver: Hertz-Mindlin Contact Resolution
status: WIP-experimental
aliases:
  - Hertz-Mindlin DEM Contact
source_of_truth: []
ontology_experimental: true
ontology_id: solver.discrete.hertz_mindlin_contact
node_class: solver
node_types:
  - solver
traits:
  - discontinuity_handling
projection_tags:
  - book:physical_scale
  - book:engineering_decision
activated_modules:
  - core.identity
  - type.solver
  - trait.discontinuity_handling
coordinate_annotations:
  S: granular_to_continuum_bridge
  M: contact_mechanics_discrete
  D: deterministic
  C: dissipative_collision_model
  K: particle_discrete
  L: near_neighbor_contacts
  R: path_dependent
  T: dynamic_contact_graph
  I: high_dimensional
  O: fully_observable_numeric_state
  G: coarse_grained_stress_fields
  Lambda: particle_size_to_continuum
  Sigma: jamming_sensitive
  Pi: particle
---

## Identity

- `display_name`: Contact-force resolution model for granular and DEM systems.
- `summary`: Computes normal/tangential contact forces with friction and damping for soft-particle dynamics.

## Activated Metadata Modules

### `type.solver`

- `method_family`: contact-force law evaluation with explicit time stepping.
- `order_of_accuracy`: integrator and contact regularization dependent.
- `stability_characteristics`: stiffness and overlap constraints impose time-step limits.
- `complexity_model`: neighbor-contact detection and pairwise contact force accumulation dominate.

### `trait.discontinuity_handling`

- `shock_sensor`: collision onset and overlap threshold detection.
- `limiter_family`: impulse/damping clipping and friction regularization.
- `entropy_fix_policy`: collision dissipation policy for numerical robustness.

## Declared Invariants

- Contact non-penetration consistency in regularized limit.

## Admissible Representations

- particle
- contact_graph

## Edge Ledger (machine-parseable)

```yaml
edges:
  - rel: eng:recommended_for
    to: paradigm.discrete.dem_granular
    direction: out
    confidence: 0.88
    when:
      granular_contact_dominant: true
```

#tech_glossary
#experimental_ontology
