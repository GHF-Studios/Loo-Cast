---
canonical_name: Transform: Parallel-in-Time Parareal
status: WIP-experimental
aliases:
  - Parareal
source_of_truth: []
ontology_experimental: true
ontology_id: transform.time.parallel_in_time_parareal
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

- `display_name`: Coarse-fine time-domain decomposition transform.
- `summary`: Iteratively maps serial temporal propagation into parallel corrected blocks.

### `type.transform`

- `domain_map`: serial time integration trajectory.
- `codomain_map`: block-decomposed parallel time trajectory.
- `invertibility`: not exact; iterative correction transform.
- `preserved_quantities`: convergence to reference fine solution under assumptions.

### `trait.multi_scale`

- `micro_scale`: fine propagator corrections.
- `macro_scale`: coarse temporal prediction.
- `bridge_operator`: correction iteration updates.

## Edge Ledger (machine-parseable)

```yaml
edges: []
```

#tech_glossary
#experimental_ontology

