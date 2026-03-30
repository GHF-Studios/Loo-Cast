use core_mod_api::usf::substrate as UsfSubstrate;
use core_mod_api::usf::zone as UsfZone;

fn register_zone() {
    for zone_type in ["empty", "solid"] {
        UsfSubstrate::add_zone_type(zone_type);
    }

    let solid_volume_phenomenon_id = "phenomenon.placeholder.metric_surface_debug";
    UsfZone::add_supported_phenomenon("solid", solid_volume_phenomenon_id, 100, 1.0, "single_primary", 1);

    UsfZone::set_selection_policy("empty", "weighted_top_priority");
    UsfZone::set_selection_policy("solid", "weighted_top_priority");

    // Empty is a true no-op: no support entries and zero density.
    UsfZone::set_density_profile("empty", 0.0, 0.0, 0.0, 0.0);
    // Solid uses script-defined phenomenon-model field policy + density shaping.
    UsfZone::set_density_profile("solid", 1.0, 0.0, 0.0, 1.0);
}
