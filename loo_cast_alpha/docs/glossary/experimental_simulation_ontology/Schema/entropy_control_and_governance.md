---
canonical_name: Entropy Control and Governance (Experimental)
status: WIP-experimental
aliases: []
source_of_truth: []
---

Graph richness is allowed.
Graph drift is not.

Entropy controls:

1. Controlled vocabulary for `node_types` and `traits`.
2. Namespace registry for edges with meta-class ownership.
3. Activation-graph validation for metadata modules.
4. Stable `ontology_id` format.
5. Deprecation and replacement semantics.

`ontology_id` convention:

- Pattern: `<node_class>.<domain>.<slug>`
- Examples:
  - `paradigm.pde.incompressible_navier_stokes`
  - `solver.nbody.fast_multipole_method`
  - `morphism.duality.particle_field`

Allowed status values:

- `WIP-experimental`
- `active`
- `deprecated`

Deprecation policy:

- A deprecated node keeps its `ontology_id`.
- Add outbound `gov:replaced_by` edge.
- Projection books should stop using deprecated nodes except in historical appendices.

Edge registration policy:

1. New edge keys must declare namespace and meta-class.
2. New edge key must define source/target class constraints.
3. New edge key must include one example relation instance.
4. If edge semantics exceed binary constraints, escalate to morphism or relation node.

Review checklist for each new node:

1. Do activated modules exactly match declared types and traits?
2. Are all module-required fields present?
3. Are edges namespaced and semantically valid for source/target classes?
4. Is the node indexed in at least one projection book?
5. Are optional coordinates present only when meaningful?

This governance file is intentionally strict to keep exploratory growth structured.

#tech_glossary
#experimental_ontology
