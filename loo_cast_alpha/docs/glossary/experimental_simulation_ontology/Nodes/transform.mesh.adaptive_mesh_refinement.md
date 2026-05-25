---
canonical_name: Transform: Adaptive Mesh Refinement
status: WIP-experimental
aliases:
  - AMR
source_of_truth: []
ontology_experimental: true
ontology_id: transform.mesh.adaptive_mesh_refinement
node_class: transform
node_types:
  - transform
  - mesh_refinement
traits:
  - multi_scale
projection_tags:
  - book:transform_morphism
activated_modules:
  - core.identity
  - type.transform
  - trait.multi_scale
coordinate_annotations:
  S: cross_scale
  Pi: grid
---

## Identity

- `display_name`: Resolution-adaptive spatial discretization transform.
- `summary`: Dynamically maps coarse mesh regions to refined local representations.

### `type.transform`

- `domain_map`: coarse-grid state fields.
- `codomain_map`: nested multi-resolution mesh fields.
- `invertibility`: approximate with projection/restriction operators.
- `preserved_quantities`: conservative balances under refluxing/consistent transfer.

### `trait.multi_scale`

- `micro_scale`: fine-grid local transients.
- `macro_scale`: coarse-grid global flow.
- `bridge_operator`: prolongation/restriction and flux correction.

## Edge Ledger (machine-parseable)

```yaml
edges: []
```

#tech_glossary
#experimental_ontology

