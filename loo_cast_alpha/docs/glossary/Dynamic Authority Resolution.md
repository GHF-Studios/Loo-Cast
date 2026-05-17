---
canonical_name: Dynamic Authority Resolution
status: WIP-draft
aliases: []
source_of_truth: []
---

Dynamic Authority Resolution means effective authority is resolved at runtime by lifecycle context and operation,
rather than fixed as one global static fact.
Definition authority, runtime authority, and output/application authority stay distinct, and active authority is
interpreted per phase.

Runtime policy can narrow or re-open access per context, but cannot expand authority beyond the [[Scope Envelope]].
Globally scoped rudimentary capability surfaces may be active across many contexts, but this does not imply global
domain-state authority.

See also:

- [[Capability Role]]
- [[Scope Envelope]]
- [[Global Capability Surface]]
- [[USF Runtime Evolution Lifecycle]]

#glossary
