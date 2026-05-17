---
canonical_name: Capability Scope Key
status: WIP-draft
aliases: []
source_of_truth: []
---

The Capability Scope Key identifies the authority scope where capability ownership and compatibility must resolve.

A scope key is stable across definition validation and runtime activation.
It is not a transient runtime handle and it is not a callback-access mask.
A scope key defines the static authority envelope (for example slot/owner boundary + capability target scope) that
must resolve before [[Runtime Lock]].

Scope-key semantics:

1. Ownership and precedence are evaluated per scope key.
2. Singleton-critical responsibilities must resolve to exactly one effective owner per scope key before activation.
3. Collection-capability responsibilities use the scope key as a partition boundary, with multiplicity rules enforced
   inside that partition.
4. Callback `ctx` allow/deny masks may dynamically narrow access at runtime, but they do not replace scope-key
   ownership resolution.

This key model composes with [[Capability Resolution Key]] and is interpreted by [[Capability Resolution Semantics]].

#glossary
