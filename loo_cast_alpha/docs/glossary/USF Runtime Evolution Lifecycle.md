---
canonical_name: USF Runtime Evolution Lifecycle
status: WIP-draft
aliases: [ ]
source_of_truth: [ ]
---

The USF Runtime Evolution Lifecycle governs runtime progression after definition freeze.
Runtime interactions are driven by active concept instances materialized from frozen declarations; these instances
carry logic closures and data defined by declaration scripts.
Execution flows through profile-tailored `ctx` capability-object subgraphs that bound exposed capability objects via
hierarchical graph composition (atomic + composite nodes) and include/exclude path declarations.
This runtime flow emits intents that runtime authorities reconcile, commit, and apply as state transitions.
ECS is substrate and execution medium in this flow, not semantic concept authority.
At this layer, lifecycle semantics are intentionally/unavoidably high-level and still underexplored in deeper
operational detail.

Implementation-facing notes: [USF Runtime Evolution Lifecycle Notes](USF%20Runtime%20Evolution%20Lifecycle%20Notes.md)

#glossary
