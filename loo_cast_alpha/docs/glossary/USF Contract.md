---
canonical_name: USF Contract Family
status: WIP-draft
aliases:
  - USF Contract
source_of_truth: [ ]
---

The USF Contract Family defines foundational simulation principles and contract-level structure in core_engine.
It defines both capability typing and the scale system as core constituents.
In that system, [[Scale]] is the canonical semantic coordinate, [[Scale Definition]] declares what is meaningful at
that coordinate, [[Scale Support]] declares capability support at that coordinate, and [[Scale Realizer]] defines
per-slice semantic realization behavior.
Scale declaration and compatibility rules are defined by the [[Scale Contract]].
The default first-party runtime counterpart is [[USF Runtime]] in `core_mod`; alternative counterparts are valid when
they satisfy this [[Contract Family]].

Implementation-facing notes: [USF Contract Runtime Boundary Notes](USF%20Contract%20Runtime%20Boundary%20Notes.md)

#glossary
