# USF Implementation and Refactor Progress

Status: `review_hold`
Owner: `codex + user`
Last updated: `2026-03-11`
Renamed from: `ProxyWindowingRefactorProgress.md`

## Purpose
This is now a general USF implementation and refactor archive, not only a proxy-windowing tracker.
It preserves high-signal assistant responses and the current execution direction, so we can review first and execute later with confidence.

## Important Review Rule
1. Treat this document as a planning and review artifact first.
2. Do not treat captured response snapshots as auto-approved implementation work.
3. Move items to active execution only after explicit review confirmation.

## Preserved Response Snapshot A (Near-Verbatim)
This snapshot is preserved from the assistant response that locked plan steps and executed the required validation gate.

> Plan steps are now locked in as execution-only phases in `docs/current_refactor.md`, including acceptance criteria and a validation gate. I explicitly avoided any “how to plan” meta-steps and only kept concrete implementation steps of the USF plan.
>
> I executed the requested validation sequence without running `cargo clean`:
> 1. `cargo test` passed (workspace; `core_mod_api` tests and doctests passed).
> 2. `./build.sh dev` passed.
> 3. `./run.sh dev` executed but aborts in this sandbox with `failed to initiate panic, error 5` (exit 134).

## Preserved Response Snapshot B (Near-Verbatim)
This snapshot is preserved from the assistant response after the explicit “please proceed”, where render authority handoff was executed and validated.

> I continued with the next execution phase: moving render fully onto frontier/node-state authority and removing chunk-coordinate snapshot authority.
>
> I patched `core_mod_api/src/render/systems.rs` so that:
> 1. `update_phenomenon_model_surfaces_system` consumes live `PhenomenonNodeState` snapshots keyed by `frontier_node_seed`.
> 2. `sync_phenomenon_chunk_proxy_system` binds chunk proxies to real frontier seeds (scale + distance-aware selection), rather than deriving authoritative state from chunk-coordinate hashing.
> 3. The `phenomenon_snapshot_from_chunk_coord` authority path and related tests were removed/replaced with frontier-seed selection tests.
>
> Validation after this handoff:
> 1. `cargo test` passed.
> 2. `./build.sh dev` passed.
> 3. `./run.sh dev` executed; in this sandbox it aborts with `failed to initiate panic, error 5`.

## Preserved Response Snapshot C (Near-Verbatim)
This snapshot is preserved from the follow-up execution where zone-driven realization became runtime-authoritative for chunk proxy ownership.

> I implemented zone-driven realization end-to-end in runtime, not only as registry scaffolding:
> 1. `usf/zone` now maintains deterministic `chunk -> zone` and `zone -> parent zone` indices in `ZoneRuntimeState`.
> 2. `ZoneAnchor` now carries hierarchy linkage (`parent`) and remains time-factor aware.
> 3. Added `ZoneRealizationState` and `ZonePhenomenon` marker to realize each zone into a deterministic `Phenomenon` entity (stable phenomenon id derived from zone id).
> 4. `sync_phenomenon_chunk_proxy_system` now resolves each chunk proxy’s source phenomenon from zone realization (with global-root fallback only when needed), instead of forcing one global phenomenon for every chunk.
> 5. Frontier seed selection is now scoped to the selected chunk phenomenon’s node lineage first, then global fallback.
> 6. Surface snapshot cache keys now include `(phenomenon_id, frontier_seed)` and mesh signatures hash `proxy.source`, preventing cross-phenomenon cache collisions.
> 7. Schedule-hook companion file types are now concretely used in `zone_update/` with split responsibilities:
>    - `*.lib.rhai` (`zone_math.lib.rhai`)
>    - `*.hook.rhai` (`zone_pipeline.hook.rhai`)
>    - `*.zone.rhai` (`zone_runtime.zone.rhai`)
>
> Validation after this pass:
> 1. `cargo test -p core_mod_api` passed (`122` tests).
> 2. `./build.sh dev` passed.
> 3. `./run.sh dev` still aborts in this sandbox with `failed to initiate panic, error 5` (exit `134`).

## Preserved Response Snapshot D (Near-Verbatim)
This snapshot is preserved from the next hard-cut execution where fallback proxy authority was removed and zone behavior mapping moved to script registration.

