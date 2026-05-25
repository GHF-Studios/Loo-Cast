---
canonical_name: Transform: Discrete-to-Continuous Limit
status: WIP-experimental
aliases:
  - Discrete Continuous Limit
source_of_truth: []
ontology_experimental: true
ontology_id: transform.limit.discrete_to_continuous
node_class: transform
node_types:
  - transform
  - limiting_process
traits: []
projection_tags:
  - book:transform_morphism
  - book:mathematical_structure
  - book:computational_representation
activated_modules:
  - core.identity
  - type.transform
coordinate_annotations:
  S: cross_scale
  M: asymptotic_analysis
  D: deterministic_or_ensemble
  C: model_dependent
  K: discrete_to_continuous
  L: local_or_nonlocal_limit
  R: reversible_or_irreversible
  T: topology_limit_sensitive
  I: dimension_lift_or_reduction
  O: model_dependent
  G: renormalized_or_mean_field
  Lambda: grid_spacing_to_zero_or_N_to_infinity
  Sigma: stable_or_singular_limit
  Pi: any
---

## Identity

- `display_name`: Asymptotic map from discrete models to continuum equations.
- `summary`: Captures limit processes such as `dx -> 0`, `N -> infinity`, and weak convergence constructions.

## Activated Metadata Modules

### `type.transform`

- `domain_map`: discrete states, particles, or lattice configurations.
- `codomain_map`: continuum field or measure-valued description.
- `invertibility`: generally non-invertible in practice.
- `preserved_quantities`: selected conserved quantities under consistent scaling.

## Declared Invariants

- Consistency with limiting conservation statements under stated assumptions.

## Admissible Representations

- grid
- particle
- measure
- field

## Edge Ledger (machine-parseable)

```yaml
edges:
  - rel: math:connects
    to: paradigm.pde.incompressible_navier_stokes
    direction: out
    confidence: 0.73
  - rel: math:connects
    to: paradigm.pde.compressible_flow_with_shocks
    direction: out
    confidence: 0.75
  - rel: math:connects
    to: paradigm.field.poisson_continuum
    direction: out
    confidence: 0.7
```

## Related Nodes

- [paradigm.pde.incompressible_navier_stokes](paradigm.pde.incompressible_navier_stokes.md)
- [paradigm.pde.compressible_flow_with_shocks](paradigm.pde.compressible_flow_with_shocks.md)
- [paradigm.field.poisson_continuum](paradigm.field.poisson_continuum.md)

#tech_glossary
#experimental_ontology
