---
canonical_name: Paradigm: QM/MM Hybrid Coupling
status: WIP-experimental
aliases:
  - QM/MM
source_of_truth: []
ontology_experimental: true
ontology_id: paradigm.multiscale.qmmm_hybrid
node_class: paradigm
node_types:
  - hybrid_coupling
traits:
  - multi_scale
projection_tags:
  - book:physical_scale
  - book:mathematical_structure
  - book:engineering_decision
activated_modules:
  - core.identity
  - trait.multi_scale
coordinate_annotations:
  S: quantum_to_molecular
  M: hybrid_variational_dynamics
  D: deterministic_or_stochastic
  C: model_dependent_conservation
  K: hybrid
  L: local_quantum_region_with_extended_classical_region
  R: reversible_or_irreversible_model_dependent
  T: adaptive_partition_topology_possible
  I: high_dimensional
  O: partially_observable
  G: effective_coupling_closures
  Lambda: partition_refinement_limits
  Sigma: stiff_interface_regime
  Pi: hybrid_basis_and_particle
---

## Identity

- `display_name`: Coupled quantum-mechanical and molecular-mechanical simulation paradigm.
- `summary`: Resolves chemically active regions at quantum fidelity while embedding in larger classical environments.

## Activated Metadata Modules

### `trait.multi_scale`

- `micro_scale`: quantum active-site electron and nuclei dynamics.
- `macro_scale`: classical environment and long-range structural response.
- `bridge_operator`: interface Hamiltonians and force/energy partitioning operators.

## Declared Invariants

- Interface consistency for force and energy exchange.

## Admissible Representations

- hybrid_basis_and_particle
- domain_decomposition

## Edge Ledger (machine-parseable)

```yaml
edges:
  - rel: math:coarse_grained_by
    to: transform.coarse_graining.ensemble_map
    direction: out
    confidence: 0.69
  - rel: eng:recommended_solver
    to: solver.time.velocity_verlet
    direction: out
    confidence: 0.65
    when:
      mm_subsystem_time_integration: true
```

## Related Nodes

- [transform.coarse_graining.ensemble_map](transform.coarse_graining.ensemble_map.md)
- [solver.time.velocity_verlet](solver.time.velocity_verlet.md)

#tech_glossary
#experimental_ontology
