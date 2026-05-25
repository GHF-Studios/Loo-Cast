---
canonical_name: Solver: FFT Gravity Poisson Solver
status: WIP-experimental
aliases:
  - FFT Poisson Gravity
source_of_truth: []
ontology_experimental: true
ontology_id: solver.gravity.fft_poisson
node_class: solver
node_types:
  - solver
traits:
  - long_range
projection_tags:
  - book:physical_scale
  - book:engineering_decision
activated_modules:
  - core.identity
  - type.solver
  - trait.long_range
coordinate_annotations:
  S: astrophysical_to_cosmological
  M: spectral_elliptic_solve
  D: deterministic
  C: conservative_potential_approximation
  K: grid
  L: global_spectral_coupling
  R: reversible_or_irreversible_model_dependent
  T: periodic_domain_topology
  I: high_dimensional
  O: fully_observable_numeric_state
  G: mean_field_long_range
  Lambda: dx_to_zero
  Sigma: aliasing_and_boundary_sensitive
  Pi: spectral
---

## Identity

- `display_name`: Spectral Poisson solver for long-range gravitational potentials.
- `summary`: Efficient long-range force solution on periodic grids in particle-mesh cosmology stacks.

## Activated Metadata Modules

### `type.solver`

- `method_family`: FFT-based spectral inversion of Poisson equations.
- `order_of_accuracy`: spectral in smooth periodic settings.
- `stability_characteristics`: robust in periodic domains with careful de-aliasing.
- `complexity_model`: `O(N log N)` FFT-dominated.

### `trait.long_range`

- `accelerator_family`: global Fourier-mode potential solve.
- `error_control`: mesh resolution and assignment-kernel control.
- `far_field_model`: long-range potential represented in spectral domain.

## Declared Invariants

- Consistent periodic Poisson solve under spectral discretization assumptions.

## Admissible Representations

- spectral
- grid

## Edge Ledger (machine-parseable)

```yaml
edges:
  - rel: eng:recommended_for
    to: paradigm.cosmo.particle_mesh_nbody
    direction: out
    confidence: 0.9
    when:
      periodic_cosmology_domain: true
```

#tech_glossary
#experimental_ontology
