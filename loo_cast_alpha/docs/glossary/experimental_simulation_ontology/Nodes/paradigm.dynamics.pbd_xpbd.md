---
canonical_name: Paradigm: Position-Based Dynamics / XPBD
status: WIP-experimental
aliases:
  - PBD XPBD
source_of_truth: []
ontology_experimental: true
ontology_id: paradigm.dynamics.pbd_xpbd
node_class: paradigm
node_types:
  - constraint_projection_dynamics
traits:
  - structure_preserving
projection_tags:
  - book:physical_scale
activated_modules:
  - core.identity
  - trait.structure_preserving
coordinate_annotations:
  S: mechanical_system
  Pi: particle_or_graph
---

## Identity

- `display_name`: Constraint-projection dynamics paradigm for stable interactive mechanics.
- `summary`: Compliance-aware projection framework for cloth, soft bodies, and articulated constraints.

### `trait.structure_preserving`

- `preserved_invariants`: constraint satisfaction envelopes.
- `monitor_strategy`: constraint residual and drift diagnostics.

## Edge Ledger (machine-parseable)

```yaml
edges: []
```

#tech_glossary
#experimental_ontology

