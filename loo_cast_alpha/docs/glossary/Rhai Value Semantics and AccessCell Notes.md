# Rhai Value Semantics and AccessCell Notes

#tech_glossary

Related glossary terms:

- [USF Instantiation Scripts](USF%20Instantiation%20Scripts.md)
- [USF Runtime](USF%20Runtime.md)
- [Runtime Substrate](Runtime%20Substrate.md)
- [USF Math Raw Model Foundation Notes](USF%20Math%20Raw%20Model%20Foundation%20Notes.md)

Quarantine extraction (`TMP_rhai_semantic_reset_quarantine`) with high signal but provisional authority:

1. Value semantics vocabulary is explicit:
   `Clone`, `Owned`, `Ref`, `Mut`, `ScopedOwned`, `ScopedRef`, `ScopedMut`.
2. These modes describe runtime access behavior, not persistent storage classes.
3. `AccessCell` is the runtime borrowing boundary between Rust and script-facing flows.
4. Scoped modes rely on provider-managed frame/window lifecycles.
5. Stale/invalid access and lifecycle contract violations are panic-fast.

`AccessCell` behavior surface that matters for future rewrite/migration:

- explicit start/end read and write transitions
- no overlap of write with any active read/write path
- optional take/invalidation path for scoped borrow recovery
- contention guardrail (bounded busy wait, then panic)

Current design tension (intentional and unresolved):

1. Thin-bridge direction:
   keep Rhai-facing semantics minimal and treat facade/bridge as mostly translation glue. Simpler but less flexible
   rhai-facing semantics.
2. Rich-semantics direction:
   retain a deeper value-semantics model (`Scoped*`, lifecycle windows, explicit access policy) as a first-class
   scripting contract. More complex but also more flexible rhai-facing semantics.
3. Integration constraint:
   whichever direction wins must compose cleanly with [[Capability Declaration]]s and profile-tailored `ctx`
   capability-object subgraphs.

Both directions have merit; final shape is still open and should be decided after more runtime + ergonomics review.

Legacy source pointers:

-
`loo_cast_legacy/documents/temp_stuff/TMP_rhai_semantic_reset_quarantine/documents/markdown_summary/rhai_value_semantics.md`
-
`loo_cast_legacy/documents/temp_stuff/TMP_rhai_semantic_reset_quarantine/core_mod_api/src/backend/rhai_binding/value_semantics/modes.rs`
-
`loo_cast_legacy/documents/temp_stuff/TMP_rhai_semantic_reset_quarantine/core_mod_api/src/backend/rhai_binding/value_semantics/access_cell.rs`

Rustdoc anchors:

- `crates/core_mod/src/spec/rhai.rs`
