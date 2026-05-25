---
canonical_name: Paradigm: DLVO Interfacial Interaction Theory
status: WIP-experimental
aliases:
  - DLVO
source_of_truth: []
ontology_experimental: true
ontology_id: paradigm.interface.dlvo
node_class: paradigm
node_types:
  - interfacial_interaction_model
traits:
  - nonlocal
projection_tags:
  - book:physical_scale
activated_modules:
  - core.identity
  - trait.nonlocal
coordinate_annotations:
  S: colloidal_interface
  Pi: particle_or_field
---

## Identity

- `display_name`: Colloidal interaction paradigm combining electrostatic and van der Waals effects.
- `summary`: Effective-potential framework for wetting, aggregation, and stability phenomena.

### `trait.nonlocal`

- `interaction_kernel`: electrostatic and dispersion potential kernels.
- `support_radius`: finite-range screened and long-range contributions.
- `decay_behavior`: exponential plus algebraic components.

## Edge Ledger (machine-parseable)

```yaml
edges: []
```

#tech_glossary
#experimental_ontology

