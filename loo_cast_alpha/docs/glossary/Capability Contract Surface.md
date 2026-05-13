---
canonical_name: Capability Contract Surface
status: WIP-draft
aliases: [ ]
source_of_truth: [ ]
---

The Capability Contract Surface defines what a capability contract must declare and satisfy.
Capabilities implement the capability contract and expose [[Scaled Capability Channel]]s as scale-specific execution
faces whose required shape is derived from that contract.
For each capability and each global scale, support must be declared explicitly.
Each capability-scale pair has exactly one support state: `supported` or `unsupported`.
This family defines compatibility and boundary rules for capability implementations within the [[Contract Surface]].
Runtime orchestration and channel coordination are handled by the [[Capability Runtime]].
