# Rhai Script Ergonomics (Summary)

## Implemented

- Local alias preprocessing with:
  - `use full::path::Type as Alias;`
- Alias expansion for:
  - `Alias::...`
  - bare `Alias` token to canonical path-type literal
- Alias safety checks:
  - duplicate aliases rejected
  - keyword aliases rejected
  - symbol-collision aliases rejected

## Pending/Partial

- Typed-id helper ergonomics across more APIs.
- Display shorthand for generic-heavy signatures while keeping canonical IDs unchanged.

## Rule

Ergonomics cannot weaken canonical dispatch-key determinism.
