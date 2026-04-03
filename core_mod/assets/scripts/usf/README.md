# USF Typed Script Bootstrap

The Rhai engine now loads USF content by script type from this tree.

Each file must expose the matching entrypoint (typed ctx-first):

- `*.metric.rhai` -> `register_metric(ctx)`
- `*.zone.rhai` -> `register_zone(ctx)`
- `*.metric_set.rhai` -> `register_metric_set(ctx)`
- `*.mod.rhai` -> `register_mod(ctx)`
- `*.modpack.rhai` -> `register_modpack(ctx)`
- `*.zlm.rhai` -> `register_zlm(ctx)`
- `*.scale.rhai` -> `register_scale(ctx)`
- `*.phenomenon.rhai` -> `register_phenomenon(ctx)`
- `*.phenomenon_model.rhai` -> `register_phenomenon_model(ctx)`

Typed ctx injection (current state):

- `register_mod(ctx)` receives `UsfModScriptCtx`
- `register_modpack(ctx)` receives `UsfModpackScriptCtx`
- `register_metric(ctx)` receives `UsfMetricScriptCtx`
- `register_metric_set(ctx)` receives `UsfMetricSetScriptCtx`
- `register_zone(ctx)` receives `UsfZoneScriptCtx`
- `register_zlm(ctx)` receives `UsfZlmScriptCtx`
- `register_scale(ctx)` receives `UsfScaleScriptCtx`
- `register_phenomenon(ctx)` receives `UsfPhenomenonScriptCtx`
- `register_phenomenon_model(ctx)` receives `UsfPhenomenonModelScriptCtx`

Authoring rule:

- each `*.metric.rhai` file defines exactly one metric
- each `*.zone.rhai` file defines exactly one zone type contract
- each `*.phenomenon.rhai` file defines exactly one phenomenon
- each `*.phenomenon_model.rhai` file defines exactly one phenomenon model
- each `*.mod.rhai` file defines exactly one mod

Mod ownership is path-based for non-global script types:

- `usf/<type>/<mod.id>/.../*.rhai` -> belongs to that mod id
- root-level package scripts are invalid and fail bootstrap (`usf/<type>/<file>.rhai` is no longer allowed)

Current backend support is implemented for:

- metric registry
- metric-set registry
- zone registration
- zone density profiles
- USF mod registration (modpack-owned activation)
- USF modpack registration (mod routing)
- ZLM registration
- scale contract + DPT schema derivation from metric sets
- phenomenon registry
- zone-supported phenomena (priority/weight/spawn policy/max_active) + selection policy
- phenomenon-model registry with explicit scale-index selection
- explicit model topology/support metadata (`monolithic_chunk` / `partitioned_by_chunk` + support radius)
- fixed engine kernels for DPT sampling + DPT categorization (scripts must register kernel IDs they reference via scale ctx; kernel implementations remain engine-owned)

Composition policy is strict:

- active modpack is selected via config key `usf/active_modpack_id`
- the active modpack is the authoritative mod catalogue for runtime composition
- modpack mod order is authoritative
- dependency/load-after graph is resolved deterministically after activation (`depends_on`, `load_after`, `priority`)
- mod conflicts (`conflicts_with`) are hard errors when both mods are enabled
- additive-key collisions are hard-error; singleton domains (`scale`, `dpt_schema`, `zlm`) use owner policy (`hard_error`/`replace`/`replace_if_higher_priority`)
- no engine-level fallback mod/modpack/schema/binding generation exists

Script authority constraints (current contracts):

- `*.metric_set.rhai` should explicitly list metric members with `add_metric_set_metric(...)`; avoid implicit "all metrics" expansion
- DPT sampler kernel resolves metric values by script metric semantics (`semantics_tag`/`name`), not by hardcoded metric index position
- per-domain script ctx registration now auto-populates mod manifest ownership requirements
- phenomenon capabilities are now explicit script metadata (defaulted from kind on `ctx.register(...)`, optionally extended via `ctx.add_capability(...)`)
- `*.mod.rhai` currently controls mod metadata/policies (priority/dependencies/conflicts/singleton policies)
- scale declaration helpers support `single`, `range`, `set`, and `all` selection styles for `scale`, `dpt_schema`, and `zlm` requirements
- mod-level metadata is declared in `*.mod.rhai` via:
  - `ctx.set_priority(...)`
  - `ctx.depends_on(...)`
  - `ctx.load_after(...)`
  - `ctx.conflicts_with(...)`
  - `ctx.set_singleton_conflict_policy(...)`

`max_active` currently applies per `(zone_type, phenomenon_id)` support entry in the active
zone-realization window. When the cap is reached, additional zones skip spawning for that support.

Current placeholder gameplay contracts:

- one terrain metric drives classification: `demo_mass_density`
- three derived root-position metrics are provided: `root_pos_x`, `root_pos_y`, `root_pos_z`
- three zones are used: `empty` (no support/no manifestation instance), `spawn_buffer` (near-origin noop), and `solid` (spawns one manifestation-density phenomenon)
- the chunk manifestation instance contract is currently driven by one phenomenon id: `demo_manifestation_density`
- demo phenomenon/model scripts declare non-mesh contracts (`audio emitter`, `particle emitter`, `interaction trigger`) to validate capability-family extensibility end-to-end
- `*.phenomenon_model.rhai` defines the model field policy via `set_manifestation_density_field(...)`
- `*.phenomenon_model.rhai` can define model material policy via `set_manifestation_material_profile(...)`
- `*.phenomenon_model.rhai` can define optional capability contracts via:
  - `set_simulation_service(...)`
  - `set_manifestation_audio_emitter(...)`
  - `set_manifestation_particle_emitter(...)`
  - `set_interaction_trigger(...)`
- `*.phenomenon_model.rhai` must declare topology/support with:
  - `ctx.set_topology("monolithic_chunk" | "partitioned_by_chunk")`
  - `ctx.set_support_chunk_radius(...)` (required `>= 1` for partitioned)
- meshing/material/collider application is engine-owned capability code; scripts declare phenomenon + model policy contracts only
