# USF Typed Script Bootstrap

The Rhai engine now loads USF content by script type from this tree.

Each file must expose the matching entrypoint:

- `*.metric.rhai` -> `register_metric()`
- `*.zone.rhai` -> `register_zone()`
- `*.metric_set.rhai` -> `register_metric_set()`
- `*.dpt_sampler.rhai` -> `register_dpt_sampler()`
- `*.dpt_categorizer.rhai` -> `register_dpt_categorizer()`
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
- DPT sampler/categorizer registration
- USF content-package registration (default + config-key activation policy)
- USF content-profile registration (content package routing)
- ZLM registration
- scale binding + DPT schema derivation from metric sets
- phenomenon registry
- zone-supported phenomena (priority/weight/spawn policy/max_active) + selection policy
- phenomenon-model registry (including primary-model assignment)

Composition policy is strict:

- active profile is selected via config key `usf_content/active_profile_id`
- the active profile defines ordered package IDs
- package merges are hard-error on duplicate keys across selected packages

`max_active` currently applies per `(zone_type, phenomenon_id)` support entry for top-scale
zone realization. When the cap is reached, additional zones skip spawning for that support.
