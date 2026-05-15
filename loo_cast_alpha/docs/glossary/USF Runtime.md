---
canonical_name: USF Runtime
status: WIP-draft
aliases: []
source_of_truth: []
---

The USF Runtime is the default first-party runtime implementation of the [[USF Contract]] family in `core_mod`.
It realizes one active USF implementation and composes with the [[Capability Runtime]] and the [[Modding Runtime]].
It invokes declaration-surface entrypoints with profile-tailored `ctx` capability-object subgraphs during activation,
then executes the resulting runtime concept instances.
These subgraphs come from hierarchical API graph composition (atomic + composite nodes) with include/exclude
path declarations and runtime open/close behavior.
It executes contract-defined behavior rather than defining the contract itself.

Implementation-facing notes:

- [USF Contract Runtime Boundary Notes](USF%20Contract%20Runtime%20Boundary%20Notes.md)
- [USF Runtime Evolution Lifecycle Notes](USF%20Runtime%20Evolution%20Lifecycle%20Notes.md)
- [USF Math Raw Model Foundation Notes](USF%20Math%20Raw%20Model%20Foundation%20Notes.md)
- [Rhai Generic Dispatch Policy Notes](Rhai%20Generic%20Dispatch%20Policy%20Notes.md)
- [Rhai Value Semantics and AccessCell Notes](Rhai%20Value%20Semantics%20and%20AccessCell%20Notes.md)
- [USF Position Stack and Overflow Policy Notes](USF%20Position%20Stack%20and%20Overflow%20Policy%20Notes.md)

#glossary
