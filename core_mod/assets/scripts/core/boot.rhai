use rhai_binding::schedule_hooks as ScheduleHooks;
use rhai_binding::usf_substrate as UsfSubstrate;
use rhai_binding::usf_zone as UsfZone;

for hook in [
    "pre_startup",
    "startup",
    "post_startup",
    "first",
    "pre_update",
    "substrate_pre_update",
    "zone_pre_update",
    "phenomenon_pre_update",
    "update",
    "substrate_update",
    "zone_update",
    "phenomenon_update",
    "post_update",
    "last",
] {
    ScheduleHooks::add(hook);
}

let canonical_zones = ["void", "forest", "wetland", "arid", "alpine"];

UsfSubstrate::clear_zone_types();
for zone_type in canonical_zones {
    UsfSubstrate::add_zone_type(zone_type);
}

UsfSubstrate::clear_dpt_schemas();
UsfSubstrate::clear_zlm_maps();

let scale_level_count = UsfSubstrate::scale_level_count();
for scale_index in 0..scale_level_count {
    UsfSubstrate::set_dpt_schema(scale_index, 1, "void");
    UsfSubstrate::add_dpt_metric(scale_index, 0, "temperature", true);
    UsfSubstrate::add_dpt_metric(scale_index, 1, "humidity", true);
    UsfSubstrate::add_dpt_metric(scale_index, 2, "elevation", true);
    UsfSubstrate::add_dpt_metric(scale_index, 3, "vegetation_density", false);

    UsfSubstrate::set_zlm_scale(scale_index, 1, "void");
    let wetland_rule = UsfSubstrate::add_zlm_rule(scale_index, "wetland");
    UsfSubstrate::add_zlm_metric_band(scale_index, wetland_rule, 1, 0.70, 1.0);
    UsfSubstrate::add_zlm_metric_band(scale_index, wetland_rule, 2, 0.0, 0.35);

    let forest_rule = UsfSubstrate::add_zlm_rule(scale_index, "forest");
    UsfSubstrate::add_zlm_metric_band(scale_index, forest_rule, 1, 0.45, 1.0);
    UsfSubstrate::add_zlm_metric_band(scale_index, forest_rule, 0, 0.20, 0.85);

    let alpine_rule = UsfSubstrate::add_zlm_rule(scale_index, "alpine");
    UsfSubstrate::add_zlm_metric_band(scale_index, alpine_rule, 2, 0.75, 1.0);

    let arid_rule = UsfSubstrate::add_zlm_rule(scale_index, "arid");
    UsfSubstrate::add_zlm_metric_band(scale_index, arid_rule, 1, 0.0, 0.30);
    UsfSubstrate::add_zlm_metric_band(scale_index, arid_rule, 0, 0.45, 1.0);
}

UsfZone::clear_phenomenon_kinds();
UsfZone::set_phenomenon_kind("void", "mandelbulb");
UsfZone::set_phenomenon_kind("arid", "mandelbulb");
UsfZone::set_phenomenon_kind("alpine", "mandelbulb");
UsfZone::set_phenomenon_kind("forest", "sierpinski_sponge");
UsfZone::set_phenomenon_kind("wetland", "sierpinski_sponge");
