---
canonical_name: Paradigm: Cohesive-Zone Fracture
status: WIP-experimental
aliases: []
source_of_truth: []
ontology_experimental: true
ontology_id: paradigm.fracture.cohesive_zone
node_class: paradigm
node_types:
  - fracture_mechanics_model
traits:
  - discontinuity_handling
projection_tags:
  - book:physical_scale
activated_modules:
  - core.identity
  - trait.discontinuity_handling
coordinate_annotations:
  S: continuum_structural
  Pi: mesh
---

## Identity

- `display_name`: Interface traction-separation fracture framework.
- `summary`: Crack initiation/propagation paradigm with embedded interface laws.

### `trait.discontinuity_handling`

- `shock_sensor`: crack nucleation and interface-separation criteria.
- `limiter_family`: traction regularization and damage clipping.
- `entropy_fix_policy`: dissipation-consistent failure progression policy.

## Edge Ledger (machine-parseable)

```yaml
edges: []
```

#tech_glossary
#experimental_ontology

