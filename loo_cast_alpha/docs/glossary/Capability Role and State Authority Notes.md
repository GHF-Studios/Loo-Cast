# Capability Role and State Authority Notes

#tech_glossary

Related glossary terms:

- [Capability Role](Capability%20Role.md)
- [Capability Contract](Capability%20Contract.md)
- [Capability Runtime](Capability%20Runtime.md)
- [USF Runtime Evolution Lifecycle](USF%20Runtime%20Evolution%20Lifecycle.md)

Current role notes:

1. Current role taxonomy is `input` and `output`.
2. Both are currently described as non-mutating with respect to canonical game-world state.
3. Runtime lifecycle notes still require explicit state-transition authority in reconcile/commit/apply paths.

This means runtime authority routing is a live design surface:

- which layer owns canonical mutation authority
- how capability roles map to that authority boundary
- whether additional role types are needed

Legacy source pointers:

- `loo_cast_legacy/documents/temp_stuff/TMP_engine_capability_stack_raw_model/new_insights.md`
- `loo_cast_legacy/documents/temp_stuff/TMP_usf_plan_capability_platform.md`

Rustdoc anchors:

- `crates/core_mod/src/spec/mod.rs`
