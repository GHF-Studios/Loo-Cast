---
canonical_name: Capability Contract
status: WIP-draft
aliases:
  - Capability Contract Surface
source_of_truth: [ ]
---

The Capability Contract defines what a capability contract must declare and satisfy.
Capabilities implement the capability contract and expose [[Scaled Capability Channel]]s as scale-specific execution
faces whose required shape is derived from that contract.
This contract family defines compatibility and boundary rules for capability implementations within the [[Contract]].
Scale compatibility declarations are defined by the [[Scale Contract]] through [[Scale]]s.
Runtime orchestration and channel coordination are handled by the [[Capability Runtime]].
