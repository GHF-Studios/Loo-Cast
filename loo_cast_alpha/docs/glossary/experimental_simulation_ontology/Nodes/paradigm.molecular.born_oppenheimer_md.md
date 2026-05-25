---
canonical_name: Paradigm: Born-Oppenheimer Molecular Dynamics
status: WIP-experimental
aliases:
  - BOMD
source_of_truth: []
ontology_experimental: true
ontology_id: paradigm.molecular.born_oppenheimer_md
node_class: paradigm
node_types:
  - ab_initio_molecular_dynamics
traits:
  - multi_scale
projection_tags:
  - book:physical_scale
activated_modules:
  - core.identity
  - trait.multi_scale
coordinate_annotations:
  S: molecular_quantum_hybrid
  Pi: basis_and_particle
---

## Identity

- `display_name`: Molecular dynamics on ground-state electronic surfaces.
- `summary`: Iterative electronic structure plus ionic integration paradigm.

### `trait.multi_scale`

- `micro_scale`: electronic ground-state solves.
- `macro_scale`: ionic trajectory evolution.
- `bridge_operator`: force extraction from electronic states.

## Edge Ledger (machine-parseable)

```yaml
edges: []
```

#tech_glossary
#experimental_ontology

