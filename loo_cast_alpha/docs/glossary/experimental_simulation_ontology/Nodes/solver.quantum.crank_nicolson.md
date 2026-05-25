---
canonical_name: Solver: Crank-Nicolson Finite Difference
status: WIP-experimental
aliases:
  - Crank-Nicolson TDSE
source_of_truth: []
ontology_experimental: true
ontology_id: solver.quantum.crank_nicolson
node_class: solver
node_types:
  - solver
traits:
  - structure_preserving
projection_tags:
  - book:physical_scale
  - book:engineering_decision
activated_modules:
  - core.identity
  - type.solver
  - trait.structure_preserving
coordinate_annotations:
  S: quantum_microscopic
  M: implicit_fd_time_stepping
  D: deterministic
  C: norm_stable_discretization
  K: discrete_grid
  L: local_stencil_with_global_solve
  R: approximately_reversible
  T: fixed_grid_topology
  I: high_dimensional
  O: fully_observable_numeric_state
  G: coarse_grained_optional
  Lambda: dx_dt_to_zero
  Sigma: A_stable
  Pi: grid
---

## Identity

- `display_name`: Implicit midpoint-like scheme for TDSE finite-difference evolution.
- `summary`: Robust alternative to split-operator methods when boundary and potential structure are less FFT-friendly.

## Activated Metadata Modules

### `type.solver`

- `method_family`: implicit second-order time integration with sparse linear solves.
- `order_of_accuracy`: second order in time.
- `stability_characteristics`: unconditionally stable in linear settings with consistent discretization.
- `complexity_model`: sparse solve per time step dominates runtime.

### `trait.structure_preserving`

- `preserved_invariants`: near-unitary update under consistent linear solve tolerance.
- `monitor_strategy`: norm and phase drift diagnostics.

## Declared Invariants

- Norm stability in closed-system discretizations.

## Admissible Representations

- grid
- sparse_system

## Edge Ledger (machine-parseable)

```yaml
edges:
  - rel: eng:recommended_for
    to: paradigm.quantum.tdse
    direction: out
    confidence: 0.84
    when:
      implicit_stability_preferred: true
```

#tech_glossary
#experimental_ontology
