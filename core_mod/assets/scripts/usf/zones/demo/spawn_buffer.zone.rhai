use core_mod_api::usf::substrate as UsfSubstrate;
use core_mod_api::usf::zone as UsfZone;

fn register_zone() {
    UsfSubstrate::add_zone_type("spawn_buffer");
    UsfZone::set_selection_policy("spawn_buffer", "weighted_top_priority");
    UsfZone::set_density_profile("spawn_buffer", 0.0, 0.0, 0.0, 0.0);
}
