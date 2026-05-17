---
canonical_name: Scope Envelope
status: WIP-draft
aliases: []
source_of_truth: []
---

The Scope Envelope is the hard maximum API/context graph scope allowed by script type/profile/template contract.
Runtime policy can dynamically narrow access and can later re-open previously narrowed paths, but this movement is
always inside the same contract envelope.
Runtime narrowing/re-opening cannot create scope that was never part of the envelope.

This applies to both domain-specific capability surfaces and explicitly declared global rudimentary surfaces.

See also:

- [[Capability Scope Key]]
- [[Capability Resolution Semantics]]
- [[Global Capability Surface]]
- [[Runtime Lock]]

#glossary
