---
canonical_name: Morphism: Statistical Mechanics Bayesian Analogy
status: WIP-experimental
aliases:
  - StatMech Bayesian Analogy
source_of_truth: []
ontology_experimental: true
ontology_id: morphism.analogy.statmech_bayesian
node_class: morphism
node_types:
  - morphism
  - analogy
traits: []
projection_tags:
  - book:transform_morphism
  - book:mathematical_structure
activated_modules:
  - core.identity
  - type.morphism
coordinate_annotations:
  S: information_scale
  M: cross_domain_mapping
  D: ensemble
  C: entropy_regularized
  K: hybrid
  L: model_dependent
  R: irreversible_update
  T: phase_landscape_analogy
  I: probabilistic_high_dimensional
  O: partially_observable
  G: statistical
  Lambda: sample_or_system_size_to_infinity
  Sigma: multimodal_or_critical
  Pi: monte_carlo_or_variational
---

## Identity

- `display_name`: Cross-domain analogy between statistical mechanics and Bayesian inference.
- `morphism_class`: analogy.

## Activated Metadata Modules

### `type.morphism`

- `source_pattern`: thermodynamic ensemble weighting and partition structure.
- `target_pattern`: posterior weighting and evidence normalization.
- `equivalence_mode`: interpretive and partially formal.
- `assumptions`: compatible energy/log-likelihood correspondence and normalization structure.

## Mapping Structure

- `source_nodes`: `paradigm.statistical_mechanics.ensemble_inference`.
- `target_nodes`: `paradigm.bayesian_inference.posterior_dynamics`.
- `forward_map`: energy landscape to negative log-posterior map.
- `inverse_map`: posterior objective interpretation as effective free-energy landscape.
- `domain_of_validity`: formal analogy and selected exact correspondences.

## Preserved or Transformed Quantities

- `preserved_quantities`: normalization structure and variational principles.
- `lost_quantities`: direct physical interpretation in non-physical inference models.
- `error_or_gap_model`: analogy gap when modeling assumptions diverge.

## Edge Ledger (machine-parseable)

```yaml
edges:
  - rel: xdom:maps_from
    to: paradigm.statistical_mechanics.ensemble_inference
    direction: out
    confidence: 0.82
  - rel: xdom:maps_to
    to: paradigm.bayesian_inference.posterior_dynamics
    direction: out
    confidence: 0.82
```

## Related Nodes

- [paradigm.statistical_mechanics.ensemble_inference](paradigm.statistical_mechanics.ensemble_inference.md)
- [paradigm.bayesian_inference.posterior_dynamics](paradigm.bayesian_inference.posterior_dynamics.md)

#tech_glossary
#experimental_ontology