> I completed both requested hard cuts:
> 1. Removed chunk-proxy fallback authority: `sync_phenomenon_chunk_proxy_system` now requires zone resolution (`chunk -> zone -> phenomenon`) and no longer falls back to global-root phenomenon ownership.
> 2. Added boot-time Rhai zone behavior registration via new runtime module `rhai_binding::usf_zone`:
>    - `clear_phenomenon_kinds()`
>    - `set_phenomenon_kind(zone_type, phenomenon_kind)`
> 3. Added static bridge store `USF_ZONE_KIND_BY_TYPE`, and `ZoneBehaviorRegistry` now hydrates from that script-populated map before zone realization runs.
> 4. `boot.rhai` now explicitly registers default zone mappings (`void/arid/alpine -> mandelbulb`, `forest/wetland -> sierpinski_sponge`) through this Rhai API.
>
> Validation after these hard cuts:
> 1. `cargo test -p core_mod_api` passed (`123` tests).
> 2. `./build.sh dev` passed.
> 3. `./run.sh dev` still aborts in this sandbox with `failed to initiate panic, error 5` (exit `134`).

## Preserved Response Snapshot E (Near-Verbatim)
This snapshot is preserved from the follow-up execution to enforce deterministic USF phase ordering and add observer-style realization signals.

> I tightened structural scheduling and observer surfaces:
> 1. Added ordered USF simulation sets (`Substrate -> Zone -> Phenomenon`) and wired zone/phenomenon systems into those sets.
> 2. Changed schedule-hook registry from unordered set to insertion-ordered list; hook registration order from `boot.rhai` is now deterministic.
> 3. Added dedicated phenomenon schedule hook files and boot registration:
>    - `phenomenon_pre_update`
>    - `phenomenon_update`
> 4. Added `ZoneRealizationEvent` message stream (`Spawned`/`Despawned`) emitted by zone realization reconciliation for observer-style consumers.
> 5. Updated global render-root creation so it no longer spawns a synthetic `Phenomenon`; zones remain the source of phenomenon entities.
>
> Validation after this phase:
> 1. `cargo test -p core_mod_api` passed (`123` tests).
> 2. `./build.sh dev` passed.
> 3. `./run.sh dev` aborts in sandbox due missing X display (`XOpenDisplayFailed`), not a compile/runtime-logic regression.

## Preserved Response Snapshot F (Near-Verbatim)
This snapshot is preserved from the follow-up hard-cut to reduce per-chunk phenomenon duplication artifacts and align chunk/zone authority with USF hierarchy semantics.

> I implemented the next structural runtime cuts:
> 1. Reworked chunk target selection to strict hierarchical closure:
>    - load radius only at active scale
>    - coarser scales are loaded only as parents of already-targeted child chunks (transitive closure)
> 2. Replaced transient zone-id derivation from frame-local connected-component hashes with overlap-based reconciliation:
>    - new components reuse prior zone ids when overlap exists
>    - merges/splits resolve deterministically via overlap + stable tie-break
>    - zone anchor entities now persist through incremental chunk loading instead of respawning per component growth
> 3. Removed cross-phenomenon frontier-seed fallback in chunk proxy sync:
>    - chunk proxies now bind frontier seeds only from their owning phenomenon lineage
> 4. Added explicit USF sub-phase sets and wired script hook names to those sets:
>    - `UsfSubstrateSet::{Pre, Runtime, Post}`
>    - `UsfZoneSet::{Pre, Runtime, Post}`
>    - `UsfPhenomenonSet::{Pre, Runtime, Post}`
>    - plus new `substrate_pre_update.rhai` and `substrate_update.rhai`.
>
> Validation after this phase:
> 1. `cargo test -p core_mod_api` passed (`127` tests).
> 2. `./build.sh dev` passed.
> 3. `./run.sh dev` still aborts in this sandbox with `failed to initiate panic, error 5` (exit `134`).

## Preserved Response Snapshot G (Near-Verbatim)
This snapshot is preserved from the follow-up scripting runtime pass to align hook execution with the desired file/module and parameterized-entrypoint model.

