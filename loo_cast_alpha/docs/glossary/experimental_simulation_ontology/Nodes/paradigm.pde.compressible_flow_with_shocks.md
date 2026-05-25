---
canonical_name: Paradigm: Compressible Flow with Shocks
status: WIP-experimental
aliases:
  - Compressible Euler/Navier-Stokes Shock Regime
source_of_truth: []
ontology_experimental: true
ontology_id: paradigm.pde.compressible_flow_with_shocks
node_class: paradigm
node_types:
  - PDE
  - hyperbolic_conservation_law
traits:
  - discontinuity_handling
projection_tags:
  - book:physical_scale
  - book:mathematical_structure
  - book:engineering_decision
  - book:computational_representation
activated_modules:
  - core.identity
  - type.PDE
  - trait.discontinuity_handling
coordinate_annotations:
  S: continuum
  M: PDE
  D: deterministic
  C: entropy_producing
  K: continuous_with_weak_discontinuities
  L: local
  R: irreversible
  T: shock_surface_formation
  I: high_dimensional
  O: partially_observable
  G: statistical_closure_optional
  Lambda: high_Reynolds_limit
  Sigma: bifurcation_and_chaotic_regimes
  Pi: grid
---

## Identity

- `display_name`: Compressible PDE dynamics with admissible shocks.
- `summary`: Hyperbolic or mixed hyperbolic-parabolic conservation laws where entropy-compatible discontinuities emerge.

## Activated Metadata Modules

### `type.PDE`

- `boundary_conditions`: inflow/outflow, wall, characteristic boundary closures.
- `weak_form`: weak conservation form with entropy condition.
- `flux_form`: conservative flux vector with numerical Riemann interface fluxes.
- `discretization_options`: finite volume, DG, high-resolution finite difference.

### `trait.discontinuity_handling`

- `shock_sensor`: gradient-based or smoothness-indicator shock detectors.
- `limiter_family`: TVD, WENO, slope-limiter variants.
- `entropy_fix_policy`: entropy-stable flux selection or explicit entropy fix.

## Declared Invariants

- Conservative update of mass, momentum, and total energy (up to numerical error).

## Admissible Representations

- grid
- discontinuous_galerkin

## Edge Ledger (machine-parseable)

```yaml
edges:
  - rel: eng:recommended_solver
    to: solver.riemann.hllc_hrsc
    direction: out
    confidence: 0.9
    when:
      shocks_present: true
      robust_capture_required: true
  - rel: math:linked_limit_process
    to: transform.limit.discrete_to_continuous
    direction: out
    confidence: 0.75
```

## Related Nodes

- [solver.riemann.hllc_hrsc](solver.riemann.hllc_hrsc.md)
- [transform.limit.discrete_to_continuous](transform.limit.discrete_to_continuous.md)

#tech_glossary
#experimental_ontology
