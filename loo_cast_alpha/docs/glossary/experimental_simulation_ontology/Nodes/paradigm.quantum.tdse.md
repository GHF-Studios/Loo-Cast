---
canonical_name: Paradigm: Time-Dependent Schrodinger Equation
status: WIP-experimental
aliases:
  - TDSE
source_of_truth: []
ontology_experimental: true
ontology_id: paradigm.quantum.tdse
node_class: paradigm
node_types:
  - PDE
traits:
  - variational
projection_tags:
  - book:physical_scale
  - book:mathematical_structure
  - book:engineering_decision
activated_modules:
  - core.identity
  - type.PDE
  - trait.variational
coordinate_annotations:
  S: quantum_microscopic
  M: complex_valued_pde
  D: deterministic
  C: norm_preserving_closed_system
  K: continuous
  L: local_or_nonlocal_potential
  R: reversible_closed_system
  T: fixed_domain_or_adaptive_boundary
  I: high_dimensional_wavefunction
  O: observer_relative_measurement
  G: reduced_or_effective_hamiltonians
  Lambda: discretization_to_continuum
  Sigma: unitary_stability_requirements
  Pi: grid_or_spectral
---

## Identity

- `display_name`: Quantum wavefunction evolution under time-dependent Hamiltonians.
- `summary`: Core PDE for coherent quantum dynamics in real-space or basis-space representations.

## Activated Metadata Modules

### `type.PDE`

- `boundary_conditions`: periodic, absorbing, reflective, and complex-scaled variants.
- `weak_form`: complex Hilbert-space weak formulations with Hermitian structure constraints.
- `flux_form`: probability-current-conserving flux representation where applicable.
- `discretization_options`: split-operator Fourier, Crank-Nicolson, finite difference, finite element.

### `trait.variational`

- `functional`: action or energy-functional formulations under chosen gauge.
- `admissible_space`: complex Hilbert spaces with normalization constraints.
- `stationary_condition`: Euler-Lagrange and unitary-consistency conditions.

## Declared Invariants

- Probability-norm preservation in closed-system unitary evolution.

## Admissible Representations

- grid
- spectral
- basis_expansion

## Edge Ledger (machine-parseable)

```yaml
edges:
  - rel: eng:recommended_solver
    to: solver.quantum.split_operator_fourier
    direction: out
    confidence: 0.88
    when:
      periodic_or_fft_friendly_domain: true
```

## Related Nodes

- [solver.quantum.split_operator_fourier](solver.quantum.split_operator_fourier.md)

#tech_glossary
#experimental_ontology
