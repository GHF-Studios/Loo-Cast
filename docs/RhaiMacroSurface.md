# Rhai Macro Surface

This note records the current macro split and the unification direction.

## Current families

1. Attribute-style (`#[reflect_*]`)
- Operate on local Rust items (`ItemStruct`, `ItemTrait`, `ItemImpl`, functions).
- Can infer metadata directly from item AST.
- Most useful when reflecting code declared in the same module.

2. Declarative extern-style (`reflect_extern_*!(...)`)
- Operate from explicit metadata + registrator closures.
- Designed for crate-external types, custom wrapper types, and manual
  registration control.

## Unification assessment

They are related, but not identical in capability or input model.

- `reflect_extern_top_level_module!` and `reflect_extern_sub_module!` already
  delegate to the same generator as non-extern module macros.
- For type/function/trait reflection, full hard merge into one syntax is not a
  drop-in change because attribute and declarative forms solve different
  problems (AST-derived vs explicit registrator-driven).

## Recommended direction

1. Keep both capabilities, but reduce naming noise.
- Treat `reflect_*` as canonical naming.
- Keep `reflect_extern_*` as compatibility aliases while migration is in flight.

2. Introduce normalized naming wrappers later.
- Example target surface:
  - `reflect_module!`
  - `reflect_type!` (declarative form)
  - `reflect_trait!` (declarative form)
  - plus attribute forms for local items
- Internally route to shared generators where possible.

3. Migrate existing bridge code toward one style per domain.
- For bridge declaration modules, prefer declarative style for consistency.
- Keep attribute style primarily for local-item reflection workflows.

## Short-term conclusion

Macro surface simplification is viable, but should be done as a focused macro
task, not mixed into structural cleanup. The cleanup step only documents and
prepares this migration path.
