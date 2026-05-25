---
canonical_name: Paradigm: Kohn-Sham Density Functional Theory
status: WIP-experimental
aliases:
  - DFT Kohn-Sham
source_of_truth: []
ontology_experimental: true
ontology_id: paradigm.quantum.dft_kohn_sham
node_class: paradigm
node_types:
  - electronic_structure_theory
traits:
  - variational
projection_tags:
  - book:physical_scale
  - book:domain_panorama
activated_modules:
  - core.identity
  - trait.variational
coordinate_annotations:
  S: quantum_electronic
  Pi: basis_or_grid
---

## Identity

- `display_name`: Electronic structure via density-functional fixed-point models.
- `summary`: Mean-field quantum framework for atomistic and condensed-matter simulations.

### `trait.variational`

- `functional`: exchange-correlation augmented energy functional.
- `admissible_space`: density and orbital function spaces.
- `stationary_condition`: self-consistent stationarity of Kohn-Sham equations.

## Edge Ledger (machine-parseable)

```yaml
edges: []
```

#tech_glossary
#experimental_ontology

