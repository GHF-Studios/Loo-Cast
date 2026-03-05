# Agent TODO

Purpose: live, temporary working log and progress tracker for active agent work.

Read first: `docs/RhaiAgentHandoff.md`

## Live Initiative: USF Load-Overload Safety Gate (Active)

Status: implementation in progress

### Confirmed Constraints

- Bevy is the viewport/runtime projection layer; USF state remains canonical.
- Player stays the local anchor; world context shifts around the player.
- Translation, scale, and rotation origin-shifting are separate concerns.
- Freeze must not trigger merely because a threshold was crossed.
- Freeze must trigger only when chunk-load orchestration reports overload (timeout path).
- Freeze scope includes player translation, zoom, and rotation.
- Freeze must not rely on global virtual pause, because loading workflows must continue.
- Default workflow timeout behavior remains immediate panic unless an explicit policy overrides it.
- Local zoom pivot policy remains strict window semantics with buffer-zone overshoot handling.
- Wide camera zoom config in `core_mod/assets/configs/config.toml` is intentional; previous “fake infinite top-level” report is considered resolved.

### Approved Multi-Step Plan

1. Add a global singleton gate resource (working name: `ChunkLoadGate`) with explicit states.
2. Keep threshold crossings non-blocking by default; do not lock gate on pivot event alone.
3. Introduce workflow timeout control path that allows orchestrator-level timeout requests while preserving default panic behavior.
4. Wire chunk loading orchestration to emit timeout requests on first timeout in the opt-in path.
5. Add dedicated run-condition/gating mechanism for gameplay authority surfaces (translation/zoom/rotation) separate from `run_if_not_paused`.
6. On active gate lock, hard-freeze player movement, mousewheel zoom authority, and world-rotation input.
7. Keep chunk/GPU loading workflows running while gate is active, then unlock immediately when completion conditions are truly met.
8. Add clear `warn!` logs for lock/unlock, timeout request, decision result, and recovery completion.
9. After stabilization, add gameplay red viewport border while gate is active.
10. Longer follow-up: workflow framework facelift discussion/RFC (metadata-first registration + explicit timeout decision interfaces + finally add documentation for the entire workflow framework).

### Implementation Checklist

- [x] Define `ChunkLoadGate` resource and lifecycle/state transitions.
- [x] Move chunk in-flight tracking from system-local state to globally queryable resource state.
- [x] Add timeout request/decision plumbing for workflow runner path (opt-in extension, default panic unchanged).
- [x] Integrate chunk orchestration with timeout request emission and decision handling.
- [x] Add dedicated gameplay-input gate run condition.
- [x] Gate player translation in `player::systems::update_player_system`.
- [x] Gate zoom authority in `render::systems::main_camera_zoom_system`.
- [x] Gate world-rotation authority in `player::systems::update_player_system`.
- [x] Implement deterministic unlock checks based on completed loading conditions.
- [x] Add `warn!` observability for all significant gate/timeout transitions.
- [x] Validate via `cargo test`.
- [ ] Runtime integration pass with `./build.sh dev; ./run.sh dev`.
- [x] Add gameplay overload red border (inverted semantics vs debug-suite border) after stabilization.

### Active Risks / Open Design Threads

- Workflow timeout control integration may require non-trivial runner API expansion.
- `run_if_not_paused` and overload-cutoff concerns must remain disentangled.
- Future workflow-system facelift should be scoped as a separate design track to avoid destabilizing this safety pass.
- Runtime smoke pass (`./run.sh dev`) can be externally blocked if `console_subscriber` bind address is already in use.
- Temporary runtime smoke workaround: `TOKIO_CONSOLE_BIND=127.0.0.1:0 ./run.sh dev`.

---

## Historical Snapshot (Condensed)

Completed previously in this cycle:

- Rhai startup/testing separation and `rhai_binding/testing_enabled` gating cleanup.
- Bundle/provider dispatch consolidation and AccessCell-first scoped API migration.
- Generic dispatch normalization and Rhai import alias ergonomics foundation.
- USF chunk-spacing and pivot-path stabilization work (including scale-link regression test coverage).
