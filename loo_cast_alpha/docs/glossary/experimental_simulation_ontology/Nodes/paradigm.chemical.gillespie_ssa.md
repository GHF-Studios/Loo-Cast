---
canonical_name: Paradigm: Stochastic Chemical Kinetics (SSA)
status: WIP-experimental
aliases:
  - Gillespie SSA
source_of_truth: []
ontology_experimental: true
ontology_id: paradigm.chemical.gillespie_ssa
node_class: paradigm
node_types:
  - stochastic_process
traits: []
projection_tags:
  - book:physical_scale
  - book:mathematical_structure
  - book:engineering_decision
activated_modules:
  - core.identity
  - type.stochastic_process
coordinate_annotations:
  S: molecular_chemical
  M: jump_process
  D: stochastic
  C: probability_conserving
  K: discrete_state
  L: local_reaction_channels
  R: irreversible_sample_paths
  T: dynamic_state_graph
  I: moderate_to_high_dimensional
  O: partially_observable
  G: mean_field_or_master_equation_bridge
  Lambda: system_size_to_infinity
  Sigma: metastable_and_bistable
  Pi: event_driven
---

## Identity

- `display_name`: Exact stochastic simulation of reaction-channel events.
- `summary`: Canonical event-driven model for chemically reacting systems with intrinsic noise.

## Activated Metadata Modules

### `type.stochastic_process`

- `noise_type`: intrinsic reaction noise from discrete-event randomness.
- `generator`: chemical master equation generator.
- `invariant_measure`: model-dependent stationary distributions.
- `markovianity`: Markov jump process.

## Declared Invariants

- Nonnegativity and discrete-count consistency.

## Admissible Representations

- event_driven
- state_graph

## Edge Ledger (machine-parseable)

```yaml
edges:
  - rel: eng:recommended_solver
    to: solver.stochastic.tau_leaping
    direction: out
    confidence: 0.78
    when:
      high_event_rate: true
      exact_event_order_not_required: true
```

## Related Nodes

- [solver.stochastic.tau_leaping](solver.stochastic.tau_leaping.md)

#tech_glossary
#experimental_ontology
