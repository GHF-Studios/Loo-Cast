---
canonical_name: Solver: Fixed-Node Quantum Monte Carlo
status: WIP-experimental
aliases:
  - Fixed-Node QMC
source_of_truth: []
ontology_experimental: true
ontology_id: solver.quantum.fixed_node_qmc
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
  S: quantum_many_body
  M: stochastic_variational_sampling
  D: ensemble
  C: projected_ground_state_approximation
  K: hybrid
  L: global_state_coupling
  R: irreversible_sampling
  T: nodal_surface_topology_constrained
  I: high_dimensional
  O: partially_observable
  G: variational_coarse_graining
  Lambda: sample_size_to_infinity
  Sigma: variance_and_sign_control_dependent
  Pi: monte_carlo
---

## Identity

- `display_name`: QMC engine with fixed-node constraint to control fermion sign instability.
- `summary`: Practical high-accuracy stochastic many-body solver with nodal-constraint bias tradeoffs.

## Activated Metadata Modules

### `type.solver`

- `method_family`: variational/diffusion Monte Carlo with fixed-node projection.
- `order_of_accuracy`: statistical and nodal-quality dependent.
- `stability_characteristics`: stable sampling requires population control and nodal-quality management.
- `complexity_model`: large-sample stochastic evolution and estimator accumulation.

## Declared Invariants

- Variational upper-bound properties under fixed-node assumptions.

## Admissible Representations

- monte_carlo
- particle

## Edge Ledger (machine-parseable)

```yaml
edges:
  - rel: eng:recommended_for
    to: paradigm.quantum.qmc_vmc_dmc
    direction: out
    confidence: 0.81
    when:
      fermionic_sign_control_required: true
```

#tech_glossary
#experimental_ontology
