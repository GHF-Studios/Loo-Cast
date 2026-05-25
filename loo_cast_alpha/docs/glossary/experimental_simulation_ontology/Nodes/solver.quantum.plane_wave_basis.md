---
canonical_name: Solver: Plane-Wave Basis Expansion
status: WIP-experimental
aliases:
  - Plane-Wave DFT Solver Core
source_of_truth: []
ontology_experimental: true
ontology_id: solver.quantum.plane_wave_basis
node_class: solver
node_types:
  - solver
traits:
  - multi_scale
projection_tags:
  - book:physical_scale
  - book:computational_representation
activated_modules:
  - core.identity
  - type.solver
  - trait.multi_scale
coordinate_annotations:
  S: quantum_electronic
  M: spectral_basis_method
  D: deterministic
  C: variational_basis_controlled
  K: continuous_to_discrete_basis
  L: global_mode_coupling
  R: irreversible_iteration
  T: reciprocal_space_topology
  I: high_dimensional
  O: fully_observable_numeric_state
  G: pseudopotential_coarse_graining
  Lambda: cutoff_to_infinity
  Sigma: basis_cutoff_dependent
  Pi: spectral
---

## Identity

- `display_name`: Plane-wave basis representation for periodic electronic-structure solves.
- `summary`: Spectral representation widely used with pseudopotentials in DFT workflows.

## Activated Metadata Modules

### `type.solver`

- `method_family`: spectral basis expansion with iterative diagonalization.
- `order_of_accuracy`: controlled by cutoff and pseudopotential quality.
- `stability_characteristics`: robust in periodic systems; conditioning tied to basis cutoff.
- `complexity_model`: FFT and eigensolver dominated.

### `trait.multi_scale`

- `micro_scale`: high-frequency wavefunction components.
- `macro_scale`: low-frequency collective electronic density behavior.
- `bridge_operator`: cutoff-filtered spectral projection.

## Declared Invariants

- Variational convergence with increasing cutoff.

## Admissible Representations

- spectral
- basis_expansion

## Edge Ledger (machine-parseable)

```yaml
edges:
  - rel: eng:recommended_for
    to: paradigm.quantum.dft_kohn_sham
    direction: out
    confidence: 0.85
    when:
      periodic_system: true
```

#tech_glossary
#experimental_ontology
