---
canonical_name: Morphism: Particle Field Duality
status: WIP-experimental
aliases:
  - Particle-Field Duality Mapping
source_of_truth: []
ontology_experimental: true
ontology_id: morphism.duality.particle_field
node_class: morphism
node_types:
  - morphism
  - duality
traits: []
projection_tags:
  - book:transform_morphism
  - book:mathematical_structure
activated_modules:
  - core.identity
  - type.morphism
coordinate_annotations:
  S: cross_scale
  M: duality
  D: deterministic_or_ensemble
  C: model_dependent_conservation
  K: particle_field_hybrid
  L: long_range_or_localized_field
  R: model_dependent
  T: topology_sensitive
  I: representation_reorganization
  O: partially_observable
  G: mean_field_bridge
  Lambda: N_to_infinity_and_regularity_limits
  Sigma: regime_dependent
  Pi: particle_and_grid
---

## Identity

- `display_name`: Structured mapping between particle and field formulations.
- `morphism_class`: duality.

## Activated Metadata Modules

### `type.morphism`

- `source_pattern`: particle interaction dynamics.
- `target_pattern`: field potential and continuum coupling.
- `equivalence_mode`: approximate or asymptotic depending on scaling assumptions.
- `assumptions`: smoothness, closure, and boundary regularity constraints.

## Mapping Structure

- `source_nodes`: `paradigm.particle.nbody_dynamics`.
- `target_nodes`: `paradigm.field.poisson_continuum`.
- `forward_map`: particle-to-density projection then field solve.
- `inverse_map`: field-gradient sampling to effective particle forces.
- `domain_of_validity`: large particle counts or smooth empirical measure regimes.

## Preserved or Transformed Quantities

- `preserved_quantities`: total source mass/charge and selected moments.
- `lost_quantities`: micro-level pairwise trajectory detail.
- `error_or_gap_model`: closure and finite-sampling approximation error.

## Edge Ledger (machine-parseable)

```yaml
edges:
  - rel: math:maps_from
    to: paradigm.particle.nbody_dynamics
    direction: out
    confidence: 0.8
  - rel: math:maps_to
    to: paradigm.field.poisson_continuum
    direction: out
    confidence: 0.8
  - rel: comp:implementation_support
    to: solver.nbody.fast_multipole_method
    direction: out
    confidence: 0.74
```

## Related Nodes

- [paradigm.particle.nbody_dynamics](paradigm.particle.nbody_dynamics.md)
- [paradigm.field.poisson_continuum](paradigm.field.poisson_continuum.md)
- [solver.nbody.fast_multipole_method](solver.nbody.fast_multipole_method.md)

#tech_glossary
#experimental_ontology
