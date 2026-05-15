---
canonical_name: Capability Runtime
status: WIP-draft
aliases: []
source_of_truth: []
---

The Capability Runtime is the runtime orchestration layer for capabilities in the [[Spacetime Engine]].
It handles dynamic discovery, registration, coordination, and execution routing for capability implementations.
Declaration scripts consume capability objects through profile-tailored `ctx` API subgraphs; runtime concept instances
execute closure logic against the realized capability implementations.
`ctx` subgraphs are composed from hierarchical API graph nodes (atomic + composite) via include/exclude path rules and
can dynamically open/close by runtime policy.
Capability implementations expose [[Scaled Capability Channel]] structures as per-scale execution paths for that
runtime execution.
The runtime realizes contracts defined by the [[Capability Contract]] and coordinates
with [[Observer-Relative Simulation]].

Implementation-facing notes:

- [USF Contract Runtime Boundary Notes](USF%20Contract%20Runtime%20Boundary%20Notes.md)
- [Rhai Generic Dispatch Policy Notes](Rhai%20Generic%20Dispatch%20Policy%20Notes.md)

#glossary
