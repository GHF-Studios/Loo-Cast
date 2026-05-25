---
canonical_name: Solver: Hybrid Monte Carlo
status: WIP-experimental
aliases:
  - HMC
source_of_truth: []
ontology_experimental: true
ontology_id: solver.quantum.hybrid_monte_carlo
node_class: solver
node_types:
  - solver
traits: []
projection_tags:
  - book:physical_scale
  - book:engineering_decision
  - book:computational_representation
activated_modules:
  - core.identity
  - type.solver
coordinate_annotations:
  S: quantum_subatomic
  M: mcmc_hamiltonian_sampling
  D: ensemble
  C: detailed_balance
  K: hybrid_discrete_continuous
  L: local_update_with_global_acceptance
  R: irreversible_chain_with_reversible_proposals
  T: fixed_lattice_topology
  I: high_dimensional
  O: partially_observable
  G: statistical
  Lambda: step_size_to_zero
  Sigma: acceptance_tuned_stability
  Pi: monte_carlo
---

## Identity

- `display_name`: Hamiltonian-trajectory MCMC with Metropolis correction.
- `summary`: Workhorse sampling algorithm for lattice gauge systems and high-dimensional posteriors.

## Activated Metadata Modules

### `type.solver`

- `method_family`: Hamiltonian Monte Carlo with accept/reject correction.
- `order_of_accuracy`: integrator-order dependent.
- `stability_characteristics`: acceptance and autocorrelation depend on step-size, mass preconditioning, and trajectory length.
- `complexity_model`: repeated force evaluations and linear-system solves dominate runtime.

## Declared Invariants

- Stationary target distribution under detailed-balance-compliant update.

## Admissible Representations

- monte_carlo
- lattice_state

## Edge Ledger (machine-parseable)

```yaml
edges:
  - rel: eng:recommended_for
    to: paradigm.quantum.lattice_qcd
    direction: out
    confidence: 0.9
    when:
      gauge_sampling_required: true
```

## Related Nodes

- [paradigm.quantum.lattice_qcd](paradigm.quantum.lattice_qcd.md)

#tech_glossary
#experimental_ontology
