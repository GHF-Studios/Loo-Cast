---
canonical_name: Solver: Bit-Packed Rule-Table Automata Engine
status: WIP-experimental
aliases:
  - Bit-Packed Cellular Automata
source_of_truth: []
ontology_experimental: true
ontology_id: solver.bio.bitpacked_rule_tables
node_class: solver
node_types:
  - solver
traits: []
projection_tags:
  - book:physical_scale
  - book:computational_representation
activated_modules:
  - core.identity
  - type.solver
coordinate_annotations:
  S: biological_to_emergent
  M: discrete_rule_dynamics
  D: deterministic_or_stochastic_rules
  C: rule_based
  K: discrete_lattice
  L: local_neighborhood_updates
  R: irreversible_update_paths
  T: fixed_or_dynamic_lattice_topology
  I: high_dimensional
  O: fully_observable_numeric_state
  G: coarse_emergent_patterns
  Lambda: lattice_size_to_infinity
  Sigma: critical_rule_set_dependent
  Pi: grid
---

## Identity

- `display_name`: Cellular automata engine with compact bit-level rule evaluation.
- `summary`: High-throughput discrete simulation backend for local-rule biological/emergent models.

## Activated Metadata Modules

### `type.solver`

- `method_family`: synchronous or asynchronous lattice rule updates with bitwise packing.
- `order_of_accuracy`: not applicable in continuous-order sense.
- `stability_characteristics`: governed by rule set and update scheduling policy.
- `complexity_model`: near linear in cell count with strong constant-factor optimization.

## Declared Invariants

- Deterministic reproducibility for fixed rule tables and update order.

## Admissible Representations

- grid
- bitset_state

## Edge Ledger (machine-parseable)

```yaml
edges:
  - rel: eng:recommended_for
    to: paradigm.bio.cellular_automata
    direction: out
    confidence: 0.91
    when:
      large_lattice_discrete_rules: true
```

#tech_glossary
#experimental_ontology
