---
canonical_name: Paradigm: Tensor Network MPS
status: WIP-experimental
aliases:
  - Matrix Product States
source_of_truth: []
ontology_experimental: true
ontology_id: paradigm.quantum.tensor_network_mps
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

- `display_name`: One-dimensional tensor-network state representation.
- `summary`: Compressed quantum many-body representation with controllable bond dimension.

### `trait.multi_scale`

- `micro_scale`: local tensor factors.
- `macro_scale`: entanglement-structured global state.
- `bridge_operator`: bond-dimension truncation and renormalization sweeps.

## Edge Ledger (machine-parseable)

```yaml
edges: []
```

#tech_glossary
#experimental_ontology

