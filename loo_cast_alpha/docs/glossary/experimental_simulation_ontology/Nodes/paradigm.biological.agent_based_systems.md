---
canonical_name: Paradigm: Agent-Based Systems
status: WIP-experimental
aliases:
  - Agent-Based Modeling
  - ABM
source_of_truth: []
ontology_experimental: true
ontology_id: paradigm.biological.agent_based_systems
node_class: paradigm
node_types:
  - agent_based_model
traits:
  - multi_scale
projection_tags:
  - book:physical_scale
  - book:mathematical_structure
  - book:engineering_decision
  - book:computational_representation
activated_modules:
  - core.identity
  - trait.multi_scale
coordinate_annotations:
  S: biological_to_cognitive
  M: rule_based_discrete_dynamics
  D: deterministic_or_stochastic
  C: model_dependent
  K: discrete_agents
  L: local_interaction_with_global_patterns
  R: irreversible_path_dependent
  T: dynamic_interaction_topology
  I: high_dimensional_emergent
  O: partially_observable
  G: coarse_population_statistics
  Lambda: agent_count_to_large_limit
  Sigma: phase_transition_and_swarming_regimes
  Pi: agent_and_graph
---

## Identity

- `display_name`: Heterogeneous interacting agents with local behavioral rules.
- `summary`: Framework for emergent collective behavior across ecological, social, and robotic systems.

## Activated Metadata Modules

### `trait.multi_scale`

- `micro_scale`: individual agent update rules.
- `macro_scale`: emergent population-level order parameters.
- `bridge_operator`: moment closures, density fields, and empirical coarse observables.

## Declared Invariants

- Rule-consistency and constraint satisfaction for agent state transitions.

## Admissible Representations

- agent
- graph
- spatial_partition

## Edge Ledger (machine-parseable)

```yaml
edges:
  - rel: math:coarse_grained_by
    to: transform.coarse_graining.ensemble_map
    direction: out
    confidence: 0.74
```

## Related Nodes

- [transform.coarse_graining.ensemble_map](transform.coarse_graining.ensemble_map.md)

#tech_glossary
#experimental_ontology
