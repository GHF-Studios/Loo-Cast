# USF Instantiation Script Profile Notes

#tech_glossary

Related glossary terms:

- [USF Instantiation Scripts](USF%20Instantiation%20Scripts.md)
- [USF Definition Lifecycle](USF%20Definition%20Lifecycle.md)
- [USF Instance Graph](USF%20Instance%20Graph.md)
- [USF Math Raw Model Foundation Notes](USF%20Math%20Raw%20Model%20Foundation%20Notes.md)

Current profile-direction notes (legacy MVP slice alignment):

1. Script file profiles are explicit (`scale`, `metric`, `phenomenon`, `phenomenon_realizer`).
2. Capability use is context-rooted and profile-gated.
3. Alias preprocessing (`use ... as ...`) is part of the script-loading flow.
4. Definition content is loaded, validated, and frozen for runtime progression.
5. Each profile maps to one declaration kind.
6. One script/file always defines one singleton-like Rhai declaration-type object of that profile kind.
7. Profile "type" here is script/declaration typing in Rhai-domain terms, not Rust typing and not plain Rust
   object-instance semantics.
8. Capabilities in scripts are Rhai-native dynamic API objects (human-readable string IDs), with profile/policy-based
   grant or deny access.
9. Executing script declaration code yields a full working declaration-type object that semantically describes one
   concept (for example one scale type or one phenomenon type).
10. Complex concepts are still authored as one file/one concept by using richer declaration syntax and logic within that
    file.
11. Runtime later materializes USF concept instances (for example Scale/Phenomenon instances) from frozen declaration
    objects.
12. These runtime concept instances carry closures/logic that execute through profile-tailored `ctx` capability-object
    subgraphs.
13. Capability semantics are split between declaration-level capability APIs (builder-like declaration surface) and
    runtime concept-instance behavior.

Current startup-flow shape used as reference:

1. Read script files.
2. Validate capability paths against profile.
3. Preprocess aliases.
4. Compile and execute declaration entrypoints with profile-tailored `ctx` API subgraphs to materialize declaration-type
   objects.
5. Activate runtime and materialize concept instances from frozen declarations.
6. Emit runtime proof logging.
7. Freeze definition-side mutation.

Legacy source pointers:

- `loo_cast_legacy/documents/markdown_summary/usf_script_profiles_and_mvp_slice.md`
-
`loo_cast_legacy/documents/temp_stuff/TMP_rhai_semantic_reset_quarantine/documents/markdown_summary/rhai_script_ergonomics.md`
-
`loo_cast_legacy/documents/temp_stuff/TMP_rhai_semantic_reset_quarantine/documents/markdown_summary/scripting_runtime_reference.md`

Adjacent notes:

- [Rhai Generic Dispatch Policy Notes](Rhai%20Generic%20Dispatch%20Policy%20Notes.md)
- [Rhai Value Semantics and AccessCell Notes](Rhai%20Value%20Semantics%20and%20AccessCell%20Notes.md)

Rustdoc anchors:

- `crates/core_mod/src/spec/mod.rs`
