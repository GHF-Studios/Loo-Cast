# TMP Rhai Semantic Reset Quarantine

Purpose: hold pre-reset Rhai runtime/bridge/value-semantics implementation artifacts for deferred value extraction.

## Quarantined Trees

- `core_mod_api/src/backend/rhai_binding/bridges/*`
- `core_mod_api/src/backend/rhai_binding/runtime/*`
- `core_mod_api/src/backend/rhai_binding/value_semantics/*`
- `documents/markdown_summary/{scripting_runtime_reference,rhai_bridge_playbook,rhai_binding_roadmap,rhai_generic_binding_policy,rhai_script_ergonomics,rhai_value_semantics}.md`

## Status

- Quarantined on pass: scripting semantic reset (inventory-first reflection substrate).
- Active authoritative Rhai substrate now lives under:
  - `core_mod_api/src/backend/rhai_binding/{engine,bind,meta,path,internals}/*`
  - `documents/markdown_summary/rhai_reflection_substrate.md`
