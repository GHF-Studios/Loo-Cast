# Rhai Generic Dispatch Policy Notes

#tech_glossary

Related glossary terms:

- [USF Instantiation Scripts](USF%20Instantiation%20Scripts.md)
- [USF Instantiation Script Profile Notes](USF%20Instantiation%20Script%20Profile%20Notes.md)
- [USF Runtime](USF%20Runtime.md)
- [Capability Runtime](Capability%20Runtime.md)
- [Runtime Substrate](Runtime%20Substrate.md)
- [USF Math Raw Model Foundation Notes](USF%20Math%20Raw%20Model%20Foundation%20Notes.md)

Quarantine extraction (`TMP_rhai_semantic_reset_quarantine`) with high signal but provisional authority:

1. Rhai cannot request new Rust monomorphizations at runtime.
2. Generic-like behavior likely needs an explicit dispatch registry pipeline.
3. Signature ID prefixes (`QUERY_SIG__`, `MESSAGE_SIG__`, `BUNDLE_SIG__`, `RESOURCE_SIG__`) are strong candidates, not
   final law.
4. Canonical Rust path-style type/trait IDs are likely useful for traceability and determinism.
5. Duplicate/invalid registration and resolver-miss handling should be fail-fast by default unless we later define
   softer recovery paths.

Declaration-first posture (primary model):

1. A script profile defines exactly one declaration kind.
2. One script/file corresponds to one declaration instance of that profile kind.
3. "Type" here is script/declaration type in Rhai-domain terms, not Rust type.
4. Profile defines the allowed script API graph domains (and their utility surface).
5. Domains that are nonsensical, non-implementable for the kind, or dangerous are intentionally omitted.
6. Runtime method calls are secondary and profile-bounded: they are methods on declared instances, not the conceptual
   center.

Legacy dispatch extraction (still useful, but secondary to declaration semantics):

1. Reflect/declare operation metadata.
2. Register known signatures/catalog entries.
3. Resolve through deterministic keys/paths.
4. Execute through resolver/provider paths without bypass.

Open design space (rephrased around declaration/profile model):

1. How profile -> declaration-kind mapping is encoded and validated at load time.
2. How strict the one-script/one-declaration-instance rule should be in edge cases.
3. How API graph-domain allow/deny surfaces are authored, reviewed, and evolved per profile.
4. How profile-bounded instance methods map to capability/runtime hooks without leaking unrelated domains.
5. How fail-fast vs softer failure policy is scoped per profile and environment.
6. Which registry/dispatch details remain global and which should become profile-local.

Raw-model alignment (math + scripting contract posture):

- script entrypoints are declaration-profile entrypoints first
- facade/bridge surfaces are the monomorphized Rhai-safe contract surface used by those profile-bounded instance methods
- the bridge layer should stay thin: translate between Rhai-friendly shapes and contract-facing typed surfaces
- avoid exposing raw generic math trait surfaces directly to scripts
- preserve explicit kind/repr/policy selection semantics (`OpMode` + `OpPolicy`)

Legacy source pointers:

-
`loo_cast_legacy/documents/temp_stuff/TMP_rhai_semantic_reset_quarantine/documents/markdown_summary/rhai_generic_binding_policy.md`
-
`loo_cast_legacy/documents/temp_stuff/TMP_rhai_semantic_reset_quarantine/core_mod_api/src/backend/rhai_binding/runtime/ecs/dispatch_policy.rs`
-
`loo_cast_legacy/documents/temp_stuff/TMP_rhai_semantic_reset_quarantine/documents/markdown_summary/rhai_bridge_playbook.md`
- `loo_cast_legacy/documents/temp_stuff/TMP_usf_math_raw_model/catalog.rs`
- `loo_cast_legacy/documents/temp_stuff/TMP_usf_math_raw_model/scalar/shared.rs`
- `loo_cast_legacy/documents/temp_stuff/TMP_usf_math_raw_model/vector/shared.rs`

Rustdoc anchors:

- `crates/core_mod/src/spec/rhai.rs`
