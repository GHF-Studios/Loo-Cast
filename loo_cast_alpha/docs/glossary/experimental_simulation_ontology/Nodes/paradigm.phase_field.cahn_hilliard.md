---
canonical_name: Paradigm: Cahn-Hilliard Phase-Field
status: WIP-experimental
aliases: []
source_of_truth: []
ontology_experimental: true
ontology_id: paradigm.phase_field.cahn_hilliard
node_class: paradigm
node_types:
  - PDE
traits:
  - variational
projection_tags:
  - book:physical_scale
activated_modules:
  - core.identity
  - type.PDE
  - trait.variational
coordinate_annotations:
  S: mesoscale_material
  Pi: grid
---

## Identity

- `display_name`: Conserved phase-field evolution for demixing and spinodal dynamics.
- `summary`: Fourth-order gradient-flow-like phase-separation paradigm.

### `type.PDE`

- `boundary_conditions`: no-flux and periodic common forms.
- `weak_form`: mixed weak formulations for chemical potential coupling.
- `flux_form`: conservative mass flux driven by chemical potential gradients.
- `discretization_options`: mixed FEM, finite difference, finite volume.

### `trait.variational`

- `functional`: Ginzburg-Landau free energy.
- `admissible_space`: conserved-order-parameter function spaces.
- `stationary_condition`: chemical-potential equilibrium conditions.

## Edge Ledger (machine-parseable)

```yaml
edges: []
```

#tech_glossary
#experimental_ontology

