---
canonical_name: Solver: Kohn-Sham SCF Iteration
status: WIP-experimental
aliases:
  - SCF Iteration
source_of_truth: []
ontology_experimental: true
ontology_id: solver.quantum.scf_iteration
node_class: solver
node_types:
  - solver
traits: []
projection_tags:
  - book:physical_scale
  - book:engineering_decision
activated_modules:
  - core.identity
  - type.solver
coordinate_annotations:
  S: quantum_electronic
  M: nonlinear_fixed_point
  D: deterministic
  C: energy_functional_descent_mixed
  K: discrete_basis_or_grid
  L: global_density_coupling
  R: irreversible_iteration
  T: fixed_basis_topology
  I: high_dimensional
  O: partially_observable
  G: mean_field
  Lambda: basis_to_complete_limit
  Sigma: convergence_mixing_dependent
  Pi: grid_or_basis_expansion
---

## Identity

- `display_name`: Self-consistent field fixed-point solver for Kohn-Sham DFT.
- `summary`: Iterative electronic-structure solver based on density/potential consistency.

## Activated Metadata Modules

### `type.solver`

- `method_family`: fixed-point and quasi-Newton density mixing iterations.
- `order_of_accuracy`: basis and discretization dependent.
- `stability_characteristics`: convergence depends on mixing, preconditioning, and electronic structure regime.
- `complexity_model`: repeated Hamiltonian builds and eigenvalue solves.

## Declared Invariants

- Charge normalization and self-consistency residual reduction targets.

## Admissible Representations

- basis_expansion
- grid

## Edge Ledger (machine-parseable)

```yaml
edges:
  - rel: eng:recommended_for
    to: paradigm.quantum.dft_kohn_sham
    direction: out
    confidence: 0.9
    when:
      electronic_structure_required: true
```

#tech_glossary
#experimental_ontology
