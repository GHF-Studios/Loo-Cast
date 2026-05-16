---
canonical_name: USF Instance Graph
status: WIP-draft
aliases: []
source_of_truth: []
---

The USF Instance Graph is the active structured set of runtime-materialized USF capability instances and their
relations.
These instances are derived from capabilities produced by instantiation scripts and established at Runtime Lock.
It is organized as a 71-scale stack of canonical [[Scale Slice]]s indexed by [[Scale]].
In the runtime substrate, scale slices are simulated in parallel and composed through capability outputs.
Core scale-system elements in this graph include [[Scale Definition]] declarations, active scale slices, and per-slice
[[Scale Realizer]] bindings.
Activation validity is constrained by contract-level invariants including [[Scale Realizer Cardinality]].
Capability lifecycle/loop semantics and multiplicity class definitions are canonicalized in [[Capability]].

Multiplicity enforcement in this graph:

1. Slot-singleton capabilities:
   one `Scale` capability and one `Scale Realizer` capability per occupied scale slot.
2. Scale-collection capabilities:
   `Phenomenon` and `Metric` capabilities are keyed per scale, with many allowed and at least one required per scale.

#glossary
