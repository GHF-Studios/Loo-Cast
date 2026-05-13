---
canonical_name: Capability Role
status: WIP-draft
aliases: []
source_of_truth: []
---

A Capability Role describes the fundamental archetypal contract shape of a capability under the [[Capability Contract]].
This role is hard-coded as contract metadata and each capability has exactly one role.
Current roles are `input` and `output`.
Input capabilities gather or ingest data without directly mutating canonical game-world state, while output capabilities
present or display state without directly mutating canonical game-world state.
Capabilities are implemented in Rust as dynamic wrapper interfaces over arbitrary Rust functionality, while their
data-oriented instantiations are loaded at runtime through [[USF Instantiation Scripts]].
Additional roles may be introduced later when explicitly locked.
