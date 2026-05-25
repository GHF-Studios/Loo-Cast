---
canonical_name: Transform: Balanced-Truncation State-Space Reduction
status: WIP-experimental
aliases:
  - Balanced Truncation
source_of_truth: []
ontology_experimental: true
ontology_id: transform.reduction.state_space_balanced_truncation
node_class: transform
node_types:
  - transform
  - model_reduction
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
  Pi: vector_or_matrix
---

## Identity

- `display_name`: Controllability-observability balanced reduced-model transform.
- `summary`: Maps high-order linear dynamical systems to low-order surrogates with error bounds.

### `type.transform`

- `domain_map`: full-order state-space realization.
- `codomain_map`: reduced balanced state-space realization.
- `invertibility`: non-invertible due truncation.
- `preserved_quantities`: dominant Hankel-energy modes.

### `trait.multi_scale`

- `micro_scale`: weakly controllable/observable modes.
- `macro_scale`: dominant energetic modes.
- `bridge_operator`: balancing and truncation mappings.

## Edge Ledger (machine-parseable)

```yaml
edges: []
```

#tech_glossary
#experimental_ontology

