---
canonical_name: Paradigm: Incompressible Navier-Stokes
status: WIP-experimental
aliases:
  - INS
source_of_truth: []
ontology_experimental: true
ontology_id: paradigm.pde.incompressible_navier_stokes
node_class: paradigm
node_types:
  - PDE
  - continuum_mechanics
traits:
  - multi_scale
  - structure_preserving
projection_tags:
  - book:physical_scale
  - book:mathematical_structure
  - book:engineering_decision
  - book:computational_representation
activated_modules:
  - core.identity
  - type.PDE
  - trait.multi_scale
  - trait.structure_preserving
coordinate_annotations:
  S: continuum
  M: PDE
  D: deterministic
  C: weakly_dissipative
  K: continuous
  L: local
  R: irreversible
  T: fixed_or_moving_domain
  I: high_dimensional
  O: partially_observable
  G: statistical_or_renormalized
  Lambda: low_Mach_limit
  Sigma: nonlinear_or_turbulent
  Pi: grid_or_spectral
---

## Identity

- `display_name`: Incompressible Navier-Stokes system.
- `summary`: Continuum PDE model for viscous incompressible flow with divergence-free velocity.

## Activated Metadata Modules

### `type.PDE`

- `boundary_conditions`: no-slip, slip, inflow/outflow, periodic.
- `weak_form`: Galerkin weak form over divergence-free test spaces.
- `flux_form`: advective momentum flux plus viscous stress divergence.
- `discretization_options`: finite volume, finite element, spectral, staggered-grid finite difference.

### `trait.multi_scale`

- `micro_scale`: Kolmogorov-scale eddies and near-wall layers.
- `macro_scale`: domain-scale transport and coherent structures.
- `bridge_operator`: LES/RANS closure or filtered equation mapping.

### `trait.structure_preserving`

- `preserved_invariants`: mass conservation, bounded kinetic-energy drift under stable discretization.
- `monitor_strategy`: divergence norm and global mass-balance residual tracking.

## Declared Invariants

- Divergence-free constraint.
- Global mass conservation.

## Admissible Representations

- grid
- spectral
- mixed finite element

## Edge Ledger (machine-parseable)

```yaml
edges:
  - rel: eng:recommended_solver
    to: solver.time.imex_bdf2
    direction: out
    confidence: 0.84
    when:
      stiffness: high
      multiple_time_scales: true
  - rel: math:coarse_grained_by
    to: transform.coarse_graining.ensemble_map
    direction: out
    confidence: 0.78
  - rel: math:linked_limit_process
    to: transform.limit.discrete_to_continuous
    direction: out
    confidence: 0.73
```

## Related Nodes

- [solver.time.imex_bdf2](solver.time.imex_bdf2.md)
- [transform.coarse_graining.ensemble_map](transform.coarse_graining.ensemble_map.md)
- [transform.limit.discrete_to_continuous](transform.limit.discrete_to_continuous.md)

#tech_glossary
#experimental_ontology
