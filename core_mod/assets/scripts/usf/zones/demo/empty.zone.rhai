use core_mod_api::usf::substrate as UsfSubstrate;
use core_mod_api::usf::zone as UsfZone;

fn register_zone() {
    UsfSubstrate::add_zone_type("empty");
    UsfZone::set_selection_policy("empty", "weighted_top_priority");
    UsfZone::set_density_profile("empty", 0.0, 0.0, 0.0, 0.0);
}
