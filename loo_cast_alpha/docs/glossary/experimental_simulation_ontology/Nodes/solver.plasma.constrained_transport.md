---
canonical_name: Solver: Constrained Transport MHD
status: WIP-experimental
aliases:
  - CT MHD
source_of_truth: []
ontology_experimental: true
ontology_id: solver.plasma.constrained_transport
node_class: solver
node_types:
  - solver
traits:
  - structure_preserving
projection_tags:
  - book:physical_scale
  - book:engineering_decision
activated_modules:
  - core.identity
  - type.solver
  - trait.structure_preserving
coordinate_annotations:
  S: plasma_to_astrophysical
  M: conservative_mhd_stencil
  D: deterministic
  C: divergence_free_magnetic_structure
  K: discrete_grid
  L: local_stencil_with_face_coupling
  R: irreversible_numerical
  T: mesh_topology_sensitive
  I: high_dimensional
  O: fully_observable_numeric_state
  G: closure_optional
  Lambda: dx_dt_to_zero
  Sigma: cfl_limited
  Pi: grid
---

## Identity

- `display_name`: Divergence-preserving magnetic update strategy in MHD solvers.
- `summary`: Enforces discrete `div(B)=0` constraints through topology-aware flux updates.

## Activated Metadata Modules

### `type.solver`

- `method_family`: finite-volume constrained transport with edge-centered electromotive updates.
- `order_of_accuracy`: reconstruction and time integrator dependent.
- `stability_characteristics`: robust when coupled with compatible Riemann fluxes and CT discretization.
- `complexity_model`: local flux updates with face/edge coupling overhead.

### `trait.structure_preserving`

- `preserved_invariants`: low magnetic divergence and conservative variable consistency.
- `monitor_strategy`: divergence norms and magnetic-energy diagnostics.

## Declared Invariants

- Controlled discrete divergence of magnetic field.

## Admissible Representations

- grid

## Edge Ledger (machine-parseable)

```yaml
edges:
  - rel: eng:recommended_for
    to: paradigm.plasma.ideal_mhd
    direction: out
    confidence: 0.9
    when:
      divergence_control_priority: true
```

#tech_glossary
#experimental_ontology
