---
canonical_name: Solver: Hyperbolic Divergence Cleaning
status: WIP-experimental
aliases:
  - Divergence Cleaning
source_of_truth: []
ontology_experimental: true
ontology_id: solver.plasma.divergence_cleaning
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
  S: plasma_to_astrophysical
  M: augmented_hyperbolic_system
  D: deterministic
  C: divergence_damping
  K: discrete_grid
  L: local_stencil
  R: irreversible_damping
  T: mesh_topology
  I: high_dimensional
  O: fully_observable_numeric_state
  G: closure_optional
  Lambda: dx_dt_to_zero
  Sigma: parameter_tuned_stability
  Pi: grid
---

## Identity

- `display_name`: Divergence-error propagation and damping mechanism for MHD-like systems.
- `summary`: Adds auxiliary cleaning fields to control magnetic divergence growth when CT is unavailable or combined.

## Activated Metadata Modules

### `type.solver`

- `method_family`: augmented hyperbolic/parabolic divergence-cleaning equations.
- `order_of_accuracy`: base discretization dependent.
- `stability_characteristics`: cleaning speed and damping parameters require tuning.
- `complexity_model`: local additional field updates with moderate overhead.

### `trait.structure_preserving`

- `preserved_invariants`: bounded divergence-error growth.
- `monitor_strategy`: divergence norm decay and cleaning-field diagnostics.

## Declared Invariants

- Controlled divergence-error propagation under selected parameters.

## Admissible Representations

- grid

## Edge Ledger (machine-parseable)

```yaml
edges:
  - rel: eng:recommended_for
    to: paradigm.plasma.resistive_mhd
    direction: out
    confidence: 0.83
    when:
      ct_not_available: true
```

#tech_glossary
#experimental_ontology
