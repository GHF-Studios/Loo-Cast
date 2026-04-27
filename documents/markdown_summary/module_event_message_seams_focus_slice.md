# Module Event/Message Seams (Focus Slice)

Purpose: sketch a cross-module event/message-oriented seam model for the current focus slice without flattening USF semantics.

Status: directional contract sketch for upcoming refactor passes.

## Scope (This Slice)

- `core`
- `chunk` (USF-specific)
- `debug`
- `usf`
- `rhai_binding`
- `render`
- `player`

## Direction Locked In

- `chunk` remains a USF-specific module.
- `ChunkId` is a newtype over `GridVec`.
- `UnitVec` models intra-chunk local state.
- Prefer runtime seams that communicate via Bevy event/message flows instead of direct cross-module "call this now" coupling.

## Current Coupling Hotspots (Implementation Reality)

- `core` directly imports chunk types for proxy-root and transform projection logic:
  - `core_mod_api/src/backend/core/systems.rs`
- `debug/gizmo` directly imports chunk types for selection/mobility filtering and scale handling:
  - `core_mod_api/src/backend/debug/gizmo/systems.rs`
- `chunk` currently mixes boundary planning with direct spawn/despawn ownership:
  - `core_mod_api/src/backend/chunk/systems.rs`

## Seam Sketch (Black-Box View)

### `player`
- Owns: authoritative player anchor state.
- Emits: anchor updates used by chunk topology math.
- Consumes: none in this slice.

### `chunk` (USF-specific)
- Owns: chunk topology/windowing math, boundary planning, load-gate state.
- Emits: boundary lifecycle and boundary delta events/messages.
- Consumes: anchor updates and realization directives.
- Does not own: direct gameplay materialization inside chunk boundaries.

### `usf`
- Owns: canonical USF contracts and runtime authority flows.
- Emits: realization directives and diagnostics/runtime facts.
- Consumes: chunk boundary lifecycle/deltas and script-entrypoint intents.

### `rhai_binding`
- Owns: engine bootstrap, preprocessing, binding graph registration.
- Emits: typed script-entrypoint intents into USF runtime.
- Consumes: runtime snapshots/diagnostics for script-facing feedback surfaces.

### `core`
- Owns: orchestration pressure/runtime coordination state and proxy lifecycle framework.
- Emits: runtime coordination directives and module-level orchestration signals.
- Consumes: module facts/intents (including chunk/usf/debug-facing events/messages).

### `render`
- Owns: render-facing proxy/runtime state.
- Emits: runtime telemetry facts.
- Consumes: core coordination directives and projected state facts.

### `debug`
- Owns: inspection/tooling UI and debug interaction intents.
- Emits: debug/tool intents.
- Consumes: runtime facts from core/usf/render/chunk.

## Event/Message Delivery Semantics (Not Locked Yet)

This pass does not hard-lock taxonomy names. Practical split to evaluate during implementation:

- Broadcast-like events/messages:
  - runtime facts, diagnostics, telemetry, lifecycle observations
- Authority-routed events/messages:
  - directives/intents that must be consumed by a specific authority path

The exact naming can stay provisional until code passes expose the stable split naturally.

## Pass Loop (Docs -> Code -> Refine)

1. Capture seam map and module black-box responsibilities (this doc + platform diagram).
2. Implement first module-level events/messages for focused seams.
3. Replace direct chunk imports in non-chunk modules where event/message boundaries are introduced.
4. Re-evaluate naming and semantics after concrete usage patterns emerge.
5. Update docs/diagrams again before next pass.

Pass boundaries are intentionally revisitable as implementation reveals better cuts.

## Cross References

- `../intention_records/platform_records/30_module_event_message_seams.puml`
- `../intention_records/platform_records/10_runtime_composition.puml`
- `../intention_records/scripting_records/32_flow_runtime_tick_orchestration.puml`
- `../intention_records/scripting_records/11_context_authority_boundaries.puml`
- `usf_script_profiles_and_mvp_slice.md`
- `rhai_reflection_substrate.md`
