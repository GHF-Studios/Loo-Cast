---
canonical_name: Solver: Velocity Verlet Integrator
status: WIP-experimental
aliases:
  - Verlet
source_of_truth: []
ontology_experimental: true
ontology_id: solver.time.velocity_verlet
node_class: solver
node_types:
  - solver
traits:
  - structure_preserving
projection_tags:
  - book:engineering_decision
  - book:computational_representation
  - book:mathematical_structure
activated_modules:
  - core.identity
  - type.solver
  - trait.structure_preserving
coordinate_annotations:
  S: molecular_nanoscale
  M: symplectic_time_integration
  D: deterministic
  C: near_hamiltonian_preservation
  K: discrete_time
  L: local_update
  R: approximately_reversible
  T: fixed_phase_space_dimension
  I: high_dimensional
  O: fully_observable_numeric_state
  G: coarse_graining_compatible
  Lambda: dt_to_zero
  Sigma: bounded_energy_drift
  Pi: particle
---

## Identity

- `display_name`: Symplectic explicit integrator for second-order dynamics.
- `summary`: Standard molecular and particle-dynamics integrator with strong long-horizon behavior.

## Activated Metadata Modules

### `type.solver`

- `method_family`: symplectic explicit integrator.
- `order_of_accuracy`: second order.
- `stability_characteristics`: stable at moderate time steps bounded by fastest vibration modes.
- `complexity_model`: low arithmetic overhead per time step.

### `trait.structure_preserving`

- `preserved_invariants`: symplectic structure and bounded qualitative energy behavior.
- `monitor_strategy`: energy drift and phase-space diagnostics.

## Declared Invariants

- Symplectic update map.

## Admissible Representations

- particle
- vector

## Edge Ledger (machine-parseable)

```yaml
edges:
  - rel: eng:recommended_for
    to: paradigm.molecular.classical_md
    direction: out
    confidence: 0.86
    when:
      long_horizon_stability_required: true
```

## Related Nodes

- [paradigm.molecular.classical_md](paradigm.molecular.classical_md.md)

#tech_glossary
#experimental_ontology
