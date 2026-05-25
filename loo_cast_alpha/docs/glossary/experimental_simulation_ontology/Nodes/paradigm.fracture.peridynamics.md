---
canonical_name: Paradigm: Peridynamics
status: WIP-experimental
aliases: []
source_of_truth: []
ontology_experimental: true
ontology_id: paradigm.fracture.peridynamics
node_class: paradigm
node_types:
  - nonlocal_continuum_model
traits:
  - nonlocal
projection_tags:
  - book:physical_scale
activated_modules:
  - core.identity
  - trait.nonlocal
coordinate_annotations:
  S: continuum_structural
  Pi: particle_or_meshless
---

## Identity

- `display_name`: Nonlocal continuum fracture and deformation framework.
- `summary`: Integral-interaction mechanics paradigm that naturally accommodates discontinuities.

### `trait.nonlocal`

- `interaction_kernel`: horizon-based pairwise force kernel.
- `support_radius`: finite interaction horizon.
- `decay_behavior`: compact-support kernel behavior.

## Edge Ledger (machine-parseable)

```yaml
edges: []
```

#tech_glossary
#experimental_ontology

