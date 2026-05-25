---
canonical_name: Paradigm: Phonon Boltzmann Transport
status: WIP-experimental
aliases:
  - Phonon BTE
source_of_truth: []
ontology_experimental: true
ontology_id: paradigm.thermal.phonon_bte
node_class: paradigm
node_types:
  - kinetic_transport_model
traits:
  - multi_scale
projection_tags:
  - book:physical_scale
activated_modules:
  - core.identity
  - trait.multi_scale
coordinate_annotations:
  S: nanoscale_thermal
  Pi: monte_carlo_or_grid
---

## Identity

- `display_name`: Kinetic thermal transport via phonon distribution dynamics.
- `summary`: Non-equilibrium heat transport framework beyond classical Fourier closure.

### `trait.multi_scale`

- `micro_scale`: mode-level scattering events.
- `macro_scale`: effective thermal flux fields.
- `bridge_operator`: moment closures and kinetic averaging.

## Edge Ledger (machine-parseable)

```yaml
edges: []
```

#tech_glossary
#experimental_ontology

