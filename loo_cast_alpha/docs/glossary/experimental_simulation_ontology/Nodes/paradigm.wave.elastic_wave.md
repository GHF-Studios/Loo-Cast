---
canonical_name: Paradigm: Elastic Wave Propagation
status: WIP-experimental
aliases: []
source_of_truth: []
ontology_experimental: true
ontology_id: paradigm.wave.elastic_wave
node_class: paradigm
node_types:
  - wave_propagation_model
traits:
  - multi_scale
projection_tags:
  - book:physical_scale
activated_modules:
  - core.identity
  - trait.multi_scale
coordinate_annotations:
  S: continuum_structural_wave
  Pi: grid_or_mesh
---

## Identity

- `display_name`: Elastic stress-wave propagation in solids.
- `summary`: Seismic/structural wave paradigm with material and geometric dispersion effects.

### `trait.multi_scale`

- `micro_scale`: local material heterogeneity effects.
- `macro_scale`: domain-scale wavefront dynamics.
- `bridge_operator`: homogenized wave-speed and attenuation closures.

## Edge Ledger (machine-parseable)

```yaml
edges: []
```

#tech_glossary
#experimental_ontology

