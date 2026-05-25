---
canonical_name: Paradigm: Resistive Magnetohydrodynamics
status: WIP-experimental
aliases:
  - Resistive MHD
source_of_truth: []
ontology_experimental: true
ontology_id: paradigm.plasma.resistive_mhd
node_class: paradigm
node_types:
  - plasma_mhd_model
traits:
  - discontinuity_handling
projection_tags:
  - book:physical_scale
activated_modules:
  - core.identity
  - trait.discontinuity_handling
coordinate_annotations:
  S: plasma_astrophysical
  Pi: grid
---

## Identity

- `display_name`: Magnetofluid paradigm including finite resistivity and reconnection physics.
- `summary`: Dissipative plasma-fluid framework for non-ideal electromagnetic dynamics.

### `trait.discontinuity_handling`

- `shock_sensor`: current-sheet and shock diagnostics.
- `limiter_family`: MHD limiter/reconstruction families.
- `entropy_fix_policy`: dissipation-consistent non-ideal flux policies.

## Edge Ledger (machine-parseable)

```yaml
edges: []
```

#tech_glossary
#experimental_ontology

