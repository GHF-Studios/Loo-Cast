---
canonical_name: Runtime Lock
status: WIP-draft
aliases: []
source_of_truth: []
---

The Runtime Lock is the boundary where validated composition becomes immutable runtime state in
the [[Spacetime Engine]].
After lock, the active mod graph and resolved ownership mappings are fixed, and runtime load or unload mutation is not
part of this model.
Deterministic behavior here is grounded in pre-runtime validation and fixed load ordering through
the [[Modding Runtime]] and the [[Slot Graph Composition]].
