---
canonical_name: Solver: HLLC HRSC
status: WIP-experimental
aliases:
  - HLLC Shock-Capturing Solver
source_of_truth: []
ontology_experimental: true
ontology_id: solver.riemann.hllc_hrsc
node_class: solver
node_types:
  - solver
  - riemann_solver
traits:
  - discontinuity_handling
projection_tags:
  - book:engineering_decision
  - book:computational_representation
  - book:mathematical_structure
activated_modules:
  - core.identity
  - type.solver
  - trait.discontinuity_handling
coordinate_annotations:
  S: continuum
  M: finite_volume_hyperbolic
  D: deterministic
  C: entropy_compatible
  K: discrete_time_space
  L: local_stencil
  R: irreversible_numerical
  T: discontinuity_resolving
  I: high_dimensional
  O: fully_observable_numeric_state
  G: closure_optional
  Lambda: dx_dt_to_zero
  Sigma: nonlinear_shock_regime
  Pi: grid
---

## Identity

- `display_name`: High-resolution shock-capturing method using HLLC interface flux.
- `summary`: Robust solver for compressible flows with shocks and contact discontinuities.

## Activated Metadata Modules

### `type.solver`

- `method_family`: finite-volume Godunov-type HRSC.
- `order_of_accuracy`: first-to-high order depending on reconstruction.
- `stability_characteristics`: CFL-limited explicit update with entropy-consistent flux policies.
- `complexity_model`: per-cell reconstruction and Riemann solve per interface per step.

### `trait.discontinuity_handling`

- `shock_sensor`: slope/gradient and wave-speed based indicators.
- `limiter_family`: TVD and WENO-family limiters.
- `entropy_fix_policy`: entropy-consistent wave-speed selection and entropy fixes.

## Declared Invariants

- Conservative finite-volume update.

## Admissible Representations

- grid
- block_structured_grid

## Edge Ledger (machine-parseable)

```yaml
edges:
  - rel: eng:recommended_for
    to: paradigm.pde.compressible_flow_with_shocks
    direction: out
    confidence: 0.9
    when:
      shocks_present: true
      robust_capture_required: true
  - rel: comp:implements_flux_update_for
    to: paradigm.pde.compressible_flow_with_shocks
    direction: out
    confidence: 0.9
```

## Related Nodes

- [paradigm.pde.compressible_flow_with_shocks](paradigm.pde.compressible_flow_with_shocks.md)

#tech_glossary
#experimental_ontology
