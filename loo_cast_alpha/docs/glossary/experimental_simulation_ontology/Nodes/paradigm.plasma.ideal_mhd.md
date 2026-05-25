---
canonical_name: Paradigm: Ideal Magnetohydrodynamics
status: WIP-experimental
aliases:
  - Ideal MHD
source_of_truth: []
ontology_experimental: true
ontology_id: paradigm.plasma.ideal_mhd
node_class: paradigm
node_types:
  - plasma_mhd_model
traits:
  - structure_preserving
projection_tags:
  - book:physical_scale
activated_modules:
  - core.identity
  - trait.structure_preserving
coordinate_annotations:
  S: plasma_astrophysical
  Pi: grid
---

## Identity

- `display_name`: Conducting-fluid electromagnetic coupling without resistive dissipation.
- `summary`: Conservative plasma-fluid paradigm with frozen-in magnetic-field behavior.

### `trait.structure_preserving`

- `preserved_invariants`: ideal invariants under compatible discretizations.
- `monitor_strategy`: divergence and conserved-variable residual checks.

## Edge Ledger (machine-parseable)

```yaml
edges: []
```

#tech_glossary
#experimental_ontology

