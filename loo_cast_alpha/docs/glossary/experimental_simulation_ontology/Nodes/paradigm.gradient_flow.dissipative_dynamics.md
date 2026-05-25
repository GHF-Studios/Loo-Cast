---
canonical_name: Paradigm: Gradient Flow Dissipative Dynamics
status: WIP-experimental
aliases:
  - Dissipative Gradient Flow
source_of_truth: []
ontology_experimental: true
ontology_id: paradigm.gradient_flow.dissipative_dynamics
node_class: paradigm
node_types:
  - ODE_system
traits:
  - variational
projection_tags:
  - book:mathematical_structure
  - book:transform_morphism
activated_modules:
  - core.identity
  - trait.variational
coordinate_annotations:
  S: abstract_state_space
  M: ODE_and_variational
  D: deterministic
  C: entropy_or_energy_dissipative
  K: continuous
  L: local_metric_dependent
  R: irreversible
  T: attractor_forming
  I: dimensionality_dependent
  O: fully_or_partially_observable
  G: coarse_or_exact
  Lambda: time_step_to_zero
  Sigma: stable_or_bifurcating
  Pi: vector_or_field
---

## Identity

- `display_name`: Dynamics following steepest descent in a chosen metric.
- `summary`: Time-evolution formulation equivalent to optimization flow under selected assumptions.

## Activated Metadata Modules

### `trait.variational`

- `functional`: Lyapunov or free-energy functional.
- `admissible_space`: metric space with differentiable or generalized gradient structure.
- `stationary_condition`: zero-gradient equilibrium condition.

## Declared Invariants

- Non-increasing Lyapunov functional along trajectories.

## Admissible Representations

- vector
- field

## Edge Ledger (machine-parseable)

```yaml
edges:
  - rel: math:equivalence_instance
    to: morphism.equivalence.optimization_gradient_flow
    direction: out
    confidence: 0.79
```

## Related Nodes

- [morphism.equivalence.optimization_gradient_flow](morphism.equivalence.optimization_gradient_flow.md)

#tech_glossary
#experimental_ontology
