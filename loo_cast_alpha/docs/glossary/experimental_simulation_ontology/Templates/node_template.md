---
canonical_name: Node Template (Experimental)
status: WIP-experimental
aliases: []
source_of_truth: []
ontology_experimental: true
ontology_id: template.node.base
node_class: template
node_types: []
traits: []
projection_tags: []
activated_modules:
  - core.identity
coordinate_annotations: {}
---

Use this template for paradigm, solver, transform, principle, and representation nodes.

## Identity

- `display_name`:
- `summary`:
- `version`:

## Activated Metadata Modules

### `type.PDE` (if active)

- `boundary_conditions`:
- `weak_form`:
- `flux_form`:
- `discretization_options`:

### `type.solver` (if active)

- `method_family`:
- `order_of_accuracy`:
- `stability_characteristics`:
- `complexity_model`:

### `type.transform` (if active)

- `domain_map`:
- `codomain_map`:
- `invertibility`:
- `preserved_quantities`:

### `trait.variational` (if active)

- `functional`:
- `admissible_space`:
- `stationary_condition`:

### `trait.stochastic` (if active)

- `sampling_regime`:
- `estimator_family`:
- `variance_control`:

### `trait.nonlocal` (if active)

- `interaction_kernel`:
- `support_radius`:
- `decay_behavior`:

### `trait.multi_scale` (if active)

- `micro_scale`:
- `macro_scale`:
- `bridge_operator`:

### `trait.structure_preserving` (if active)

- `preserved_invariants`:
- `monitor_strategy`:

### `trait.discontinuity_handling` (if active)

- `shock_sensor`:
- `limiter_family`:
- `entropy_fix_policy`:

### `trait.long_range` (if active)

- `accelerator_family`:
- `error_control`:
- `far_field_model`:

## Declared Invariants

- list invariants or `none`.

## Admissible Representations

- list allowed representations (`grid`, `particle`, `spectral`, `graph`, etc.).

## Edge Ledger (machine-parseable)

```yaml
edges: []
```

## Notes

- optional implementation notes.

#tech_glossary
#experimental_ontology
