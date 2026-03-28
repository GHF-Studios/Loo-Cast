# USF Typed Script Bootstrap

The Rhai engine now loads USF content by script type from this tree.

Each file must expose the matching entrypoint:

- `*.metric.rhai` -> `register_metric()`
- `*.zone.rhai` -> `register_zone()`
- `*.metric_set.rhai` -> `register_metric_set()`
- `*.dpt_sampler.rhai` -> `register_dpt_sampler()`
- `*.dpt_categorizer.rhai` -> `register_dpt_categorizer()`
- `*.zlm.rhai` -> `register_zlm()`
- `*.scale.rhai` -> `register_scale()`
- `*.phenomenon.rhai` -> `register_phenomenon()`
- `*.phenomenon_model.rhai` -> `register_phenomenon_model()`

Current backend support is implemented for:

- metric registry
- metric-set registry
- zone registration
- zone density profiles
- DPT sampler/categorizer registration
- ZLM registration
- scale binding + DPT schema derivation from metric sets
- phenomenon registry
- zone-supported phenomena (priority/weight/spawn policy/max_active) + selection policy
- phenomenon-model registry (including primary-model assignment)

`max_active` currently applies per `(zone_type, phenomenon_id)` support entry for top-scale
zone realization. When the cap is reached, additional zones skip spawning for that support.
