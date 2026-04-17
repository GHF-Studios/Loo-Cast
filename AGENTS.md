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
   - `docs/VISION_ARCHITECTURE.md`
   - `docs/USF_FLOW_GRAPH.md`
   - `docs/PROTOTYPE_CAPABILITIES.md`
   - `docs/STYLE_PATTERNS.md`
3. Current code behavior (ground truth for what is implemented right now).
4. Archived planning notes in `documents/temp_stuff/` (historical context only).

## Mandatory Read Order (New Session)

1. `AGENTS.md`
2. `docs/VISION_ARCHITECTURE.md`
3. `docs/USF_FLOW_GRAPH.md`
4. `docs/PROTOTYPE_CAPABILITIES.md`
5. `docs/STYLE_PATTERNS.md`
6. `docs/Crates.md` and `docs/Scripting.md`
7. Relevant source files for the active task

## Project Orientation (Short)

- USF is script-configured at the content/ontology layer.
- Rust/Bevy is the capability and execution platform.
- Rhai is for content declaration/orchestration, not for replacing engine implementation ownership.
- Public USF-facing concepts are the canonical language (metrics, metric sets, scales, phenomenon realizers, phenomena, phenomenon models, mods, modpacks, capability channels).
- Bevy ECS is an internal execution substrate and implementation detail behind the USF-facing model.
- Per-scale realization is driven by phenomenon realizers over metric/substrate state.
- Capability channels are part of the typed ctx graph and are used by scripts through intent emission.

## Runtime Composition Anchors

- Engine entrypoint: `core_engine/src/main.rs`
- Plugin group composition: `core_mod_api/src/lib.rs` (`CoreApiPluginGroup`)
- Workflow runtime framework: `core_mod_api/src/workflow/*`
- Rhai bootstrap + typed USF script contracts: `core_mod_api/src/rhai_binding/engine/bootstrap.rs`
- Schedule entrypoint runtime: `core_mod_api/src/rhai_binding/engine/schedule_entrypoint.rs`
- Core scripts + USF content scripts: `core_mod/assets/scripts/*`
- Canonical runtime catalog/registries: `core_mod_api/src/usf/mod_packs/mod.rs`
- Unified runtime concept query view: `UsfRuntimeConceptView` in `core_mod_api/src/usf/mod_packs/mod.rs`
- Bootstrap worldgen descent controller: `core_mod_api/src/usf/worldgen/mod.rs`

## Manifest Maintenance Rule

Update this manifest and the three canonical docs whenever one of these changes:

1. USF authority boundaries.
2. Runtime/bootstrap composition flow.
3. Core terminology or naming contracts.
4. Prototype scope/acceptance criteria.

Do not store temporary planning as canonical docs; place it under `documents/temp_stuff/`.
