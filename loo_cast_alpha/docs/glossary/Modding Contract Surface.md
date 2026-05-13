---
canonical_name: Modding Contract Surface
status: WIP-draft
aliases: []
source_of_truth: []
---

The Modding Contract Surface defines mod declaration, lifecycle, dependency, compatibility, and integration boundaries.
Mods may introduce new capabilities and new contract families, plus their implementations.
Mod composition is additive-only: mods do not modify or remove existing registered definitions.
This family composes with other contract families through the [[Contract Surface]], while runtime orchestration is
handled by the [[Modding Runtime]].
