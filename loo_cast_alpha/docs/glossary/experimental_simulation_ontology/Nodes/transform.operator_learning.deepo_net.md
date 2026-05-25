---
canonical_name: Transform: DeepONet Operator Learning
status: WIP-experimental
aliases:
  - DeepONet
source_of_truth: []
ontology_experimental: true
ontology_id: transform.operator_learning.deepo_net
node_class: transform
node_types:
  - transform
  - operator_learning
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
  Pi: learned_operator
---

## Identity

- `display_name`: Branch-trunk neural operator mapping transform.
- `summary`: Data-driven function-to-function transform for parametric solution operators.

### `type.transform`

- `domain_map`: input function samples and parameter encodings.
- `codomain_map`: output function evaluations.
- `invertibility`: generally non-invertible.
- `preserved_quantities`: model-dependent soft or hard constraints.

### `trait.multi_scale`

- `micro_scale`: local function-sample behavior.
- `macro_scale`: global operator response.
- `bridge_operator`: learned basis composition across branch/trunk outputs.

## Edge Ledger (machine-parseable)

```yaml
edges: []
```

#tech_glossary
#experimental_ontology

