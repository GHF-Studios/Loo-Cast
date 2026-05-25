---
canonical_name: Paradigm: Tensor Network MERA
status: WIP-experimental
aliases:
  - Multiscale Entanglement Renormalization Ansatz
source_of_truth: []
ontology_experimental: true
ontology_id: paradigm.quantum.tensor_network_mera
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

- `display_name`: Hierarchical tensor-network renormalization framework.
- `summary`: Explicit scale-separating entanglement representation for critical many-body systems.

### `trait.multi_scale`

- `micro_scale`: local disentanglers and isometries.
- `macro_scale`: coarse renormalized state structure.
- `bridge_operator`: layerwise coarse-graining transformations.

## Edge Ledger (machine-parseable)

```yaml
edges: []
```

#tech_glossary
#experimental_ontology

