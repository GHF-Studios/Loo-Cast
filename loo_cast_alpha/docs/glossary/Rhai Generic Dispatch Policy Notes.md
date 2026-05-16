# Rhai Generic Dispatch Policy Notes

#tech_glossary

Related glossary terms:

- [USF Instantiation Scripts](USF%20Instantiation%20Scripts.md)
- [USF Instantiation Script Profile Notes](USF%20Instantiation%20Script%20Profile%20Notes.md)
- [USF Runtime](USF%20Runtime.md)
- [Concept Archetype](Concept%20Archetype.md)
- [Concept Declaration Artifact](Concept%20Declaration%20Artifact.md)
- [Rhai Capability](Rhai%20Capability.md)
- [Rust Capability](Rust%20Capability.md)
- [Capability Runtime](Capability%20Runtime.md)
- [Runtime Substrate](Runtime%20Substrate.md)
- [Workflow Framework](Workflow%20Framework.md)
- [USF Math Raw Model Foundation Notes](USF%20Math%20Raw%20Model%20Foundation%20Notes.md)

Quarantine extraction (`TMP_rhai_semantic_reset_quarantine`) with high signal but provisional authority:

1. Rhai cannot request new Rust monomorphizations at runtime.
2. Generic-like behavior likely needs an explicit dispatch registry pipeline.
3. Signature ID prefixes (`QUERY_SIG__`, `MESSAGE_SIG__`, `BUNDLE_SIG__`, `RESOURCE_SIG__`) are strong candidates, not
   final law.
4. Canonical Rust path-style type/trait IDs are likely useful for traceability and determinism.
5. Duplicate/invalid registration and resolver-miss handling should be fail-fast by default unless we later define
   softer recovery paths.

Terminology correction (draft):

1. "Rhai type" is useful intuition but too ambiguous for this stack.
2. Preferred term for script output is [[Concept Declaration Artifact]].
3. Concept templates live Rust-side as [[Concept Archetype]] registrations.

Declaration-first posture (primary model):

1. A script profile defines exactly one declaration kind.
2. One script/file defines exactly one singleton-like [[Concept Declaration Artifact]] of that profile kind.
3. [[Concept Archetype]]s are Rust-side template authorities (trait/registration wiring), not script-produced objects.
4. Script execution yields data-first declaration artifacts (POD-oriented with declared behavior payload), not raw Rust
   type objects.
5. Profile defines the allowed script API graph topology: atomic capability nodes plus composite category nodes.
6. Access is declared as include/exclude path declarations over that graph, so very specific capability-object subgraphs
   can be exposed.
7. Domains that are nonsensical, non-implementable for the kind, or dangerous are intentionally omitted.
8. Capabilities exposed to scripts are [[Rhai Capability]] objects, identified by human-readable string IDs, and access
   to them is granted or denied by profile/policy.
9. `ctx` is object-based and dynamic; domains/subdomains can open/close over time according to runtime policy and
   declaration context.
10. Runtime executes declaration entrypoints with profile-tailored `ctx` subgraphs to produce declaration artifacts.
11. A Rust materialization pass consumes those artifacts and produces runtime concept machinery.
12. Complex concepts can still live in one file: richer syntax/logic/fields/parameters (and optional value-semantics
    helpers) are used to keep the one-file/one-concept rule intact.
13. Capability semantics are intentionally split:
    declaration-level [[Rhai Capability]] API surfaces and runtime-side [[Rust Capability]] execution/orchestration
    surfaces.
14. This keeps scripts declaration/object-descriptor first while runtime behavior remains Rust-side.

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
7. How declaration entrypoints + `ctx` capability-object subgraphs map into declaration artifacts and then into
   materialized runtime concepts without leaking unrelated domains.
8. How fail-fast vs softer failure policy is scoped per profile and environment.
9. Which registry/dispatch details remain global and which should become profile-local.

Raw-model alignment (math + scripting contract posture):

- script entrypoints are declaration-profile entrypoints first
- facade/bridge surfaces are the monomorphized Rhai-safe contract surface used by declaration entrypoints, `ctx`
  capability-object
  subgraphs, and capability objects
- the bridge layer should stay thin: translate between Rhai-friendly shapes and contract-facing typed surfaces
- workflow orchestration should consume materialized Rust-side artifacts and should not directly drive raw Rhai engine
  internals
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
