---
canonical_name: Paradigm: Neuromechanical Rigid-Body Coupling
status: WIP-experimental
aliases: []
source_of_truth: []
ontology_experimental: true
ontology_id: paradigm.neuromech.rigid_body_coupling
node_class: paradigm
node_types:
  - neuromechanical_coupling_model
traits:
  - multi_scale
projection_tags:
  - book:physical_scale
activated_modules:
  - core.identity
  - trait.multi_scale
coordinate_annotations:
  S: biomechanical_to_cognitive
  Pi: graph_or_rigid_body
---

## Identity

- `display_name`: Coupled neural-control and rigid-body mechanics framework.
- `summary`: Sensorimotor simulation paradigm for embodied dynamics.

### `trait.multi_scale`

- `micro_scale`: neural activation and controller states.
- `macro_scale`: body-scale kinematics and dynamics.
- `bridge_operator`: actuation and feedback coupling maps.

## Edge Ledger (machine-parseable)

```yaml
edges: []
```

#tech_glossary
#experimental_ontology

