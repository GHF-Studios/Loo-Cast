---
canonical_name: Paradigm: Car-Parrinello Molecular Dynamics
status: WIP-experimental
aliases:
  - CPMD
source_of_truth: []
ontology_experimental: true
ontology_id: paradigm.molecular.car_parrinello_md
node_class: paradigm
node_types:
  - ab_initio_molecular_dynamics
traits:
  - variational
projection_tags:
  - book:physical_scale
activated_modules:
  - core.identity
  - trait.variational
coordinate_annotations:
  S: molecular_quantum_hybrid
  Pi: basis_and_particle
---

## Identity

- `display_name`: Coupled ionic-electronic ab initio dynamics.
- `summary`: Extended-Lagrangian framework for quantum-informed molecular motion.

### `trait.variational`

- `functional`: coupled ionic-electronic Lagrangian.
- `admissible_space`: orbital and ionic configuration spaces.
- `stationary_condition`: constrained variational dynamics of combined system.

## Edge Ledger (machine-parseable)

```yaml
edges: []
```

#tech_glossary
#experimental_ontology

