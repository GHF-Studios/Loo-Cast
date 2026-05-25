---
canonical_name: Paradigm: Reactive Molecular Dynamics (ReaxFF)
status: WIP-experimental
aliases:
  - ReaxFF Dynamics
source_of_truth: []
ontology_experimental: true
ontology_id: paradigm.molecular.reactive_reaxff
node_class: paradigm
node_types:
  - reactive_molecular_dynamics
traits:
  - multi_scale
projection_tags:
  - book:physical_scale
activated_modules:
  - core.identity
  - trait.multi_scale
coordinate_annotations:
  S: molecular_nanoscale
  Pi: particle
---

## Identity

- `display_name`: Bond-forming/breaking molecular dynamics with reactive force fields.
- `summary`: Reactive atomistic framework bridging chemistry and materials dynamics.

### `trait.multi_scale`

- `micro_scale`: bond-level events and local reactions.
- `macro_scale`: emergent transport and material response.
- `bridge_operator`: reactive force updates and coarse observables.

## Edge Ledger (machine-parseable)

```yaml
edges: []
```

#tech_glossary
#experimental_ontology

