---
canonical_name: Paradigm: Lattice Quantum Chromodynamics
status: WIP-experimental
aliases:
  - LQCD
source_of_truth: []
ontology_experimental: true
ontology_id: paradigm.quantum.lattice_qcd
node_class: paradigm
node_types:
  - lattice_field_model
  - monte_carlo_physics
traits:
  - multi_scale
projection_tags:
  - book:physical_scale
  - book:mathematical_structure
  - book:computational_representation
activated_modules:
  - core.identity
  - trait.multi_scale
coordinate_annotations:
  S: quantum_subatomic
  M: lattice_field_theory
  D: ensemble
  C: gauge_constrained
  K: discrete_lattice
  L: local_lattice_coupling
  R: ensemble_irreversible_sampling
  T: fixed_lattice_topology
  I: high_dimensional
  O: partially_observable
  G: renormalized
  Lambda: continuum_limit
  Sigma: critical_slowing_down_regime
  Pi: grid_and_monte_carlo
---

## Identity

- `display_name`: Lattice discretization of QCD path-integral dynamics.
- `summary`: Non-perturbative gauge-field simulation framework based on Euclidean spacetime lattices.

## Activated Metadata Modules

### `trait.multi_scale`

- `micro_scale`: lattice-link and plaquette fluctuations.
- `macro_scale`: hadronic observables and effective field behavior.
- `bridge_operator`: renormalization-group and continuum extrapolation procedures.

## Declared Invariants

- Gauge-symmetry consistency under chosen discretization.

## Admissible Representations

- grid
- tensor_network
- monte_carlo

## Edge Ledger (machine-parseable)

```yaml
edges:
  - rel: eng:recommended_solver
    to: solver.quantum.hybrid_monte_carlo
    direction: out
    confidence: 0.9
    when:
      gauge_sampling_required: true
  - rel: math:linked_limit_process
    to: transform.limit.discrete_to_continuous
    direction: out
    confidence: 0.76
```

## Related Nodes

- [solver.quantum.hybrid_monte_carlo](solver.quantum.hybrid_monte_carlo.md)
- [transform.limit.discrete_to_continuous](transform.limit.discrete_to_continuous.md)

#tech_glossary
#experimental_ontology
