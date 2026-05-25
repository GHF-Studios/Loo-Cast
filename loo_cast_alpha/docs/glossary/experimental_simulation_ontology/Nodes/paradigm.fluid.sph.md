---
canonical_name: Paradigm: Smoothed-Particle Hydrodynamics
status: WIP-experimental
aliases:
  - SPH
source_of_truth: []
ontology_experimental: true
ontology_id: paradigm.fluid.sph
node_class: paradigm
node_types:
  - meshless_fluid_model
traits:
  - nonlocal
projection_tags:
  - book:physical_scale
activated_modules:
  - core.identity
  - trait.nonlocal
coordinate_annotations:
  S: continuum_to_particle_fluid
  Pi: particle
---

## Identity

- `display_name`: Meshless particle fluid paradigm with kernel interactions.
- `summary`: Particle representation for free-surface and large-deformation fluid dynamics.

### `trait.nonlocal`

- `interaction_kernel`: smoothing kernels over neighbor clouds.
- `support_radius`: finite smoothing-length neighborhood.
- `decay_behavior`: compact-support kernel truncation.

## Edge Ledger (machine-parseable)

```yaml
edges: []
```

#tech_glossary
#experimental_ontology

