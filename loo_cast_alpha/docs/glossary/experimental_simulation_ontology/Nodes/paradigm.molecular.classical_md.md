---
canonical_name: Paradigm: Classical Molecular Dynamics
status: WIP-experimental
aliases:
  - MD
source_of_truth: []
ontology_experimental: true
ontology_id: paradigm.molecular.classical_md
node_class: paradigm
node_types:
  - particle_system
traits:
  - nonlocal
  - multi_scale
projection_tags:
  - book:physical_scale
  - book:mathematical_structure
  - book:engineering_decision
  - book:computational_representation
activated_modules:
  - core.identity
  - trait.nonlocal
  - trait.multi_scale
coordinate_annotations:
  S: molecular_nanoscale
  M: many_body_ode
  D: deterministic_or_stochastic_thermostatted
  C: Hamiltonian_or_controlled_dissipative
  K: particle
  L: finite_range_with_long_range_corrections
  R: reversible_or_thermostatted_irreversible
  T: dynamic_neighbor_topology
  I: high_dimensional
  O: partially_observable
  G: coarse_grained_or_all_atom
  Lambda: dt_to_zero
  Sigma: metastable_and_chaotic
  Pi: particle
---

## Identity

- `display_name`: Atomistic many-body time evolution under force-field or ab-initio potentials.
- `summary`: Canonical simulation paradigm for molecular structure, dynamics, and transport.

## Activated Metadata Modules

### `trait.nonlocal`

- `interaction_kernel`: bonded local terms plus nonbonded pairwise kernels.
- `support_radius`: short-range cutoffs with optional long-range electrostatic corrections.
- `decay_behavior`: mixed near-field strong interactions and screened or Ewald-type long-range behavior.

### `trait.multi_scale`

- `micro_scale`: bond vibrations and local collisions.
- `macro_scale`: diffusion, conformation, mesoscale transport.
- `bridge_operator`: coarse-graining maps and force-matching closures.

## Declared Invariants

- Total energy conservation in microcanonical settings.

## Admissible Representations

- particle
- particle_mesh_hybrid

## Edge Ledger (machine-parseable)

```yaml
edges:
  - rel: eng:recommended_solver
    to: solver.time.velocity_verlet
    direction: out
    confidence: 0.86
    when:
      long_horizon_stability_required: true
```

## Related Nodes

- [solver.time.velocity_verlet](solver.time.velocity_verlet.md)

#tech_glossary
#experimental_ontology
