---
canonical_name: Paradigm: Radiative Transfer Monte Carlo
status: WIP-experimental
aliases:
  - Photon Transport Monte Carlo
source_of_truth: []
ontology_experimental: true
ontology_id: paradigm.radiative_transfer.monte_carlo
node_class: paradigm
node_types:
  - transport_radiative_model
traits:
  - stochastic
projection_tags:
  - book:physical_scale
activated_modules:
  - core.identity
  - trait.stochastic
coordinate_annotations:
  S: continuum_radiative
  Pi: monte_carlo
---

## Identity

- `display_name`: Stochastic photon packet transport paradigm.
- `summary`: Probabilistic radiative-transfer framework for complex scattering/absorption regimes.

### `trait.stochastic`

- `sampling_regime`: photon-packet random walk with probabilistic interaction events.
- `estimator_family`: path-length, collision, and tally-based radiative estimators.
- `variance_control`: importance sampling, stratification, and Russian-roulette/splitting controls.

## Edge Ledger (machine-parseable)

```yaml
edges: []
```

#tech_glossary
#experimental_ontology
