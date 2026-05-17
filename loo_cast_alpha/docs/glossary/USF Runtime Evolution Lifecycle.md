---
canonical_name: USF Runtime Evolution Lifecycle
status: WIP-draft
aliases: [ ]
source_of_truth: [ ]
---

The USF Runtime Evolution Lifecycle governs runtime progression after the definition lock transition.
Runtime interactions are driven by active capability instances materialized from capabilities established at the
definition lock transition;
these instances
carry logic closures and data defined by declaration scripts.
Execution flows through profile-tailored `ctx` capability-object subgraphs that bound exposed capability objects via
hierarchical graph composition (atomic + composite nodes) and include/exclude path declarations.
When Rhai-declared callbacks are invoked in this phase, runtime enforces resolved effective callback `ctx` path masks
from allow/deny policy resolution.
This runtime flow emits intents that runtime authorities reconcile, commit, and apply as state transitions.
ECS is substrate and execution medium in this flow, not capability-type-template authority.
At this layer, lifecycle semantics are intentionally/unavoidably high-level and still underexplored in deeper
operational detail.

Implementation-facing notes:
[USF Runtime Evolution Lifecycle Notes](USF%20Runtime%20Evolution%20Lifecycle%20Notes.md)
and
[Runtime Intent Reconcile Commit Apply Mapping Notes](Runtime%20Intent%20Reconcile%20Commit%20Apply%20Mapping%20Notes.md)
Dependency-layer and callback-mask policy framing are defined in
[Capability Dependency Layer Notes](Capability%20Dependency%20Layer%20Notes.md).

#glossary
