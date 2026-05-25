---
canonical_name: Paradigm: General Relativistic Magnetohydrodynamics
status: WIP-experimental
aliases:
  - GRMHD
source_of_truth: []
ontology_experimental: true
ontology_id: paradigm.astro.grmhd
node_class: paradigm
node_types:
  - PDE
traits:
  - discontinuity_handling
  - long_range
projection_tags:
  - book:physical_scale
  - book:mathematical_structure
  - book:engineering_decision
activated_modules:
  - core.identity
  - type.PDE
  - trait.discontinuity_handling
  - trait.long_range
coordinate_annotations:
  S: astrophysical_to_cosmological
  M: relativistic_pde_system
  D: deterministic
  C: conservative_with_numerical_dissipation
  K: continuous
  L: local_hyperbolic_with_long_range_gravity_context
  R: irreversible_shock_physics
  T: dynamic_spacetime_topology_possible
  I: high_dimensional
  O: partially_observable
  G: effective_closure_or_subgrid
  Lambda: resolution_and_metric_limits
  Sigma: strongly_nonlinear
  Pi: grid
---

## Identity

- `display_name`: Coupled relativistic fluid and electromagnetic dynamics in curved spacetime.
- `summary`: Core astrophysical paradigm for accretion disks, jets, and compact-object environments.

## Activated Metadata Modules

### `type.PDE`

- `boundary_conditions`: horizon-compatible, outflow, reflective, and characteristic variants.
- `weak_form`: conservative weak formulations in curved spacetime coordinates.
- `flux_form`: relativistic conservative fluxes with geometric source terms.
- `discretization_options`: HRSC finite-volume, DG variants, constrained-transport-compatible schemes.

### `trait.discontinuity_handling`

- `shock_sensor`: relativistic characteristic and gradient-based detectors.
- `limiter_family`: TVD/WENO-style reconstructions adapted to relativistic variables.
- `entropy_fix_policy`: entropy-compatible flux policies and floors for physical admissibility.

### `trait.long_range`

- `accelerator_family`: metric and gravity-coupling acceleration via hierarchical or multigrid approaches.
- `error_control`: metric-consistency and divergence-control diagnostics.
- `far_field_model`: asymptotic boundary/metric approximations.

## Declared Invariants

- Conservative variable consistency and divergence-control constraints.

## Admissible Representations

- grid
- block_structured_grid

## Edge Ledger (machine-parseable)

```yaml
edges:
  - rel: eng:recommended_solver
    to: solver.gravity.p3m
    direction: out
    confidence: 0.72
    when:
      gravity_long_range_coupling: true
  - rel: eng:recommended_solver
    to: solver.riemann.hllc_hrsc
    direction: out
    confidence: 0.81
    when:
      shock_dominated_flow: true
```

## Related Nodes

- [solver.gravity.p3m](solver.gravity.p3m.md)
- [solver.riemann.hllc_hrsc](solver.riemann.hllc_hrsc.md)

#tech_glossary
#experimental_ontology
