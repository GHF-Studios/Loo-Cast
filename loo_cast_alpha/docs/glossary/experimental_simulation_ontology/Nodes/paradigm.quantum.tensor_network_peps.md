---
canonical_name: Paradigm: Tensor Network PEPS
status: WIP-experimental
aliases:
  - Projected Entangled Pair States
source_of_truth: []
ontology_experimental: true
ontology_id: paradigm.quantum.tensor_network_peps
node_class: paradigm
node_types:
  - tensor_network_method
traits:
  - multi_scale
projection_tags:
  - book:physical_scale
activated_modules:
  - core.identity
  - trait.multi_scale
coordinate_annotations:
  S: quantum_many_body
  Pi: tensor_network
---

## Identity

- `display_name`: Higher-dimensional tensor-network state representation.
- `summary`: Entanglement-structured representation for lattice quantum systems beyond 1D.

### `trait.multi_scale`

- `micro_scale`: local tensor contractions.
- `macro_scale`: emergent many-body state amplitudes.
- `bridge_operator`: contraction approximations and bond truncation.

## Edge Ledger (machine-parseable)

```yaml
edges: []
```

#tech_glossary
#experimental_ontology

