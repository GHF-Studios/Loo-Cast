---
canonical_name: Paradigm: Relativistic Particle-in-Cell
status: WIP-experimental
aliases:
  - Relativistic PIC
source_of_truth: []
ontology_experimental: true
ontology_id: paradigm.quantum.relativistic_pic
node_class: paradigm
node_types:
  - plasma_particle_field_hybrid
traits:
  - multi_scale
projection_tags:
  - book:physical_scale
  - book:domain_panorama
activated_modules:
  - core.identity
  - trait.multi_scale
coordinate_annotations:
  S: quantum_to_plasma
  Pi: particle_and_grid
---

## Identity

- `display_name`: Relativistic particle-field coupling for plasma kinetics.
- `summary`: Particle push plus field solve architecture for high-energy plasma regimes.

### `trait.multi_scale`

- `micro_scale`: particle trajectories and local field updates.
- `macro_scale`: collective field structures and transport.
- `bridge_operator`: charge/current deposition and field interpolation.

## Edge Ledger (machine-parseable)

```yaml
edges: []
```

#tech_glossary
#experimental_ontology
