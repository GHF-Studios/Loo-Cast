---
canonical_name: USF Instantiation Scripts
status: WIP-draft
aliases: []
source_of_truth: []
---

The USF Instantiation Scripts are the declaration-centric authoring surface under [[Loo Cast]] for defining
singleton-like Rhai declaration-type objects for contract-defined USF concept kinds from the [[USF Contract]].
Executing these declaration entrypoints with profile-tailored `ctx` capability-object subgraphs materializes full
declaration objects
that include structured data plus logic closures.
The `ctx` graph is hierarchical (atomic capability nodes + composite/category nodes) and filtered through
include/exclude path declarations for each profile.
Runtime activation then materializes active USF concept instances from those frozen declarations into the
[[USF Instance Graph]].
Capabilities are exposed to scripts as declaration-level API objects through `ctx`; runtime behavior is executed by the
resulting concept instances.
In practice this makes scripts object descriptors first, effectively the closest thing to game-asset authoring in this
project’s architecture.
This surface is governed by the [[USF Definition Lifecycle]].

Implementation-facing notes:

- [USF Instantiation Script Profile Notes](USF%20Instantiation%20Script%20Profile%20Notes.md)
- [USF Math Raw Model Foundation Notes](USF%20Math%20Raw%20Model%20Foundation%20Notes.md)
- [Rhai Generic Dispatch Policy Notes](Rhai%20Generic%20Dispatch%20Policy%20Notes.md)
- [Rhai Value Semantics and AccessCell Notes](Rhai%20Value%20Semantics%20and%20AccessCell%20Notes.md)

#glossary
