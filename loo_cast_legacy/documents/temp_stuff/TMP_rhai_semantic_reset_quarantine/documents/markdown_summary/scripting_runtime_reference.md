# Scripting Runtime Reference

Current implementation + near-term target notes for Rhai runtime integration.

## Runtime Anchors

- Engine runtime plugin + handle: `core_mod_api/src/backend/rhai_binding/engine/mod.rs`
- Source preprocess pipeline: `core_mod_api/src/backend/rhai_binding/engine/preprocess.rs`
- Bridge declarations: `core_mod_api/src/backend/rhai_binding/bridges/*`
- Runtime wrappers: `core_mod_api/src/backend/rhai_binding/runtime/*`

## Execution Ownership

- Runtime lifecycle orchestration is Rust-owned.
- Script execution timing is determined by runtime capability composition, not script-driven phase wiring.
- Discovery and execution are attached to typed content contracts and registry metadata.

## Contract Direction

- Typed script contracts define USF content/ontology.
- Ctx graph is the script interaction surface.
- Capability channels are ctx nodes; write paths emit typed intents.
- Reconciliation hooks can use declared evaluator functions.
- Failures are panic-fast by default unless a sentinel path is explicitly declared.

## Math Facade Linkage

- Script-facing math should be consumed through facade/binding surfaces, not raw generic contracts.
- Rust owns math implementation details; Rhai owns orchestration and content usage.
- See `usf_math_rhai_binding_surface.md` for the contract-level mapping.

## Related Canonical Diagrams

- `../intention_records/scripting_records/00_manifest.puml`
- `../intention_records/scripting_records/21_structure_ctx_api_graph.puml`
- `../intention_records/scripting_records/22_structure_capability_channel_graph.puml`
- `../intention_records/scripting_records/50_decision_contracts.puml`
