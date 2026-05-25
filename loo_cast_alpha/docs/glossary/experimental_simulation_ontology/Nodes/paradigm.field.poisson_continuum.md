---
canonical_name: Paradigm: Field Poisson Continuum Model
status: WIP-experimental
aliases:
  - Poisson Field Formulation
source_of_truth: []
ontology_experimental: true
ontology_id: paradigm.field.poisson_continuum
node_class: paradigm
node_types:
  - PDE
  - elliptic_field_equation
traits:
  - nonlocal
projection_tags:
  - book:physical_scale
  - book:mathematical_structure
  - book:transform_morphism
activated_modules:
  - core.identity
  - type.PDE
  - trait.nonlocal
coordinate_annotations:
  S: mesoscopic_to_cosmological
  M: PDE
  D: deterministic
  C: conservative_potential_form
  K: continuous
  L: nonlocal_interpretation
  R: reversible
  T: fixed_or_dynamic_domain
  I: high_dimensional
  O: partially_observable
  G: mean_field
  Lambda: continuum_limit
  Sigma: linear_or_weakly_nonlinear
  Pi: grid_or_spectral
---

## Identity

- `display_name`: Continuum potential-field representation of interaction structure.
- `summary`: Maps source distributions to potential fields, often dual to particle formulations.

## Activated Metadata Modules

### `type.PDE`

- `boundary_conditions`: Dirichlet, Neumann, mixed, periodic.
- `weak_form`: Laplacian bilinear form with source term.
- `flux_form`: gradient-based flux from scalar potential.
- `discretization_options`: finite element, finite difference, multigrid, spectral.

### `trait.nonlocal`

- `interaction_kernel`: Green's function kernel interpretation.
- `support_radius`: global support in unbounded domains.
- `decay_behavior`: dimension-dependent algebraic decay.

## Declared Invariants

- Conservation encoded through source/flux balance constraints.

## Admissible Representations

- grid
- finite_element_mesh
- spectral

## Edge Ledger (machine-parseable)

```yaml
edges:
  - rel: math:duality_instance
    to: morphism.duality.particle_field
    direction: out
    confidence: 0.8
  - rel: math:linked_limit_process
    to: transform.limit.discrete_to_continuous
    direction: out
    confidence: 0.7
```

## Related Nodes

- [morphism.duality.particle_field](morphism.duality.particle_field.md)
- [transform.limit.discrete_to_continuous](transform.limit.discrete_to_continuous.md)

#tech_glossary
#experimental_ontology
