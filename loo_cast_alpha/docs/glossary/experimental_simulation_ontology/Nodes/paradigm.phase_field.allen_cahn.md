---
canonical_name: Paradigm: Allen-Cahn Phase-Field
status: WIP-experimental
aliases: []
source_of_truth: []
ontology_experimental: true
ontology_id: paradigm.phase_field.allen_cahn
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

- `display_name`: Nonconserved phase-field evolution with interface motion.
- `summary`: Relaxational dynamics for order-parameter transitions and interface kinetics.

### `type.PDE`

- `boundary_conditions`: Neumann, Dirichlet, periodic.
- `weak_form`: reaction-diffusion weak form for order parameter.
- `flux_form`: gradient and potential-driven interface motion terms.
- `discretization_options`: finite difference, finite element, finite volume.

### `trait.variational`

- `functional`: double-well free-energy functional.
- `admissible_space`: order-parameter function spaces.
- `stationary_condition`: Euler-Lagrange equilibrium profiles.

## Edge Ledger (machine-parseable)

```yaml
edges: []
```

#tech_glossary
#experimental_ontology

