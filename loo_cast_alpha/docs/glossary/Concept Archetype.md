The Concept Archetype is the Rust-side template authority for one concept kind in the declaration stack.
It is defined through Rust contracts/traits and registration wiring, and it constrains how declaration artifacts are
validated and materialized.

At the current draft stage, the active archetype set is intentionally small and fixed (for example `Scale`, `Metric`,
`Phenomenon`, `Scale Realizer`).

Scripts do not define archetypes; scripts define [[Concept Declaration Artifact]]s that target an archetype.

#glossary