> I implemented hook-entrypoint and file-type structuring updates:
> 1. Hook runner now supports `main(world, params)` as the preferred entrypoint and falls back to `main(world)` for legacy hooks.
> 2. Added hook context map injection (`params`) with runtime metadata:
>    - `hook_name`, `hook_file`, `domain`, `stage`
>    - `delta_seconds`, `elapsed_seconds` (if virtual time exists)
>    - `active_scale_index` (if zone temporal context exists)
> 3. Extended companion-file ordering to include additional USF typed script roles:
>    - `*.substrate.rhai`
>    - `*.zone.rhai`
>    - `*.phenomenon.rhai`
> 4. Updated USF schedule hook files to use `fn main(world, params)` signatures.
>
> Validation after this phase:
> 1. `cargo test -p core_mod_api` passed.
> 2. `./build.sh dev` passed.
> 3. `./run.sh dev` still aborts in this sandbox with `failed to initiate panic, error 5` (exit `134`).

## Preserved Response Snapshot H (Near-Verbatim)
This snapshot is preserved from the follow-up hard-cut replacing generic map hook params with domain-typed parameter surfaces.

> I replaced generic hook param maps with typed hook param payloads while keeping `main(world, params)`:
> 1. Added runtime-registered Rhai types:
>    - `GlobalHookParams`
>    - `SubstrateHookParams`
>    - `ZoneHookParams`
>    - `PhenomenonHookParams`
>    - plus shared `HookCommonParams`
> 2. Hook runner now dispatches domain-specific typed params by hook name family (`substrate_*`, `zone_*`, `phenomenon_*`, fallback global).
> 3. Typed fields include shared metadata/timing and domain-specific counters (for example zone loaded/realized counts, phenomenon active node/frontier counts).
> 4. Updated USF hook script files to consume typed params and removed assumptions of map-style indexing.
>
> Validation after this phase:
> 1. `cargo test -p core_mod_api` passed (`129` tests).
> 2. `./build.sh dev` passed.
> 3. `./run.sh dev` still aborts in this sandbox with `failed to initiate panic, error 5` (exit `134`).

## Preserved Response Snapshot I (Near-Verbatim)
This snapshot is preserved from the follow-up substrate-contract pass where DPT/ZLM registry ownership moved to boot-time script registration.

> I implemented the next structural substrate cut so DPT/ZLM definitions are script-authored and Rust-enforced:
> 1. Added a new boot-time Rhai runtime module `rhai_binding::usf_substrate` with deterministic authoring APIs:
>    - `scale_level_count`
>    - `clear/add zone_type`
>    - `clear/set/add` for DPT schemas and metrics
>    - `clear/set/add` for ZLM scales, rules, and metric bands
> 2. Added typed static stores for substrate script payloads:
>    - `USF_ZONE_TYPES`
>    - `USF_DPT_SCHEMAS_BY_SCALE`
>    - `USF_ZLM_SCALES_BY_SCALE`
> 3. `DefinitionRegistry` now hydrates from script-authored zone types and DPT schema overrides (deterministic scale-order application), while retaining baseline defaults when no script data exists.
> 4. `ZlmRegistry` now hydrates script scale-map overrides, includes explicit per-scale `fallback_zone`, and classifies against that fallback when no rule matches.
> 5. Strengthened ZLM validation:
>    - fallback zone must exist in known zone types
>    - revision must be > 0 and not behind schema revision
>    - rule metric ranges must be finite/ordered and metrics must exist in the scale schema
> 6. Updated `boot.rhai` to declare canonical substrate contracts for every scale (zone types, DPT metrics, ZLM rules) through `rhai_binding::usf_substrate`, and updated `docs/Scripting.md` with the new API.
>
> Validation after this phase:
> 1. `cargo test -p core_mod_api` passed (`136` tests).
> 2. `./build.sh dev` passed.
> 3. `./run.sh dev` still aborts in this sandbox with `failed to initiate panic, error 5` (exit `134`).

## Preserved Response Snapshot J (Near-Verbatim)
This snapshot is preserved from the follow-up substrate math cut replacing per-chunk hash randomness with deterministic coherent DPT sampling.

