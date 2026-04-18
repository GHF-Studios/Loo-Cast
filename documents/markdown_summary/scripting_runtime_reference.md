# Scripting Runtime Reference

Current implementation + near-term target notes for Rhai runtime integration.

## Runtime Anchors

- Bootstrap: `core_mod_api/src/rhai_binding/engine/bootstrap.rs`
- Schedule entrypoint runtime: `core_mod_api/src/rhai_binding/engine/schedule_entrypoint.rs`
- Bridge declarations: `core_mod_api/src/rhai_binding/bridges/*`
- Runtime wrappers: `core_mod_api/src/rhai_binding/runtime/*`

## Entrypoint Model (Current)

- `boot.rhai` registers schedule entrypoints.
- Entrypoint loader resolves companion files recursively, then entrypoint root file.
- Ordered schedule execution is preserved by registration order for each schedule phase.

## Contract Direction

- Typed script contracts define USF content/ontology.
- Ctx graph is the script interaction surface.
- Capability channels are ctx nodes; write paths emit typed intents.
- Reconciliation hooks can use declared evaluator functions.
- Failures are panic-fast by default unless a sentinel path is explicitly declared.

## Related Canonical Diagrams

- `../intention_records/scripting_records/00_manifest.puml`
- `../intention_records/scripting_records/21_structure_ctx_api_graph.puml`
- `../intention_records/scripting_records/22_structure_capability_channel_graph.puml`
- `../intention_records/scripting_records/50_decision_contracts.puml`
