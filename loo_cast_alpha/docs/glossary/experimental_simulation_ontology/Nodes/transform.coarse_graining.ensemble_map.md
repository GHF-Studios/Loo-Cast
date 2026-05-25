---
canonical_name: Transform: Coarse-Graining Ensemble Map
status: WIP-experimental
aliases:
  - Coarse-Graining Map
source_of_truth: []
ontology_experimental: true
ontology_id: transform.coarse_graining.ensemble_map
node_class: transform
node_types:
  - transform
  - coarse_graining
traits:
  - multi_scale
projection_tags:
  - book:transform_morphism
  - book:physical_scale
  - book:engineering_decision
activated_modules:
  - core.identity
  - type.transform
  - trait.multi_scale
coordinate_annotations:
  S: cross_scale
  M: probabilistic_projection
  D: ensemble
  C: effective_dissipation
  K: hybrid
  L: nonlocal_effective
  R: irreversible
  T: topology_reduced
  I: dimension_reduced
  O: partially_observable
  G: statistical_or_renormalized
  Lambda: sample_window_to_infinity
  Sigma: model_dependent
  Pi: grid_particle_or_graph
---

## Identity

- `display_name`: Map from fine-scale states to effective coarse observables.
- `summary`: Encodes scale-bridging transformation with explicit loss model.

## Activated Metadata Modules

### `type.transform`

- `domain_map`: microstate ensemble space.
- `codomain_map`: macro-observable state space.
- `invertibility`: generally non-invertible.
- `preserved_quantities`: selected conserved moments and balance laws.

### `trait.multi_scale`

- `micro_scale`: fine-grained state trajectories or distributions.
- `macro_scale`: coarse fields, moments, or latent reduced states.
- `bridge_operator`: projection, averaging, and closure correction.

## Declared Invariants

- Conservation of chosen low-order moments by construction.

## Admissible Representations

- grid
- particle
- graph
- latent_reduced_state

## Edge Ledger (machine-parseable)

```yaml
edges:
  - rel: math:applies_to
    to: paradigm.pde.incompressible_navier_stokes
    direction: out
    confidence: 0.78
  - rel: math:applies_to
    to: paradigm.particle.nbody_dynamics
    direction: out
    confidence: 0.75
```

## Related Nodes

- [paradigm.pde.incompressible_navier_stokes](paradigm.pde.incompressible_navier_stokes.md)
- [paradigm.particle.nbody_dynamics](paradigm.particle.nbody_dynamics.md)

#tech_glossary
#experimental_ontology