> I replaced DPT chunk metric generation from hash-per-chunk noise to a deterministic coherent field sampler:
> 1. `usf/dpt` now samples chunk metrics from continuous multi-octave trigonometric fields in normalized world space, seeded deterministically.
> 2. Canonical channels (`temperature`, `humidity`, `elevation`, `vegetation_density`) are now semantically coupled and spatially coherent instead of independent random values per chunk.
> 3. Extra metrics beyond the canonical four are generated as deterministic coherent channels as well.
> 4. Added DPT-focused tests:
>    - deterministic repeatability
>    - unit interval/finite guarantees
>    - neighboring-chunk smoothness relative to far chunks
>    - parent/child coherence
>
> Validation after this phase:
> 1. `cargo test -p core_mod_api` passed (`140` tests).
> 2. `./build.sh dev` passed.
> 3. `./run.sh dev` still aborts in this sandbox with `failed to initiate panic, error 5` (exit `134`).

## Preserved Response Snapshot K (Near-Verbatim)
This snapshot is preserved from the follow-up hard-cut where render proxy authority moved from chunk markers to realized zone extents.

> I completed the render-authority migration from chunk proxies to zone proxies:
> 1. Replaced `PhenomenonChunkProxy` with `PhenomenonZoneProxy { zone_id, representative_coord }` as the runtime marker component.
> 2. Replaced `sync_phenomenon_chunk_proxy_system` with `sync_phenomenon_zone_proxy_system`, wiring render proxy lifecycle directly to `ZoneRuntimeState.records` and `ZoneRealizationState.zone_to_phenomenon`.
> 3. Zone proxy assignment now derives transform, layer, window mode, and frontier seed from zone extents (`ZoneExtent`) and owning phenomenon lineage, rather than chunk coordinate ownership.
> 4. `update_phenomenon_model_surfaces_system` now filters zone proxies (`With<PhenomenonZoneProxy>`) and no longer depends on chunk proxy markers.
> 5. Replaced stale frontier-selection test usage with zone-based frontier selection coverage.
>
> Validation after this phase:
> 1. `cargo test -p core_mod_api` passed (`140` tests).
> 2. `./build.sh dev` passed.
> 3. `./run.sh dev` still aborts in this sandbox with `failed to initiate panic, error 5` (exit `134`).

## Preserved Response Snapshot L (Near-Verbatim)
This snapshot is preserved from the follow-up runtime hardening pass and render-authority simplification.

> I completed two structural follow-ups:
> 1. Hardened Rhai hook entrypoint fallback:
>    - `main(world, params)` execution now correctly falls back to `main(world)` for legacy hooks even when Rhai reports the missing function as untyped `main`.
>    - This removed the startup panic observed in outside-sandbox runs for `pre_startup.rhai`.
> 2. Removed global render-root authority:
>    - deleted `GlobalPhenomenonRoot`/`PhenomenonFrontierProxy` render marker usage
>    - removed `ensure_global_phenomenon_root_system`
>    - replaced `update_global_phenomenon_proxy_system` with `update_frontier_debug_stats_system`
>    - render scheduling now treats `sync_phenomenon_zone_proxy_system` as the authoritative proxy path
>    - surface updates now consume zone proxies directly with no global-root exclusion path.
>
> Validation after this phase:
> 1. `cargo test -p core_mod_api` passed (`140` tests).
> 2. `./build.sh dev` passed.
> 3. `./run.sh dev` (outside sandbox) now starts and runs startup/chunk workflows without the prior missing-`main` panic; manual interruption currently ends with workflow/channel abort logs.

## Execution-Only Plan Steps (Locked)
1. Substrate contracts and registries:
- Keep `usf/definition`, `usf/dpt`, `usf/zlm`, and `usf/zone` as canonical runtime substrate.
- Enforce strict invariant validation (domains, coverage, ordering, revisions).

2. Frontier state as authority:
- Render consumes frontier/node state only.
- Proxy frontier seeds bind to real phenomenon nodes (scale + distance aware).

3. Zone-driven realization:
- ZLM region hits map to zone realization intents.
- Intents resolve into full ECS `PhenomenonModel` trees.

4. Scripted scale behavior:
- Scale properties and ZLM logic are script-defined in Rhai and compiled into registries.

5. Static 3D hierarchical coherence target:
- One coherent static multi-scale structure across target scales.
- Chunk domain remains local `1000x1000x1000`.

6. Validation gate (always):
- `cargo test`
- `./build.sh dev`
- `./run.sh dev`
- never run `cargo clean`

## Current Operational State
1. Status is `review_hold` until explicit go-ahead for each major phase.
2. The preserved snapshots above are the current anchor for future implementation review.
