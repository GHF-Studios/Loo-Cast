---
canonical_name: Modding Runtime
status: WIP-draft
aliases: []
source_of_truth: []
---

The Modding Runtime is the runtime orchestration layer for mod loading, dependency resolution, registration, and
lifecycle execution.
It enforces additive composition: mods can add definitions and integrations but do not mutate or remove existing
registered definitions.
It supports introducing new contract families and implementations through declared integration points.
The runtime realizes rules defined by the [[Modding Contract]] and composes with sibling families through
the [[Contract]].
