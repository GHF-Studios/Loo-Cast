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

Current backend support is implemented for zone, DPT sampler/categorizer, ZLM, and scale substrate registration.
