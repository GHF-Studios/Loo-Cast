# USF Content Scripting Cheat Sheet (Draft Contract)

Purpose: define a scalable, mod-friendly content ABI for USF.
If this page is enough to author content, the API is good.

---

## 1. Core Model

- `Phenomenon`: stable identity (logical object).
- `Model`: concrete ECS realization of a phenomenon at a specific scale. Can be paramaterized over scale if the model is not vastly different at different scales (newtonian gravity for example).
- `Scale`: binds one metric set, which defined the required DPT/ZLM shape, and we also bundle a pointer/reference to the DPT data for that scale, stored per chunk however, and we ofc store a compatible ZLM directly in the scale to be able to categorize the chunks' DPM chunks.
- `MetricSet`: ordered metric layout for a scale (dynamic dimensionality, aka number of metrics can be whatever we want per scale).
- `DPT sampler Function`: deterministic sampler `f(context) -> dpt_chunk`, but it should be generic if possible as well, so we can just plug in the DPT chunks one by one, and provide the correct ZLM, and boom we get out structured command requests or something that will actually spawn Phenomena and actually generate the world and shit, yk? but I mean like, have it be generic if possible if it supports a range of scales instead of just a single scale.
- `ZLM`: metric-space classifier `dpt_chunk -> ztm_chunk`. Basically an n-dimensional shape that must be fully filled and not have any undefiend/empty regions, so it can act as an n-dimensional lookup table for zone types for a specific metric set's concrete value configuration. 17 metrics in a scale means the ZLM needs to fully cover 17 dimensions of metrics.
- `ZTM`: Zone Type Map with the resolved zone type for each data point in 3D space.
- `ZoneType`: semantic region/"zone" class that selects behavior/spawn presets and in the future will act as an autonomous integrated interactive entity-bound manifestation of specific DPT concrete data configurations. Can also work across scales, but ofc only if the metric itself can make sense at a range of scales without needing separate metric types, yk?

Authoritative flow:
---
`ScaleGen -> ScaledChunkGen -> DPT_Sampler_Function::<Scale<MetricSet = Whatever, DPTSampler = Something, ZLM = TheThing>>(scale, chunk_id) => dpt_chunk -> DPT_Categorizer_Function::<>(dpt_chunk, zlm) => ztm, and then the ztm is an autonomous entity that must be programmed via scripting definitions to actually spawn things and like it has the authority to do basically anything.`

## 2. Script Types (Canonical File Archetypes)

Proposed canonical layout:

```text
assets/scripts/usf/
  metrics/*.metric.rhai
  zones/*.zone.rhai
  metric_sets/*.metric_set.rhai
  zlms/*.zlm.rhai
  scales/*.scale.rhai
  phenomena/*.phenomenon.rhai
  phenomenon_models/*.phenomenon_model.rhai
  dpt_samplers/*.dpt_sampler.rhai
  dpt_categorizers/*.dpt_categorizer.rhai
assets/scripts/ecs/
  schedule_entrypoints/*.schedule_entrypoint.rhai 
```

Each file type has one entrypoint:

- `*.metric.rhai`: `fn register_metric(ctx) { ... }`
- `*.zone.rhai`: `fn register_zone(ctx) { ... }`
- `*.metric_set.rhai`: `fn register_metric_set(ctx) { ... }`
- `*.zlm.rhai`: `fn register_zlm(ctx) { ... }`
- `*.scale.rhai`: `fn register_scale(ctx) { ... }`
- `*.phenomenon.rhai`: `fn register_phenomenon(ctx) { ... }`
- `*.phenomenon_model.rhai`: `fn register_phenomenon_model(ctx) { ... }`
- `*.dpt_sampler.rhai`: `fn register_dpt_sampler(ctx) { ... }`
- `*.dpt_categorizer.rhai`: `fn register_dpt_categorizer(ctx) { ... }`
- `*.schedule_entrypoint.rhai`: `fn run(world, params) { ... }`

Rule: domain APIs are isolated. A script type get's any functions or types or methods or anything it needs via the context.
ECS Schedule hooks are another type of script, not directly related to the USF however.

---

## 3. ID Rules

- Examples:
  - `metric.generic_mass_density` (as a test/debug metric ofc, hence "generic_*")
  - `zone.high_generic_mass_density`, `zone.low_generic_mass_density`, `zone.medium_generic_mass_density` (as a test/debug zone ofc, hence "generic_*")
  - `metric_set.quetta_meter_1000`
  - `zlm.quetta_meter_1000`
  - `scale.quetta_meter_1000`
  - `phenomenon.fractal_cube`
  - `phenomenon_model.fractal_cube.quetta_meter_1000_to_quetta_meter_10`
  - `dpt_sampler.quetta_meter_1000`
  - `dpt_categorizer.quetta_meter_1000`

---

## 4. Minimal Example (Fractal Cube Debug Stack)

```rhai
// metrics/generic_mass_density.metric.rhai
fn register_metric(ctx) {
  ctx.metric_float("metric.generic_mass_density", 0.0, 1.0);
}
```

```rhai
// metric_sets/quetta_meter_1000.metric_set.rhai
fn register_metric_set(ctx) {
  ctx.metric_set("metric_set.quetta_meter_1000", [
    "metric.generic_mass_density",
  ]);
}
```

```rhai
// dpt_samplers/quetta_meter_1000.dpt_sampler.rhai
fn register_dpt_sampler(ctx) {
  ctx.dpt_sampler("dpt_sampler.quetta_meter_1000", |sample_ctx| {
    let d = fractal_cube_density(sample_ctx.world_seed, sample_ctx.world_pos);
    #{
      "metric.generic_mass_density": d
    } // translation layer compiles this to dense metric vectors
  });
}
```

