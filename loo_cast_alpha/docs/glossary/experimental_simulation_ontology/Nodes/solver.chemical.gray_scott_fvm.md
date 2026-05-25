---
canonical_name: Solver: Gray-Scott Finite Volume Reaction-Diffusion
status: WIP-experimental
aliases:
  - Gray-Scott FVM
source_of_truth: []
ontology_experimental: true
ontology_id: solver.chemical.gray_scott_fvm
node_class: solver
node_types:
  - solver
traits:
  - multi_scale
projection_tags:
  - book:physical_scale
  - book:engineering_decision
activated_modules:
  - core.identity
  - type.solver
  - trait.multi_scale
coordinate_annotations:
  S: chemical_to_meso_pattern
  M: pde_fvm
  D: deterministic
  C: weakly_dissipative
  K: discrete_grid
  L: local_stencil
  R: irreversible_reaction_dynamics
  T: dynamic_pattern_topology
  I: high_dimensional
  O: fully_observable_numeric_state
  G: coarse_grained_patterns
  Lambda: dx_dt_to_zero
  Sigma: bifurcation_regime_sensitive
  Pi: grid
---

## Identity

- `display_name`: Finite-volume reaction-diffusion solver for Gray-Scott style systems.
- `summary`: Structured PDE solver for Turing-pattern-capable chemical dynamics.

## Activated Metadata Modules

### `type.solver`

- `method_family`: conservative finite-volume diffusion-reaction updates.
- `order_of_accuracy`: stencil and time integrator dependent.
- `stability_characteristics`: stiff reaction terms often require implicit or IMEX treatment.
- `complexity_model`: grid-cell local updates plus boundary processing.

### `trait.multi_scale`

- `micro_scale`: local reaction kinetics and steep gradients.
- `macro_scale`: pattern wavelengths and global morphology statistics.
- `bridge_operator`: scale-separated diagnostics and filtered field analysis.

## Declared Invariants

- Nonnegativity and boundedness under controlled step and reaction parameters.

## Admissible Representations

- grid

## Edge Ledger (machine-parseable)

```yaml
edges:
  - rel: eng:recommended_for
    to: paradigm.chemical.reaction_diffusion_pde
    direction: out
    confidence: 0.86
    when:
      turing_pattern_analysis: true
```

#tech_glossary
#experimental_ontology
