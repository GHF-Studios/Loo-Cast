---
canonical_name: Capability Resolution Key
status: WIP-draft
aliases: []
source_of_truth: []
---

A Capability Resolution Key identifies the exact resolution target for capability selection and ownership checks
inside one [[Capability Scope Key]].
Resolution is key-based and deterministic, and singleton-critical responsibilities must resolve to one effective owner
per key before activation.
This model is interpreted through [[Capability Resolution Semantics]] and enforced before [[Runtime Lock]].
Runtime policy may later narrow callback/API usage for resolved targets, but does not rewrite key ownership outcomes.

#glossary
