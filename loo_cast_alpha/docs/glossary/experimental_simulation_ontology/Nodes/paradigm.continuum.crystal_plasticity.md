---
canonical_name: Paradigm: Crystal Plasticity
status: WIP-experimental
aliases: []
source_of_truth: []
ontology_experimental: true
ontology_id: paradigm.continuum.crystal_plasticity
node_class: paradigm
node_types:
  - constitutive_plasticity_model
traits:
  - multi_scale
projection_tags:
  - book:physical_scale
activated_modules:
  - core.identity
  - trait.multi_scale
coordinate_annotations:
  S: micro_to_continuum_structural
  Pi: mesh
---

## Identity

- `display_name`: Slip-system-based anisotropic plastic constitutive paradigm.
- `summary`: Microstructure-aware deformation framework linking crystallography to macro response.

### `trait.multi_scale`

- `micro_scale`: grain/slip-system activation.
- `macro_scale`: effective stress-strain response.
- `bridge_operator`: homogenization from crystal orientation ensembles.

## Edge Ledger (machine-parseable)

```yaml
edges: []
```

#tech_glossary
#experimental_ontology

