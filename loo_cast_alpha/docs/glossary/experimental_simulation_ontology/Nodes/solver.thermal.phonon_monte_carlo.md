---
canonical_name: Solver: Phonon Monte Carlo Transport
status: WIP-experimental
aliases:
  - Phonon MC
source_of_truth: []
ontology_experimental: true
ontology_id: solver.thermal.phonon_monte_carlo
node_class: solver
node_types:
  - solver
traits:
  - multi_scale
projection_tags:
  - book:physical_scale
  - book:engineering_decision
activated_modules:
  - core.identity
  - type.solver
  - trait.multi_scale
coordinate_annotations:
  S: molecular_to_nanoscale_thermal
  M: stochastic_transport_sampling
  D: ensemble
  C: energy_transport_conserving_approximation
  K: particle_like_quasiparticles
  L: finite_mean_free_path
  R: irreversible_scattering
  T: geometry_topology_dependent
  I: high_dimensional
  O: partially_observable
  G: kinetic_to_continuum_bridge
  Lambda: sample_size_to_infinity
  Sigma: variance_limited
  Pi: monte_carlo
---

## Identity

- `display_name`: Monte Carlo solver for phonon transport in BTE-like settings.
- `summary`: Models non-Fourier thermal effects by sampling phonon propagation and scattering events.

## Activated Metadata Modules

### `type.solver`

- `method_family`: event-driven stochastic transport sampling.
- `order_of_accuracy`: statistical and discretization dependent.
- `stability_characteristics`: variance and rare-event convergence are primary controls.
- `complexity_model`: particle count and scattering-event rate dominated.

### `trait.multi_scale`

- `micro_scale`: phonon mode and scattering dynamics.
- `macro_scale`: effective thermal flux and temperature fields.
- `bridge_operator`: moment extraction and kinetic-to-continuum closures.

## Declared Invariants

- Energy accounting under scattering and boundary models.

## Admissible Representations

- monte_carlo
- particle

## Edge Ledger (machine-parseable)

```yaml
edges:
  - rel: eng:recommended_for
    to: paradigm.thermal.phonon_bte
    direction: out
    confidence: 0.84
    when:
      ballistic_or_non_fourier_regime: true
```

#tech_glossary
#experimental_ontology
