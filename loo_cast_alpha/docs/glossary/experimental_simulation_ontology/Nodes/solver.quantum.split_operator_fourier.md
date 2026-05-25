---
canonical_name: Solver: Split-Operator Fourier TDSE Integrator
status: WIP-experimental
aliases:
  - Split-Operator FFT
source_of_truth: []
ontology_experimental: true
ontology_id: solver.quantum.split_operator_fourier
node_class: solver
node_types:
  - solver
traits:
  - structure_preserving
projection_tags:
  - book:physical_scale
  - book:engineering_decision
  - book:computational_representation
activated_modules:
  - core.identity
  - type.solver
  - trait.structure_preserving
coordinate_annotations:
  S: quantum_microscopic
  M: operator_splitting_spectral
  D: deterministic
  C: near_unitary_preservation
  K: discrete_time
  L: global_fft_coupling
  R: approximately_reversible
  T: fixed_grid_topology
  I: high_dimensional
  O: fully_observable_numeric_state
  G: effective_operator_factoring
  Lambda: dt_to_zero
  Sigma: stable_under_splitting_assumptions
  Pi: spectral
---

## Identity

- `display_name`: Operator-split TDSE integrator using Fourier transforms.
- `summary`: Alternates kinetic and potential propagators for efficient quantum evolution.

## Activated Metadata Modules

### `type.solver`

- `method_family`: spectral operator splitting.
- `order_of_accuracy`: second order in standard Strang form.
- `stability_characteristics`: robust for smooth potentials and FFT-compatible domains; aliasing and boundary handling require control.
- `complexity_model`: FFT-dominated per-step complexity.

### `trait.structure_preserving`

- `preserved_invariants`: wavefunction norm and qualitative phase-space structure.
- `monitor_strategy`: norm drift, phase error, and boundary reflection diagnostics.

## Declared Invariants

- Norm-preserving update under consistent split and boundary handling.

## Admissible Representations

- spectral
- grid

## Edge Ledger (machine-parseable)

```yaml
edges:
  - rel: eng:recommended_for
    to: paradigm.quantum.tdse
    direction: out
    confidence: 0.88
    when:
      periodic_or_fft_friendly_domain: true
```

## Related Nodes

- [paradigm.quantum.tdse](paradigm.quantum.tdse.md)

#tech_glossary
#experimental_ontology
