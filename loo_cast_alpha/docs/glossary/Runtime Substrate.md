---
canonical_name: Runtime Substrate
status: WIP-draft
aliases: []
source_of_truth: []
---

The Runtime Substrate is the execution substrate provided by the [[Spacetime Engine]] for running scale-layered
simulation and capability-driven runtime behavior.
It hosts runtime orchestration through [[Capability Runtime]], [[Modding Runtime]], and [[USF Runtime]].
ECS is the underlying execution/data medium in this substrate, while semantic concept authority remains
contract-defined.
Deterministic activation and fixed runtime composition boundaries are enforced through [[Runtime Lock]].
