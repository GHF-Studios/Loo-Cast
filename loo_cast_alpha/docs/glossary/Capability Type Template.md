---
canonical_name: Capability Type Template
status: WIP-draft
aliases: [ ]
source_of_truth: [ ]
---

The Capability Type Template is the Rust-side template authority for one [[Capability Type]].

It is defined through Rust contracts/traits and registration wiring, and it constrains how capability declarations are
validated and materialized.

Scripts do not define templates; scripts define [[Capability Declaration]]s that target a capability type template.

At the current draft stage, the active template set is intentionally small and fixed (for example `Scale`, `Metric`,
`Phenomenon`, and `Scale Realizer`).

#glossary
