---
canonical_name: Solver: SPH Cubic-Spline Kernel Stack
status: WIP-experimental
aliases:
  - SPH Cubic Spline
source_of_truth: []
ontology_experimental: true
ontology_id: solver.fluid.sph_cubic_spline
node_class: solver
node_types:
  - solver
traits:
  - nonlocal
projection_tags:
  - book:engineering_decision
  - book:computational_representation
activated_modules:
  - core.identity
  - type.solver
  - trait.nonlocal
coordinate_annotations:
  S: continuum_to_particle_hybrid
  M: meshless_particle_method
  D: deterministic_or_stochastic_extensions
  C: weakly_conservative
  K: particle
  L: finite_range_kernel
  R: irreversible_numerical
  T: dynamic_neighbor_topology
  I: high_dimensional
  O: fully_observable_numeric_state
  G: kernel_smoothing
  Lambda: h_to_zero
  Sigma: tensile_and_pairing_instability_sensitive
  Pi: particle
---

## Identity

- `display_name`: Meshless SPH solver using cubic-spline smoothing kernels.
- `summary`: Particle-based fluid solver suitable for free surfaces and large-deformation flows.

## Activated Metadata Modules

### `type.solver`

- `method_family`: meshless kernel interpolation and particle interaction updates.
- `order_of_accuracy`: kernel and consistency correction dependent.
- `stability_characteristics`: requires density correction and stabilization terms in challenging regimes.
- `complexity_model`: neighbor search and pairwise kernel evaluations dominate.

### `trait.nonlocal`

- `interaction_kernel`: compact-support cubic spline kernels.
- `support_radius`: smoothing-length dependent local neighborhood.
- `decay_behavior`: compact support with zero contribution beyond kernel radius.

## Declared Invariants

- Mass conservation in particle summation formulations.

## Admissible Representations

- particle

## Edge Ledger (machine-parseable)

```yaml
edges:
  - rel: eng:recommended_for
    to: paradigm.fluid.sph
    direction: out
    confidence: 0.87
    when:
      free_surface_dynamics: true
```

#tech_glossary
#experimental_ontology
