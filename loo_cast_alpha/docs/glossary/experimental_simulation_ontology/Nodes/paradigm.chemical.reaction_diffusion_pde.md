---
canonical_name: Paradigm: Reaction-Diffusion PDE Systems
status: WIP-experimental
aliases: []
source_of_truth: []
ontology_experimental: true
ontology_id: paradigm.chemical.reaction_diffusion_pde
node_class: paradigm
node_types:
  - PDE
traits:
  - multi_scale
projection_tags:
  - book:physical_scale
activated_modules:
  - core.identity
  - type.PDE
  - trait.multi_scale
coordinate_annotations:
  S: chemical_to_biological_pattern
  Pi: grid
---

## Identity

- `display_name`: Coupled diffusion and reactive source PDE frameworks.
- `summary`: Pattern-forming and transport-reaction paradigms including Turing mechanisms.

### `type.PDE`

- `boundary_conditions`: no-flux, periodic, inflow/outflow, mixed.
- `weak_form`: diffusion bilinear form plus nonlinear reaction sources.
- `flux_form`: conservative diffusion flux with local reaction terms.
- `discretization_options`: finite volume, finite difference, finite element.

### `trait.multi_scale`

- `micro_scale`: local reaction timescales and steep gradients.
- `macro_scale`: pattern wavelength and domain-scale transport.
- `bridge_operator`: homogenization and filtered diagnostics.

## Edge Ledger (machine-parseable)

```yaml
edges: []
```

#tech_glossary
#experimental_ontology

