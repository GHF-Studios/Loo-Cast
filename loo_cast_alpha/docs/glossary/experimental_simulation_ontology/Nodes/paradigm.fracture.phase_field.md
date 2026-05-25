---
canonical_name: Paradigm: Phase-Field Fracture
status: WIP-experimental
aliases: []
source_of_truth: []
ontology_experimental: true
ontology_id: paradigm.fracture.phase_field
node_class: paradigm
node_types:
  - fracture_mechanics_model
traits:
  - variational
projection_tags:
  - book:physical_scale
activated_modules:
  - core.identity
  - trait.variational
coordinate_annotations:
  S: continuum_structural
  Pi: mesh
---

## Identity

- `display_name`: Diffuse-crack fracture framework via auxiliary damage field.
- `summary`: Variational fracture paradigm supporting complex crack topologies.

### `trait.variational`

- `functional`: elastic plus fracture regularization energy.
- `admissible_space`: displacement and damage function spaces.
- `stationary_condition`: coupled Euler-Lagrange fracture conditions.

## Edge Ledger (machine-parseable)

```yaml
edges: []
```

#tech_glossary
#experimental_ontology

