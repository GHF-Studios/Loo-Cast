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
6. One script/file declares one instance of that profile kind.
7. Profile "type" here is script/declaration typing in Rhai-domain terms, not Rust typing.

Current startup-flow shape used as reference:

1. Read script files.
2. Validate capability paths against profile.
3. Preprocess aliases.
4. Compile and execute definition hooks.
5. Execute sample realization path.
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
