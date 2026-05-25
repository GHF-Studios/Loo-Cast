---
canonical_name: Paradigm: Chemical Master Equation
status: WIP-experimental
aliases:
  - CME
source_of_truth: []
ontology_experimental: true
ontology_id: paradigm.chemical.master_equation
node_class: paradigm
node_types:
  - stochastic_process
traits: []
projection_tags:
  - book:physical_scale
activated_modules:
  - core.identity
  - type.stochastic_process
coordinate_annotations:
  S: molecular_chemical
  Pi: state_graph
---

## Identity

- `display_name`: Probability evolution over discrete reaction-count states.
- `summary`: Exact stochastic kinetics framework for low-copy-number chemistry.

### `type.stochastic_process`

- `noise_type`: intrinsic reaction noise.
- `generator`: reaction-channel transition generator.
- `invariant_measure`: model-dependent stationary distributions.
- `markovianity`: Markov jump process.

## Edge Ledger (machine-parseable)

```yaml
edges: []
```

#tech_glossary
#experimental_ontology

