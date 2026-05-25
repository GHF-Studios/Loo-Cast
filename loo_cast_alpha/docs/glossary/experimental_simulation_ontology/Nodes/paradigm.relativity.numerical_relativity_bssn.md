---
canonical_name: Paradigm: Numerical Relativity (BSSN/CCZ4 Family)
status: WIP-experimental
aliases:
  - Numerical Relativity
source_of_truth: []
ontology_experimental: true
ontology_id: paradigm.relativity.numerical_relativity_bssn
node_class: paradigm
node_types:
  - PDE
traits:
  - multi_scale
projection_tags:
  - book:physical_scale
  - book:mathematical_structure
  - book:engineering_decision
activated_modules:
  - core.identity
  - type.PDE
  - trait.multi_scale
coordinate_annotations:
  S: astrophysical_to_cosmological
  M: constrained_hyperbolic_pde
  D: deterministic
  C: constraint_controlled
  K: continuous
  L: local_differential_structure
  R: irreversible_numerical
  T: dynamic_spacetime_topology
  I: high_dimensional
  O: partially_observable
  G: gauge_and_constraint_reductions
  Lambda: resolution_and_gauge_limits
  Sigma: strongly_nonlinear
  Pi: grid
---

## Identity

- `display_name`: Spacetime evolution under Einstein equations in numerical formulations.
- `summary`: Core relativistic framework for binary mergers, gravitational-wave sources, and spacetime dynamics.

## Activated Metadata Modules

### `type.PDE`

- `boundary_conditions`: constraint-preserving and radiative outer-boundary treatments.
- `weak_form`: formulation-dependent constrained evolution structure.
- `flux_form`: hyperbolic fluxes with gauge and source couplings.
- `discretization_options`: finite difference, finite volume, spectral element, AMR-enabled hybrids.

### `trait.multi_scale`

- `micro_scale`: near-horizon gradients and local curvature peaks.
- `macro_scale`: global wave propagation and spacetime structure.
- `bridge_operator`: adaptive refinement and gauge-control operators.

## Declared Invariants

- Constraint-violation control under selected formulation and gauge.

## Admissible Representations

- grid
- amr_grid

## Edge Ledger (machine-parseable)

```yaml
edges:
  - rel: eng:recommended_solver
    to: solver.relativity.bssn_ccz4
    direction: out
    confidence: 0.84
    when:
      full_spacetime_evolution_required: true
```

## Related Nodes

- [solver.relativity.bssn_ccz4](solver.relativity.bssn_ccz4.md)

#tech_glossary
#experimental_ontology
