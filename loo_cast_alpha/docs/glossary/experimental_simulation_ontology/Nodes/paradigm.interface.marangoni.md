---
canonical_name: Paradigm: Marangoni Convection
status: WIP-experimental
aliases: []
source_of_truth: []
ontology_experimental: true
ontology_id: paradigm.interface.marangoni
node_class: paradigm
node_types:
  - interfacial_mechanics_model
traits:
  - multi_scale
projection_tags:
  - book:physical_scale
activated_modules:
  - core.identity
  - trait.multi_scale
coordinate_annotations:
  S: continuum_interface
  Pi: grid_or_mesh
---

## Identity

- `display_name`: Surface-tension-gradient-driven interfacial flow paradigm.
- `summary`: Thermo/solutal-capillary coupling framework at fluid interfaces.

### `trait.multi_scale`

- `micro_scale`: local interfacial tension gradients.
- `macro_scale`: bulk convective structures.
- `bridge_operator`: boundary stress coupling operators.

## Edge Ledger (machine-parseable)

```yaml
edges: []
```

#tech_glossary
#experimental_ontology

