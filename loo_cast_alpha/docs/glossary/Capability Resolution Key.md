---
canonical_name: Capability Resolution Key
status: WIP-draft
aliases: []
source_of_truth: []
---

A Capability Resolution Key identifies the exact resolution target for capability selection and ownership checks.
Resolution is key-based and deterministic, and singleton-critical responsibilities must resolve to one effective owner
per key before activation.
This model is used by runtime resolution surfaces such as [[Capability Runtime]], [[Slot Graph Composition]],
and [[Runtime Lock]], and is interpreted through [[Capability Resolution Semantics]].

#glossary
