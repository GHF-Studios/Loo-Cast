---
canonical_name: Transform: Multirate Subcycling
status: WIP-experimental
aliases: []
source_of_truth: []
ontology_experimental: true
ontology_id: transform.time.multirate_subcycling
node_class: transform
node_types:
  - transform
  - time_refinement
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
  Pi: any
---

## Identity

- `display_name`: Time-resolution transform with zone-dependent step sizes.
- `summary`: Maps single-rate updates into synchronized multi-rate substep execution.

### `type.transform`

- `domain_map`: single-rate temporal evolution.
- `codomain_map`: nested fast/slow update schedule.
- `invertibility`: not strictly invertible.
- `preserved_quantities`: targeted conservative subcycle aggregates.

### `trait.multi_scale`

- `micro_scale`: fast substep dynamics.
- `macro_scale`: slow-step evolution.
- `bridge_operator`: synchronization and conservative correction maps.

## Edge Ledger (machine-parseable)

```yaml
edges: []
```

#tech_glossary
#experimental_ontology

