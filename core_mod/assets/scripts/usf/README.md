# USF Typed Script Bootstrap

The Rhai engine now loads USF content by script type from this tree.

Each file must expose the matching entrypoint:

- `*.metric.rhai` -> `register_metric()`
- `*.zone.rhai` -> `register_zone()`
- `*.metric_set.rhai` -> `register_metric_set()`
- `*.content_package.rhai` -> `register_content_package()`
- `*.content_profile.rhai` -> `register_usf_content_profile()`
- `*.zlm.rhai` -> `register_zlm()`
- `*.scale.rhai` -> `register_scale()`
- `*.phenomenon.rhai` -> `register_phenomenon()`
- `*.phenomenon_model.rhai` -> `register_phenomenon_model()`

Package ownership is path-based for non-global script types:

- `usf/<type>/<file>.rhai` -> belongs to `content_package.placeholder_gameplay.v1` (legacy/default package)
- `usf/<type>/<content_package.id>/.../*.rhai` -> belongs to that package id

Current backend support is implemented for:

- metric registry
- metric-set registry
- zone registration
- zone density profiles
- USF content-package registration (default + config-key activation policy)
- USF content-profile registration (content package routing)
- ZLM registration
- scale binding + DPT schema derivation from metric sets
- phenomenon registry
- zone-supported phenomena (priority/weight/spawn policy/max_active) + selection policy
- phenomenon-model registry (including primary-model assignment)
- fixed engine kernels for DPT sampling + DPT categorization (scripts select kernel IDs; they do not register new kernels)

Composition policy is strict:

- active profile is selected via config key `usf_content/active_profile_id`
- the active profile is the authoritative package catalogue for runtime composition
- profile package order is authoritative; package activation is evaluated in profile order
- only packages enabled by their config keys are composed
- package merges are hard-error on duplicate keys across selected packages
- no engine-level fallback package/profile/schema/binding generation exists

Script authority constraints (current contracts):

- `*.metric_set.rhai` should explicitly list metric members with `add_metric_set_metric(...)`; avoid implicit "all metrics" expansion
- DPT sampler kernel resolves metric values by script metric semantics (`semantics_tag`/`name`), not by hardcoded metric index position

`max_active` currently applies per `(zone_type, phenomenon_id)` support entry for top-scale
zone realization. When the cap is reached, additional zones skip spawning for that support.

Current placeholder gameplay contracts:

- the chunk terrain debug mesh is driven by a single phenomenon id: `phenomenon.placeholder.metric_surface_debug`
- that phenomenon must use kind `metric_surface_debug`
- `*.phenomenon.rhai` must also define the `metric_surface_debug` field function parameters via `set_metric_surface_debug_field(...)`
- meshing/collider generation remains engine-owned; scripts declare phenomenon function contract + routing