```rhai
// zones/generic_mass_density.zone.rhai
fn register_zone(ctx) {
  ctx.zone("zone.low_generic_mass_density");
  ctx.zone("zone.medium_generic_mass_density");
  ctx.zone("zone.high_generic_mass_density");
}
```

```rhai
// zlms/quetta_meter_1000.zlm.rhai
fn register_zlm(ctx) {
  ctx.zlm("zlm.quetta_meter_1000", "metric_set.quetta_meter_1000")
    .zone("zone.low_generic_mass_density")
    .band("metric.generic_mass_density", 0.00, 0.33)
    .zone("zone.medium_generic_mass_density")
    .band("metric.generic_mass_density", 0.33, 0.66)
    .zone("zone.high_generic_mass_density")
    .band("metric.generic_mass_density", 0.66, 1.00);
}
```

```rhai
// dpt_categorizers/quetta_meter_1000.dpt_categorizer.rhai
fn register_dpt_categorizer(ctx) {
  ctx.dpt_categorizer("dpt_categorizer.quetta_meter_1000", |dpt_chunk, zlm_id| {
    zlm_apply(zlm_id, dpt_chunk) // returns ZTM chunk
  });
}
```

```rhai
// scales/quetta_meter_1000.scale.rhai
fn register_scale(ctx) {
  ctx.scale("scale.quetta_meter_1000")
    .index(5)
    .metric_set("metric_set.quetta_meter_1000")
    .dpt_sampler("dpt_sampler.quetta_meter_1000")
    .dpt_categorizer("dpt_categorizer.quetta_meter_1000")
    .zlm("zlm.quetta_meter_1000");
}
```

```rhai
// phenomena/fractal_cube.phenomenon.rhai
fn register_phenomenon(ctx) {
  ctx.phenomenon("phenomenon.fractal_cube")
    .root_model("phenomenon_model.fractal_cube.quetta_meter_1000_to_quetta_meter_10");
}
```

```rhai
// phenomenon_models/fractal_cube.phenomenon_model.rhai
fn register_phenomenon_model(ctx) {
  ctx.phenomenon_model("phenomenon_model.fractal_cube.quetta_meter_1000_to_quetta_meter_10")
    .phenomenon("phenomenon.fractal_cube")
    .scale_range("scale.quetta_meter_1000", "scale.quetta_meter_10")
    .spawn_policy("zone_driven");
}
```

---

## 5. DPT Generation + Dynamic Dimensionality

Operational contract:

- `dpt_sampler` is the generator for child DPT chunks when zooming into finer detail.
- `dpt_categorizer` transforms a DPT chunk into a ZTM chunk using a compatible `zlm`.
- `metric_set` is a separate scale contract that defines metric layout; sampler/categorizer do not define metric-set structure.
- `metric_set` compiles into an ordered metric index table.
- sampler authoring may stay readable (`metric_id -> value`), while backend compiles to dense vectors of length `metric_set.metric_count`.
- storage remains dense (`Vec<f32>` or packed equivalent) plus explicit layout metadata.
- `zlm` and `dpt_categorizer` run through structured, typed APIs per metric-set contract.
- validation fails if metrics are missing, duplicated, unknown, or incompatible with the selected `metric_set`.

This preserves clean content authoring while solving N-dimensional metric-space evaluation generically.

---

## 6. Hard Invariants

- `dpt_sampler` is deterministic for same `(seed, scale, chunk_id, sample_position, params)`.
- `dpt_categorizer` is deterministic for same `(dpt_chunk, zlm)`.
- `metric_set` ordering is immutable inside a given ID/version.
- every `zlm` must fully cover its metric-space ranges (no uncategorized holes).
- every `scale` must resolve all required references (`metric_set`, `dpt_sampler`, `dpt_categorizer`, `zlm`) and define chunk storage/retrieval binding.
- zoom authority is world/scale transitions with visual continuity that feels like scaling the world around the player, not camera-FOV authority.
- only active scale is fully simulated.
- upper scales are inactive/paused and reactively updated/rendered.
- lower-scale detail is not simulated until zoomed into (manifested), and is removed again when zooming back out.

---

## 7. Content Author Checklist

1. Define metrics (`*.metric.rhai`).
2. Define zone types (`*.zone.rhai`).
3. Define metric sets (`*.metric_set.rhai`).
4. Define ZLMs (`*.zlm.rhai`).
5. Define DPT samplers (`*.dpt_sampler.rhai`).
6. Define DPT categorizers (`*.dpt_categorizer.rhai`).
7. Define scale configs (`*.scale.rhai`).
8. Define phenomena (`*.phenomenon.rhai`).
9. Define phenomenon models (`*.phenomenon_model.rhai`).
10. Wire ECS hooks only where runtime behavior is needed (`*.schedule_entrypoint.rhai`).

If these steps are enough to ship content, the scripting API is doing its job.

---

## 8. Translation Layer Scope (What Backend Must Guarantee)

- load each script type in its own domain context with strict entrypoint contracts.
- expose domain-specific context APIs per script type (no one-size-fits-all context surface).
- validate every ID reference at load time and fail fast with actionable diagnostics.
- compile script definitions into immutable registries (hot-reloadable snapshots).
- keep runtime systems data-driven from registries, not ad-hoc script overrides.
- provide deterministic sampling/categorization APIs for chunk generation.
- support scalability targets via streaming/caching/chunked evaluation.

This document is the contract for the backend/content binding layer.
