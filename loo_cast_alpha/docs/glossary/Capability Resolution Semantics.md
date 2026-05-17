---
canonical_name: Capability Resolution Semantics
status: WIP-draft
aliases: []
source_of_truth: []
---

Capability Resolution Semantics defines how capability ownership and selection resolve for one
([[Capability Scope Key]], [[Capability Resolution Key]]) pair.

Resolution is intentionally two-layered and the layers must not be conflated:

1. Static ownership resolution (pre-lock):
   composition/validation resolves effective owner(s) with deterministic precedence rules.
   Singleton-critical responsibilities must resolve to exactly one effective owner before [[Runtime Lock]].
   Collection responsibilities may resolve to multiple owners only where multiplicity policy allows it.
2. Dynamic API-surface resolution (runtime):
   callback `ctx` masks and runtime policy can narrow or deny runtime access to already-resolved capabilities.
   This layer may reject invocation paths, but it does not retroactively change pre-lock ownership outcomes.

Hard-fail conditions include unresolved required owner, singleton conflicts, invalid multiplicity, or denied required
callback paths after policy resolution.
No implicit fallback owner is introduced.

This split keeps lock-time authority deterministic while still allowing dynamic/nested capability-API graph gating at
runtime.

#glossary
