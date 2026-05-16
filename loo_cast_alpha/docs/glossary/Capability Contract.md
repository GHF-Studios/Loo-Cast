---
canonical_name: Capability Contract Family
status: WIP-draft
aliases:
  - Capability Contract
  - Capability Contract Surface
source_of_truth: [ ]
---

The Capability Contract Family defines what capability contracts must declare and satisfy.
Capabilities implement this family and expose [[Scaled Capability Channel]]s as scale-specific execution faces whose
required/allowed shape is derived from that family.
In declaration scripts, capabilities appear as [[Rhai Capability]] API objects surfaced through profile-tailored `ctx`
capability-object subgraphs.
The `ctx` graph is hierarchical (atomic capability nodes + composite/category nodes), with include/exclude path
declarations
controlling exposed subgraphs.
Runtime executes behavior through materialized capability instances that bind those declared surfaces.
Runtime orchestration and execution consume runtime capability implementations/channels and typed runtime artifacts.
Dependency semantics are layered: provider dependencies resolve through mod/runtime ownership, while declaration
dependencies resolve through profile-scoped `ctx` path access requirements.
This [[Contract Family]] defines compatibility and boundary rules for capability implementations within the [[Contract]]
through declared [[Capability Scope Key]]s and [[Capability Resolution Semantics]].
Scale compatibility declarations are defined by the [[Scale Contract]] through [[Scale Support]] over [[Scale]]
coordinates.
Runtime orchestration and channel coordination are handled by the [[Capability Runtime]].
Canonical dependency and seam rules are documented in
[Capability Dependency Layer Notes](Capability%20Dependency%20Layer%20Notes.md).

#glossary
