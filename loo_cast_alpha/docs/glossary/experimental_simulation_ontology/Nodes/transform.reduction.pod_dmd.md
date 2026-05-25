---
canonical_name: Transform: POD-DMD State-Space Reduction
status: WIP-experimental
aliases:
  - POD/DMD Reduction
source_of_truth: []
ontology_experimental: true
ontology_id: transform.reduction.pod_dmd
node_class: transform
node_types:
  - transform
  - model_reduction
traits:
  - multi_scale
projection_tags:
  - book:transform_morphism
  - book:engineering_decision
  - book:computational_representation
activated_modules:
  - core.identity
  - type.transform
  - trait.multi_scale
coordinate_annotations:
  S: cross_scale
  M: projection_and_modal_decomposition
  D: deterministic_or_data_driven
  C: approximate_conservation
  K: hybrid
  L: global_basis
  R: irreversible_truncation
  T: reduced_state_topology
  I: dimension_reduced
  O: partially_observable
  G: learned_or_statistical
  Lambda: mode_count_to_full_state
  Sigma: stable_in_well_separated_spectra
  Pi: vector_or_tensor
---

## Identity

- `display_name`: Projection-based reduced-order modeling using POD and DMD families.
- `summary`: Compresses high-dimensional simulation states into low-dimensional latent dynamics.

## Activated Metadata Modules

### `type.transform`

- `domain_map`: high-dimensional state or trajectory data.
- `codomain_map`: reduced modal coordinates and reduced operators.
- `invertibility`: approximate and truncated; full invertibility not guaranteed.
- `preserved_quantities`: selected low-order statistics and dominant coherent structures.

### `trait.multi_scale`

- `micro_scale`: fine-scale state evolution and local fluctuations.
- `macro_scale`: dominant modal dynamics and coarse observables.
- `bridge_operator`: projection and reconstruction operators.

## Declared Invariants

- Best-approximation optimality in selected projection norms.

## Admissible Representations

- vector
- tensor
- latent_reduced_state

## Edge Ledger (machine-parseable)

```yaml
edges:
  - rel: math:applies_to
    to: paradigm.pde.incompressible_navier_stokes
    direction: out
    confidence: 0.81
  - rel: math:applies_to
    to: paradigm.continuum.solid_mechanics_pde
    direction: out
    confidence: 0.76
```

## Related Nodes

- [paradigm.pde.incompressible_navier_stokes](paradigm.pde.incompressible_navier_stokes.md)
- [paradigm.continuum.solid_mechanics_pde](paradigm.continuum.solid_mechanics_pde.md)

#tech_glossary
#experimental_ontology
