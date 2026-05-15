---
canonical_name: Modding Contract
status: WIP-draft
aliases:
  - Modding Contract Surface
source_of_truth: []
---

The Modding Contract defines mod declaration, life-cycle, dependency, compatibility, and integration boundaries.
Mods may introduce new capabilities and new contract families, plus their implementations.
Mod composition is additive-only: mods do not modify or remove existing registered definitions.
This [[Contract Family]] composes with other contract families through the [[Contract]], while runtime orchestration is
handled by the [[Modding Runtime]].

#glossary
