---
canonical_name: USF Instance Graph
status: WIP-draft
aliases: []
source_of_truth: []
---

The USF Instance Graph is the active structured set of instantiated USF objects and their relations.
It is organized as a 71-scale stack of canonical [[Scale Slice]]s indexed by [[Scale]].
In the runtime substrate, scale slices are simulated in parallel and composed through capability outputs.
Core scale-system elements in this graph include [[Scale Definition]] declarations, active scale slices, and per-slice
[[Scale Realizer]] bindings.
Activation validity is constrained by contract-level invariants including [[Scale Realizer Cardinality]].

#glossary
