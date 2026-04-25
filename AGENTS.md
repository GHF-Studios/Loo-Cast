# AGENTS.md

Purpose: canonical AI entrypoint and manifest for this repository.

## Non-Negotiable Collaboration Protocol

1. If anything is ambiguous, ask the user a short question batch and stop.
2. Do not continue with assumptions that replace missing user answers.
3. If code, docs, and vision diverge:
   - state `Current Implementation` (code-observed),
   - state `Owner Direction` (user-authoritative),
   - ask for decision and wait.
4. The user is the final arbiter for product/architecture intent.

## Truth Hierarchy

1. User instructions in the current conversation.
2. Canonical docs:
   - `AGENTS.md`
   - `documents/intention_records/README.puml`
   - `documents/intention_records/usf_records/00_manifest.puml`
   - `documents/intention_records/scripting_records/00_manifest.puml`
   - `documents/intention_records/platform_records/00_manifest.puml`
   - `documents/markdown_summary/README.md`
3. Current code behavior (ground truth for what is implemented right now).
4. Archived planning notes in `documents/temp_stuff/` (historical context only).

## Mandatory Read Order (New Session)

1. `AGENTS.md`
2. `documents/intention_records/README.puml`
3. `documents/intention_records/usf_records/00_manifest.puml`
4. `documents/intention_records/usf_records/10_vision_architecture.puml`
5. `documents/intention_records/usf_records/20_flow_graph.puml`
6. `documents/intention_records/scripting_records/00_manifest.puml`
7. `documents/intention_records/platform_records/00_manifest.puml`
8. `documents/markdown_summary/README.md`
9. Relevant source files for the active task

## Project Orientation (Short)

- USF is script-configured at the content/ontology layer.
- Rust/Bevy is the capability and execution platform.
- Rhai is for content declaration/integration, not for replacing engine implementation ownership.
- Public USF-facing concepts are the canonical language (metrics, metric sets, scales, phenomenon realizers, phenomena, phenomenon models, mods, modpacks, capability channels).
- Bevy ECS is an internal execution substrate and implementation detail behind the USF-facing model.
- Per-scale realization is driven by phenomenon realizers over metric/substrate state.
- Capability channels are part of the typed ctx graph and are used by scripts through intent emission.

## Runtime Composition Anchors

- Engine entrypoint: `core_engine/src/main.rs`
- Plugin group composition: `core_mod_api/src/lib.rs` (`CoreApiPluginGroup`)
- Workflow runtime framework: `core_mod_api/src/backend/workflow/*`
- Rhai engine bootstrap/runtime handle: `core_mod_api/src/backend/rhai_binding/engine/mod.rs`
- Rhai source preprocessing pipeline: `core_mod_api/src/backend/rhai_binding/engine/preprocess.rs`
- Rhai reflection metadata + binding graph substrate: `core_mod_api/src/backend/rhai_binding/{meta,bind,path,internals}/*`
- USF runtime plugin + authority contracts: `core_mod_api/src/backend/usf/{mod.rs,authority.rs}`
- Canonical engine asset roots: `core_mod/assets/{configs,shaders,...}`

## Manifest Maintenance Rule

Update this manifest and canonical atlases/summaries whenever one of these changes:

1. USF authority boundaries.
2. Runtime/bootstrap composition flow.
3. Core terminology or naming contracts.
4. Prototype scope/acceptance criteria.

Do not store temporary planning as canonical docs; place it under `documents/temp_stuff/`.
