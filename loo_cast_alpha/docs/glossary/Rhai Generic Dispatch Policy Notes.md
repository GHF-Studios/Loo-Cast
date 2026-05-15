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
2. One script/file always defines exactly one singleton-like Rhai declaration-type object of that profile kind.
3. "Type" here is script/declaration type in Rhai-domain terms, not Rust type and not plain Rust data-object instance
   semantics.
4. Profile defines the allowed script API graph topology: atomic capability nodes plus composite category nodes.
5. Access is declared as include/exclude path sets over that graph, so very specific capability-object subgraphs can be
   exposed.
6. Domains that are nonsensical, non-implementable for the kind, or dangerous are intentionally omitted.
7. Capabilities are Rhai-native dynamic API objects, identified by human-readable string IDs, and access to them is
   granted or denied by profile/policy.
8. `ctx` is object-based and dynamic; domains/subdomains can open/close over time according to runtime policy and
   declaration context.
9. Executing a declaration script materializes a full working declaration-type object that semantically describes one
   concept (for example a scale or phenomenon type).
10. Runtime invokes declaration-surface entrypoints with profile-tailored `ctx` API subgraphs (
    exposed/available/sensible/safe capability-object domains).
11. Complex concepts can still live in one file: richer syntax/logic/fields/parameters (and optional value-semantics
    helpers) are used to keep the one-file/one-concept rule intact.
12. Capability semantics are intentionally split:
    declaration-level capability APIs (script-side, builder-like definition surface) and runtime concept instances
    (execution-side Scale/Phenomenon/etc. instances containing closures over those API subgraphs).
13. This makes declaration scripts object descriptors first, effectively the closest thing to content assets in this
    project’s model.

Legacy dispatch extraction (still useful, but secondary to declaration semantics):

1. Reflect/declare operation metadata.
2. Register known signatures/catalog entries.
3. Resolve through deterministic keys/paths.
4. Execute through resolver/provider paths without bypass.

Open design space (rephrased around declaration/profile model):

1. How profile -> declaration-kind mapping is encoded and validated at load time.
2. How the one-script/one-concept invariant is enforced in tooling/runtime (error shape, diagnostics, migration path).
3. How atomic/composite API graph nodes are authored and versioned per profile.
4. How include/exclude path grammar, precedence, and conflict resolution are specified.
5. How API graph-domain allow/deny surfaces are reviewed and evolved per profile.
6. How capability-object grants/denials are declared, composed, audited, and dynamically opened/closed per profile.
7. How declaration-surface entrypoints + `ctx` subgraphs map to runtime hooks without leaking unrelated domains.
8. How fail-fast vs softer failure policy is scoped per profile and environment.
9. Which registry/dispatch details remain global and which should become profile-local.

Raw-model alignment (math + scripting contract posture):

- script entrypoints are declaration-profile entrypoints first
- facade/bridge surfaces are the monomorphized Rhai-safe contract surface used by declaration entrypoints, `ctx` API
  subgraphs, and capability objects
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
