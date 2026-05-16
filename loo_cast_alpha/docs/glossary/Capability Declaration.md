---
canonical_name: Capability Declaration
status: WIP-draft
aliases: [ ]
source_of_truth: [ ]
---

The Capability Declaration is the singleton-like script-produced declaration payload for one [[Capability Type]].

It is data-first (POD-oriented) with declared behavior payload and metadata shaped by a target
[[Capability Type Template]].
When callbacks are declared, callback access policy inputs must resolve into effective callback `ctx` path masks before
[[Runtime Lock]].

One script file defines exactly one capability declaration.
At the definition lock transition, each validated capability declaration is promoted into a [[Capability]].
Canonical lifecycle, Rust/Rhai loop, and multiplicity semantics are defined in [[Capability]].

Workflows should orchestrate lifecycle around materialized runtime artifacts and contract boundaries, not raw
script-engine internals.

#glossary
