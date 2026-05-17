---
canonical_name: Rhai Capability
status: WIP-draft
aliases: [ ]
source_of_truth: [ ]
---

The Rhai Capability is a declaration-level capability API object exposed to scripts through profile-tailored `ctx`
capability-object subgraphs.
It is dynamic/object-based and identified in human-readable terms for script ergonomics and policy gating.

Rhai capabilities are declaration-surface semantics.
Runtime execution/orchestration semantics are carried by runtime-side Rust implementations under the
[[Capability Runtime]] and the [[Runtime Substrate]].
Rhai capability usage participates in the cyclic Rust/Rhai execution loop through callback invocation paths; it is not
an authoring-only surface.
Callback invocation access resolves to effective callback `ctx` path masks through allow/deny policy gating, rather
than implicit inheritance from declaration-entry access, and remains bounded by the [[Scope Envelope]].
Canonical loop/lifecycle/multiplicity semantics are defined in [[Capability]].

#glossary
