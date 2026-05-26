---
canonical_name: Morphism Node Template (Experimental)
status: WIP-experimental
aliases: []
source_of_truth: []
ontology_experimental: true
ontology_id: template.node.morphism
node_class: morphism
node_types:
  - morphism
traits: []
projection_tags:
  - book:transform_morphism
activated_modules:
  - core.identity
  - type.morphism
coordinate_annotations: {}
---

Use this template when a relation is too structured to remain a simple edge.

## Identity

- `display_name`:
- `morphism_class`: (`duality`, `equivalence`, `analogy`, `limit`, `coarse_graining`, `reduction`)

## Activated Metadata Modules

### `type.morphism`

- `source_pattern`:
- `target_pattern`:
- `equivalence_mode`: (`exact`, `asymptotic`, `approximate`, `interpretive`)
- `assumptions`:

## Mapping Structure

- `source_nodes`:
- `target_nodes`:
- `forward_map`:
- `inverse_map`:
- `domain_of_validity`:

## Preserved or Transformed Quantities

- `preserved_quantities`:
- `lost_quantities`:
- `error_or_gap_model`:

## Edge Ledger (machine-parseable)

```yaml
edges: []
```

## Notes

- optional proof sketch or implementation interpretation.

#tech_glossary
#experimental_ontology
