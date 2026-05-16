---
canonical_name: Scaled Capability Channel
status: WIP-draft
aliases: []
source_of_truth: []
---

A Scaled Capability Channel is a scale-scoped execution face of a capability implementation, orchestrated by
the [[Capability Runtime]].
Channel availability across scales is governed by [[Scale Support]], and execution binds to active [[Scale Slice]]
context.
Declaration scripts access channel-relevant capability objects through profile-tailored `ctx` capability-object
subgraphs, and
runtime materialized capability instances execute closure logic through those resolved channels.
Those subgraphs are derived from hierarchical API graph composition (atomic + composite nodes) with include/exclude
path declarations.
It is governed through the capability contract context inside the [[Contract]], without being its own standalone
contract family.

#glossary
