---
canonical_name: Metadata Activation Graph (Experimental)
status: WIP-experimental
aliases: []
source_of_truth: []
---

Flat metadata is forbidden in this subtree.
Metadata modules activate conditionally from `node_types` and `traits`.

Activation model:

- `node_types` activate type modules.
- `traits` activate trait modules.
- Modules can depend on other modules.
- A node must not carry fields from inactive modules.

Module registry (v0):

| Module | Trigger | Requires | Required fields |
| --- | --- | --- | --- |
| `core.identity` | always | none | `ontology_id`, `node_class`, `status` |
| `type.PDE` | `node_types` contains `PDE` | `core.identity` | `boundary_conditions`, `weak_form`, `flux_form`, `discretization_options` |
| `type.stochastic_process` | `node_types` contains `stochastic_process` | `core.identity` | `noise_type`, `generator`, `invariant_measure`, `markovianity` |
| `type.solver` | `node_types` contains `solver` | `core.identity` | `method_family`, `order_of_accuracy`, `stability_characteristics`, `complexity_model` |
| `type.transform` | `node_types` contains `transform` | `core.identity` | `domain_map`, `codomain_map`, `invertibility`, `preserved_quantities` |
| `type.morphism` | `node_types` contains `morphism` | `core.identity` | `source_pattern`, `target_pattern`, `equivalence_mode`, `assumptions` |
| `trait.variational` | `traits` contains `variational` | `core.identity` | `functional`, `admissible_space`, `stationary_condition` |
| `trait.stochastic` | `traits` contains `stochastic` | `core.identity` | `sampling_regime`, `estimator_family`, `variance_control` |
| `trait.nonlocal` | `traits` contains `nonlocal` | `core.identity` | `interaction_kernel`, `support_radius`, `decay_behavior` |
| `trait.multi_scale` | `traits` contains `multi_scale` | `core.identity` | `micro_scale`, `macro_scale`, `bridge_operator` |
| `trait.structure_preserving` | `traits` contains `structure_preserving` | `type.solver` or `type.PDE` | `preserved_invariants`, `monitor_strategy` |
| `trait.discontinuity_handling` | `traits` contains `discontinuity_handling` | `type.PDE` or `type.solver` | `shock_sensor`, `limiter_family`, `entropy_fix_policy` |
| `trait.long_range` | `traits` contains `long_range` | `type.solver` or `type.PDE` | `accelerator_family`, `error_control`, `far_field_model` |

Validation rules:

1. If a trigger is present, the corresponding module fields are mandatory.
2. If module fields appear but trigger is absent, validation fails.
3. Module dependency graph must be acyclic.
4. A deprecated module can remain only when `status: deprecated`.
5. Every declared `trait` must map to a registered `trait.*` module.
6. Unknown `node_types` are allowed only when they are declared under open-world policy in [Controlled Vocabulary Registry](../Indexes/controlled_vocabulary_registry.md) and carry no undeclared `type.*` module fields.

Example dependency snippet:

```yaml
activated_modules:
  - core.identity
  - type.PDE
  - trait.multi_scale
  - trait.structure_preserving
```

The node must then include fields for all four modules and no extra inactive-module fields.

#tech_glossary
#experimental_ontology
