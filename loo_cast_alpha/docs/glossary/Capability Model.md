---
canonical_name: Capability Model
status: draft
aliases: [ ]
source_of_truth: [ ]
---

The Capability Model allows defining high-level, extensible capabilities in the [[Spacetime Engine]] for domains such as
math, physics, rendering, agency, player and multiplayer systems, AI integration, pathfinding, and other simulation
abstractions.
A capability is a generic cross-scale backend composed of capability channels, with each channel mapped to a single
scale.
The [[Capability Model]] integrates with the [[Contract Surface]], which defines contract families, compatibility
boundaries, and lifecycle expectations.
Mods may introduce new capabilities and add, replace, or extend channels through native Rust code and engine-level
plugins, effectively extending the [[Contract Surface]] through new contract definitions.
