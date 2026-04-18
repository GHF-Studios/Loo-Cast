# Rhai Macro Surface (Summary)

Current split:

1. Attribute-style reflection macros (`#[reflect_*]`) for local AST-driven reflection.
2. Declarative extern-style macros (`reflect_extern_*!(...)`) for explicit external metadata-driven registration.

## Direction

- Keep both capability classes.
- Reduce naming noise and route through shared generators where practical.
- Perform naming/ergonomics unification as a focused macro task, not mixed into unrelated refactors.
