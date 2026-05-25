---
canonical_name: Solver: Symplectic Verlet Integrator
status: WIP-experimental
aliases:
  - Velocity Verlet
source_of_truth: []
ontology_experimental: true
ontology_id: solver.time.symplectic_verlet
node_class: solver
node_types:
  - solver
  - time_integrator
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
  S: molecular_to_mesoscopic
  M: geometric_integration
  D: deterministic
  C: Hamiltonian_preserving
  K: discrete_time
  L: local_update
  R: approximately_reversible
  T: fixed_phase_space_topology
  I: medium_to_high_dimensional
  O: fully_observable_numeric_state
  G: coarse_graining_optional
  Lambda: dt_to_zero
  Sigma: bounded_long_horizon_drift
  Pi: particle_or_vector
---

## Identity

- `display_name`: Symplectic second-order time integrator.
- `summary`: Preserves geometric structure for long-time integration of Hamiltonian-like systems.

## Activated Metadata Modules

### `type.solver`

- `method_family`: symplectic partitioned update scheme.
- `order_of_accuracy`: second order.
- `stability_characteristics`: bounded qualitative energy drift for Hamiltonian systems.
- `complexity_model`: low per-step arithmetic cost and no global linear solve in explicit form.

### `trait.structure_preserving`

- `preserved_invariants`: symplectic form and phase-space volume (discrete approximation).
- `monitor_strategy`: long-horizon energy drift and phase-volume diagnostics.

## Declared Invariants

- Symplectic structure in discrete map.

## Admissible Representations

- particle
- vector

## Edge Ledger (machine-parseable)

```yaml
edges:
  - rel: eng:recommended_for
    to: paradigm.particle.nbody_dynamics
    direction: out
    confidence: 0.83
    when:
      structure_preservation_priority: true
      long_horizon_integration: true
```

## Related Nodes

- [paradigm.particle.nbody_dynamics](paradigm.particle.nbody_dynamics.md)

#tech_glossary
#experimental_ontology
