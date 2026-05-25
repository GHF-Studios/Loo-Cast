---
canonical_name: Solver: FNO Inference Runtime
status: WIP-experimental
aliases:
  - FNO Inference
source_of_truth: []
ontology_experimental: true
ontology_id: solver.ml.fno_inference
node_class: solver
node_types:
  - solver
traits:
  - multi_scale
projection_tags:
  - book:engineering_decision
  - book:computational_representation
activated_modules:
  - core.identity
  - type.solver
  - trait.multi_scale
coordinate_annotations:
  S: cross_scale
  M: learned_operator_inference
  D: deterministic_given_weights
  C: approximate
  K: discrete_tensor_compute
  L: global_spectral_coupling
  R: irreversible_model_evaluation
  T: fixed_network_topology
  I: high_dimensional_io
  O: fully_observable_numeric_state
  G: learned
  Lambda: model_capacity_scaling
  Sigma: distribution_shift_sensitive
  Pi: learned_operator
---

## Identity

- `display_name`: Runtime evaluation solver for trained Fourier neural operators.
- `summary`: Fast surrogate inference path for operator-learning transforms.

## Activated Metadata Modules

### `type.solver`

- `method_family`: neural operator forward inference.
- `order_of_accuracy`: empirical and dataset dependent.
- `stability_characteristics`: bounded by training support and architecture regularization.
- `complexity_model`: dense tensor and FFT-like operations with hardware-dependent scaling.

### `trait.multi_scale`

- `micro_scale`: high-frequency response approximated by retained model capacity.
- `macro_scale`: global response encoded in low-frequency latent modes.
- `bridge_operator`: learned lifting, spectral kernel, and projection layers.

## Declared Invariants

- Inference determinism for fixed weights and deterministic runtime backend.

## Admissible Representations

- tensor
- learned_operator

## Edge Ledger (machine-parseable)

```yaml
edges:
  - rel: eng:recommended_for
    to: transform.operator_learning.fourier_neural_operator
    direction: out
    confidence: 0.87
    when:
      trained_model_available: true
```

## Related Nodes

- [transform.operator_learning.fourier_neural_operator](transform.operator_learning.fourier_neural_operator.md)

#tech_glossary
#experimental_ontology
