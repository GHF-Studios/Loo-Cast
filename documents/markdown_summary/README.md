# Markdown Summary Index

Purpose: focused implementation-oriented notes that support (but do not replace) the diagram atlases.

## Read With

1. `../intention_records/README.puml`
2. `../intention_records/usf_records/00_manifest.puml`
3. `../intention_records/scripting_records/00_manifest.puml`
4. `../intention_records/platform_records/00_manifest.puml`

## Summaries

- `build_and_run.md` - current build/run script behavior.
- `crates_workspace.md` - workspace crate role split.
- `assets_and_ownership.md` - asset ownership and path conventions.
- `modding_status.md` - current modding status and limits.
- `scripting_runtime_reference.md` - current Rhai runtime/bootstrap reference.
- `rhai_bridge_playbook.md` - implementation playbook for adding bridges.
- `rhai_binding_roadmap.md` - status-oriented roadmap checklist.
- `rhai_generic_binding_policy.md` - generic-like binding constraints.
- `rhai_macro_surface.md` - macro-surface split and migration direction.
- `rhai_script_ergonomics.md` - alias/ergonomics status and proposals.
- `rhai_value_semantics.md` - value-semantics vocabulary and lifecycle boundaries.
- `usf_transform_policy.md` - transform policy and migration phases.
- `usf_math.md` - USF math boundary contracts and open questions.
- `usf_math_rhai_binding_surface.md` - facade-first math binding contract for Rhai-facing script APIs.

## Rule

If a markdown summary and a canonical diagram disagree, treat the diagram + user direction as authoritative and update the markdown summary.
