---
canonical_name: Paradigm: Vortex Particle Methods
status: WIP-experimental
aliases: []
source_of_truth: []
ontology_experimental: true
ontology_id: paradigm.fluid.vortex_particles
node_class: paradigm
node_types:
  - vorticity_particle_model
traits:
  - nonlocal
projection_tags:
  - book:physical_scale
activated_modules:
  - core.identity
  - trait.nonlocal
coordinate_annotations:
  S: continuum_fluid
  Pi: particle
---

## Identity

- `display_name`: Vorticity-carrying particle framework for incompressible flow.
- `summary`: Particle-vorticity paradigm with Biot-Savart induced velocity coupling.

### `trait.nonlocal`

- `interaction_kernel`: Biot-Savart-like velocity kernels.
- `support_radius`: global or truncated long-range support.
- `decay_behavior`: algebraic decay with regularization.

## Edge Ledger (machine-parseable)

```yaml
edges: []
```

#tech_glossary
#experimental_ontology

