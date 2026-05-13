---
canonical_name: Modding Model
status: WIP-draft
aliases: []
source_of_truth: []
---

The Modding Model is the runtime orchestration layer for mod loading, dependency resolution, registration, and lifecycle
execution.
It enforces additive composition: mods can add definitions and integrations but do not mutate or remove existing
registered definitions.
It supports introducing new contract families and implementations through declared integration points.
The model realizes rules defined by the [[Modding Contract Surface]] and composes with sibling families through
the [[Contract Surface]].
