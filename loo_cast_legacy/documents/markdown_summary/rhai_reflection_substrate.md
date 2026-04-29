# Rhai Reflection Substrate

Purpose: define the active Rhai binding substrate after the semantic reset.

## Active Surface

- Engine bootstrap/preprocess:
  - `core_mod_api/src/backend/rhai_binding/engine/mod.rs`
  - `core_mod_api/src/backend/rhai_binding/engine/preprocess.rs`
- Binding graph registration:
  - `core_mod_api/src/backend/rhai_binding/bind/*`
  - `core_mod_api/src/backend/rhai_binding/internals/statics.rs`
- Reflection metadata and identity/path model:
  - `core_mod_api/src/backend/rhai_binding/meta/*`
  - `core_mod_api/src/backend/rhai_binding/path/*`

## Registration Model

- Inventory-driven registration is the canonical model.
- Reflection metadata is declared explicitly by macros/attributes in Rust.
- Runtime engine boot registers the compiled binding graph.
- No centralized domain bridge ownership model is assumed in this substrate document.

## Direction

- Script-facing capability surfaces are explicit and typed.
- Capability implementation ownership remains Rust-side.
- Script usage stays bounded to exposed APIs selected by content/script context.
