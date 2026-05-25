---
canonical_name: Paradigm: Non-Fourier Heat Conduction
status: WIP-experimental
aliases: []
source_of_truth: []
ontology_experimental: true
ontology_id: paradigm.thermal.non_fourier_heat
node_class: paradigm
node_types:
  - nonlocal_thermal_model
traits:
  - nonlocal
projection_tags:
  - book:physical_scale
activated_modules:
  - core.identity
  - trait.nonlocal
coordinate_annotations:
  S: mesoscale_thermal
  Pi: grid
---

## Identity

- `display_name`: Heat transport paradigms with memory or finite-speed propagation effects.
- `summary`: Extends classical conduction models for ultrafast and micro/nanoscale regimes.

### `trait.nonlocal`

- `interaction_kernel`: memory and spatial nonlocal kernels.
- `support_radius`: finite to long-range depending on closure.
- `decay_behavior`: model-dependent temporal/spatial decay.

## Edge Ledger (machine-parseable)

```yaml
edges: []
```

#tech_glossary
#experimental_ontology

