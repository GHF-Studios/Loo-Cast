---
canonical_name: Transform: Parallel-in-Time MGRIT
status: WIP-experimental
aliases:
  - MGRIT
source_of_truth: []
ontology_experimental: true
ontology_id: transform.time.parallel_in_time_mgrit
node_class: transform
node_types:
  - transform
  - parallel_time_decomposition
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
  Pi: distributed_compute
---

## Identity

- `display_name`: Multigrid reduction-in-time transform.
- `summary`: Temporal multigrid transform for scalable parallel time integration.

### `type.transform`

- `domain_map`: fine temporal grid trajectory.
- `codomain_map`: multilevel coarse-to-fine temporal hierarchy.
- `invertibility`: approximate through multilevel correction.
- `preserved_quantities`: converged trajectory equivalence under solver assumptions.

### `trait.multi_scale`

- `micro_scale`: fine-time error components.
- `macro_scale`: coarse-time smooth components.
- `bridge_operator`: restriction/prolongation in time.

## Edge Ledger (machine-parseable)

```yaml
edges: []
```

#tech_glossary
#experimental_ontology

