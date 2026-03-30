use core_mod_api::usf::substrate as UsfSubstrate;
use core_mod_api::usf::zone as UsfZone;

fn register_zone() {
    for zone_type in ["void", "forest", "wetland", "arid", "alpine"] {
        UsfSubstrate::add_zone_type(zone_type);
    }

    let mesh_debug_phenomenon_id = "phenomenon.placeholder.metric_surface_debug";
    UsfZone::add_supported_phenomenon("void", mesh_debug_phenomenon_id, 100, 1.0, "single_primary", 1);
    UsfZone::add_supported_phenomenon("arid", mesh_debug_phenomenon_id, 100, 1.0, "single_primary", 1);
    UsfZone::add_supported_phenomenon("alpine", mesh_debug_phenomenon_id, 100, 1.0, "single_primary", 1);
    UsfZone::add_supported_phenomenon("forest", mesh_debug_phenomenon_id, 100, 1.0, "single_primary", 1);
    UsfZone::add_supported_phenomenon("wetland", mesh_debug_phenomenon_id, 100, 1.0, "single_primary", 1);
    UsfZone::set_selection_policy("void", "weighted_top_priority");
    UsfZone::set_selection_policy("arid", "weighted_top_priority");
    UsfZone::set_selection_policy("alpine", "weighted_top_priority");
    UsfZone::set_selection_policy("forest", "weighted_top_priority");
    UsfZone::set_selection_policy("wetland", "weighted_top_priority");

    UsfZone::set_density_profile("void", 0.15, 0.00, 0.00, 0.25);
    UsfZone::set_density_profile("arid", 0.45, 0.05, 0.00, 0.55);
    UsfZone::set_density_profile("alpine", 0.55, 0.10, 0.00, 0.72);
    UsfZone::set_density_profile("forest", 0.72, 0.14, 0.05, 0.88);
    UsfZone::set_density_profile("wetland", 0.78, 0.20, 0.10, 0.95);
}
