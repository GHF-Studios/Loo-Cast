---
canonical_name: Solver: Localized Gaussian Basis Expansion
status: WIP-experimental
aliases:
  - Gaussian Basis Solver Core
source_of_truth: []
ontology_experimental: true
ontology_id: solver.quantum.gaussian_basis
node_class: solver
node_types:
  - solver
traits: []
projection_tags:
  - book:physical_scale
  - book:computational_representation
activated_modules:
  - core.identity
  - type.solver
coordinate_annotations:
  S: quantum_molecular
  M: localized_basis_method
  D: deterministic
  C: variational_basis_controlled
  K: continuous_to_discrete_basis
  L: sparse_localized_coupling
  R: irreversible_iteration
  T: basis_graph_topology
  I: high_dimensional
  O: fully_observable_numeric_state
  G: effective_core_potential_optional
  Lambda: basis_set_to_complete_limit
  Sigma: basis_conditioning_dependent
  Pi: basis_expansion
---

## Identity

- `display_name`: Localized orbital-basis approach for electronic-structure and quantum chemistry.
- `summary`: Efficient for molecular systems where locality and sparse coupling can be exploited.

## Activated Metadata Modules

### `type.solver`

- `method_family`: localized basis integral assembly with iterative SCF/eigensolve loop.
- `order_of_accuracy`: basis-set and quadrature dependent.
- `stability_characteristics`: sensitive to basis conditioning and integral approximation.
- `complexity_model`: integral evaluation and matrix operations dominate.

## Declared Invariants

- Basis-consistent charge and energy convergence targets.

## Admissible Representations

- basis_expansion
- sparse_system

## Edge Ledger (machine-parseable)

```yaml
edges:
  - rel: eng:recommended_for
    to: paradigm.quantum.dft_kohn_sham
    direction: out
    confidence: 0.84
    when:
      localized_molecular_system: true
```

#tech_glossary
#experimental_ontology
