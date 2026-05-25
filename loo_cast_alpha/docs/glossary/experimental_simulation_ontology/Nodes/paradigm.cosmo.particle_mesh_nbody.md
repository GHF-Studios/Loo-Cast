---
canonical_name: Paradigm: Cosmological Particle-Mesh N-Body
status: WIP-experimental
aliases:
  - PM N-Body
source_of_truth: []
ontology_experimental: true
ontology_id: paradigm.cosmo.particle_mesh_nbody
node_class: paradigm
node_types:
  - cosmological_nbody_model
traits:
  - long_range
projection_tags:
  - book:physical_scale
activated_modules:
  - core.identity
  - trait.long_range
coordinate_annotations:
  S: cosmological
  Pi: particle_mesh_hybrid
---

## Identity

- `display_name`: Large-scale structure dynamics with particle-mesh gravity coupling.
- `summary`: Cosmology-scale gravitating-matter paradigm using mesh long-range forces.

### `trait.long_range`

- `accelerator_family`: PM/P3M/tree long-range accelerators.
- `error_control`: force-split and assignment-kernel diagnostics.
- `far_field_model`: mesh-based far-field gravitational potential.

## Edge Ledger (machine-parseable)

```yaml
edges: []
```

#tech_glossary
#experimental_ontology

