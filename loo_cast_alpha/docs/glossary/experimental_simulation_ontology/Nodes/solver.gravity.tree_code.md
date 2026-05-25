---
canonical_name: Solver: Hierarchical Tree-Code Gravity
status: WIP-experimental
aliases:
  - Barnes-Hut Tree Code
source_of_truth: []
ontology_experimental: true
ontology_id: solver.gravity.tree_code
node_class: solver
node_types:
  - solver
traits:
  - long_range
projection_tags:
  - book:physical_scale
  - book:engineering_decision
activated_modules:
  - core.identity
  - type.solver
  - trait.long_range
coordinate_annotations:
  S: astrophysical_to_cosmological
  M: hierarchical_nbody_approximation
  D: deterministic
  C: conservative_approximation
  K: particle_tree_hybrid
  L: long_range_clustered
  R: reversible_or_irreversible_model_dependent
  T: adaptive_tree_topology
  I: high_dimensional
  O: fully_observable_numeric_state
  G: far_field_coarse_graining
  Lambda: opening_angle_to_zero
  Sigma: approximation_parameter_sensitive
  Pi: tree
---

## Identity

- `display_name`: Hierarchical tree approximation for long-range particle interactions.
- `summary`: Scales better than all-pairs interactions by aggregating distant particle clusters.

## Activated Metadata Modules

### `type.solver`

- `method_family`: tree-based multipole or center-of-mass aggregation.
- `order_of_accuracy`: opening-angle and expansion policy dependent.
- `stability_characteristics`: force error controlled by acceptance criteria and softening choices.
- `complexity_model`: typically near `O(N log N)`.

### `trait.long_range`

- `accelerator_family`: hierarchical near/far decomposition.
- `error_control`: opening-angle and expansion-order control.
- `far_field_model`: aggregated cluster multipole approximations.

## Declared Invariants

- Controlled force-approximation envelope under configured tree criteria.

## Admissible Representations

- particle
- tree

## Edge Ledger (machine-parseable)

```yaml
edges:
  - rel: eng:recommended_for
    to: paradigm.cosmo.particle_mesh_nbody
    direction: out
    confidence: 0.85
    when:
      nonuniform_particle_distribution: true
  - rel: eng:recommended_for
    to: paradigm.particle.nbody_dynamics
    direction: out
    confidence: 0.79
    when:
      long_range_interaction: true
```

#tech_glossary
#experimental_ontology
