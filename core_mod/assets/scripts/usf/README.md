# USF Typed Script Bootstrap

The Rhai engine now loads USF content by script type from this tree.

Each file must expose the matching entrypoint:

- `*.metric.rhai` -> `register_metric()`
- `*.zone.rhai` -> `register_zone()`
- `*.metric_set.rhai` -> `register_metric_set()`
- `*.mod.rhai` -> `register_mod()`
- `*.modpack.rhai` -> `register_modpack()`
- `*.zlm.rhai` -> `register_zlm()`
- `*.scale.rhai` -> `register_scale()`
- `*.phenomenon.rhai` -> `register_phenomenon()`
- `*.phenomenon_model.rhai` -> `register_phenomenon_model()`

Mod ownership is path-based for non-global script types:

- `usf/<type>/<file>.rhai` -> belongs to `mod.placeholder_gameplay.v1` (legacy/default mod)
- `usf/<type>/<mod.id>/.../*.rhai` -> belongs to that mod id

Current backend support is implemented for:

- metric registry
- metric-set registry
- zone registration
- zone density profiles
- USF mod registration (default + config-key activation policy)
- USF modpack registration (mod routing)
- ZLM registration
- scale binding + DPT schema derivation from metric sets
- phenomenon registry
- zone-supported phenomena (priority/weight/spawn policy/max_active) + selection policy
- phenomenon-model registry (including primary-model assignment)
- fixed engine kernels for DPT sampling + DPT categorization (scripts select kernel IDs; they do not register new kernels)

Composition policy is strict:

- active modpack is selected via config key `usf_content/active_modpack_id`
- the active modpack is the authoritative mod catalogue for runtime composition
- modpack mod order is authoritative; mod activation is evaluated in modpack order
- dependency/load-after graph is resolved deterministically after activation (`depends_on`, `load_after`, `priority`)
- mod conflicts (`conflicts_with`) are hard errors when both mods are enabled
- only mods enabled by their config keys are composed
- additive-key collisions are hard-error; singleton domains (`scale_binding`, `dpt_schema`, `zlm`) use owner policy (`hard_error`/`replace`/`replace_if_higher_priority`)
- no engine-level fallback mod/modpack/schema/binding generation exists

Script authority constraints (current contracts):

- `*.metric_set.rhai` should explicitly list metric members with `add_metric_set_metric(...)`; avoid implicit "all metrics" expansion
- DPT sampler kernel resolves metric values by script metric semantics (`semantics_tag`/`name`), not by hardcoded metric index position
- each `*.mod.rhai` must declare a mod manifest (required metrics/sets/zones/phenomena/models plus required per-scale bindings/schemas/ZLMs)
- mod-level metadata is declared in `*.mod.rhai` via:
  - `set_usf_mod_priority(...)`
  - `add_usf_mod_dependency(...)`
  - `add_usf_mod_load_after(...)`
  - `add_usf_mod_conflict(...)`
  - `set_usf_mod_singleton_conflict_policy(...)`

`max_active` currently applies per `(zone_type, phenomenon_id)` support entry for top-scale
zone realization. When the cap is reached, additional zones skip spawning for that support.

Current placeholder gameplay contracts:

- one metric drives classification: `solid_fill`
- two zones are used: `empty` (no support/no mesh) and `solid` (spawns one surface phenomenon)
- the chunk terrain debug mesh is driven by one phenomenon id: `phenomenon.placeholder.metric_surface_debug`
- `*.phenomenon_model.rhai` defines the model field policy via `set_metric_surface_debug_model_field(...)`
- meshing/collider generation remains engine-owned; scripts declare phenomenon + model policy contracts
