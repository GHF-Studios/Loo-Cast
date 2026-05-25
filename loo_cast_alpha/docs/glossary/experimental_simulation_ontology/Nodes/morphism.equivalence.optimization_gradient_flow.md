---
canonical_name: Morphism: Optimization Gradient-Flow Equivalence
status: WIP-experimental
aliases:
  - Optimization-Gradient Flow Mapping
source_of_truth: []
ontology_experimental: true
ontology_id: morphism.equivalence.optimization_gradient_flow
node_class: morphism
node_types:
  - morphism
  - equivalence
traits: []
projection_tags:
  - book:transform_morphism
  - book:mathematical_structure
activated_modules:
  - core.identity
  - type.morphism
coordinate_annotations:
  S: abstract_state_space
  M: variational_and_dynamical
  D: deterministic
  C: dissipative
  K: continuous_or_discrete
  L: metric_dependent
  R: irreversible
  T: attractor_forming
  I: medium_to_high_dimensional
  O: fully_observable_state
  G: coarse_or_exact
  Lambda: step_or_time_to_zero
  Sigma: convex_or_nonconvex_regime
  Pi: vector_or_field
---

## Identity

- `display_name`: Equivalence bridge between optimization updates and gradient-flow dynamics.
- `morphism_class`: equivalence.

## Activated Metadata Modules

### `type.morphism`

- `source_pattern`: iterative descent optimization.
- `target_pattern`: gradient-flow ODE/PDE dynamics.
- `equivalence_mode`: exact in select settings, asymptotic in discretized settings.
- `assumptions`: smooth objective and compatible metric structures.

## Mapping Structure

- `source_nodes`: `paradigm.optimization.energy_minimization`.
- `target_nodes`: `paradigm.gradient_flow.dissipative_dynamics`.
- `forward_map`: discrete optimizer step to time-discretized flow map.
- `inverse_map`: flow trajectory interpreted as continuous optimization path.
- `domain_of_validity`: smooth, well-posed, metric-compatible regimes.

## Preserved or Transformed Quantities

- `preserved_quantities`: descent direction and stationary-point set interpretation.
- `lost_quantities`: step-size-specific acceleration artifacts in pure continuous limit.
- `error_or_gap_model`: discretization and metric mismatch error.

## Edge Ledger (machine-parseable)

```yaml
edges:
  - rel: math:maps_from
    to: paradigm.optimization.energy_minimization
    direction: out
    confidence: 0.79
  - rel: math:maps_to
    to: paradigm.gradient_flow.dissipative_dynamics
    direction: out
    confidence: 0.79
```

## Related Nodes

- [paradigm.optimization.energy_minimization](paradigm.optimization.energy_minimization.md)
- [paradigm.gradient_flow.dissipative_dynamics](paradigm.gradient_flow.dissipative_dynamics.md)

#tech_glossary
#experimental_ontology
