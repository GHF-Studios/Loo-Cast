---
canonical_name: Transform: Fourier Neural Operator Mapping
status: WIP-experimental
aliases:
  - FNO Transform
source_of_truth: []
ontology_experimental: true
ontology_id: transform.operator_learning.fourier_neural_operator
node_class: transform
node_types:
  - transform
  - operator_learning
traits:
  - multi_scale
projection_tags:
  - book:transform_morphism
  - book:engineering_decision
  - book:computational_representation
activated_modules:
  - core.identity
  - type.transform
  - trait.multi_scale
coordinate_annotations:
  S: cross_scale
  M: learned_operator_map
  D: data_driven
  C: approximate
  K: hybrid
  L: global_spectral_coupling
  R: irreversible_training
  T: latent_function_space_topology
  I: high_dimensional_to_compressed
  O: partially_observable
  G: learned
  Lambda: data_to_infinity_limit
  Sigma: distribution_shift_sensitive
  Pi: learned_operator
---

## Identity

- `display_name`: Learned map between function spaces via Fourier-domain kernels.
- `summary`: Data-driven operator surrogate for PDE family emulation and fast inference.

## Activated Metadata Modules

### `type.transform`

- `domain_map`: input field/state functions and parameter channels.
- `codomain_map`: predicted output fields in target function space.
- `invertibility`: generally non-invertible.
- `preserved_quantities`: model-dependent; can be hard-constrained only with architecture or loss design.

### `trait.multi_scale`

- `micro_scale`: fine-grained local variation represented in spectral modes.
- `macro_scale`: dominant low-frequency/global response behavior.
- `bridge_operator`: learned spectral convolution and lifting/projection layers.

## Declared Invariants

- Approximation consistency on training-domain distributions.

## Admissible Representations

- learned_operator
- tensor
- field

## Edge Ledger (machine-parseable)

```yaml
edges:
  - rel: math:applies_to
    to: paradigm.pde.incompressible_navier_stokes
    direction: out
    confidence: 0.72
  - rel: math:applies_to
    to: paradigm.pde.compressible_flow_with_shocks
    direction: out
    confidence: 0.58
```

## Related Nodes

- [paradigm.pde.incompressible_navier_stokes](paradigm.pde.incompressible_navier_stokes.md)
- [paradigm.pde.compressible_flow_with_shocks](paradigm.pde.compressible_flow_with_shocks.md)

#tech_glossary
#experimental_ontology
