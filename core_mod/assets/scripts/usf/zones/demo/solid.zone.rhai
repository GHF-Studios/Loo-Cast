use core_mod_api::usf::substrate as UsfSubstrate;
use core_mod_api::usf::zone as UsfZone;

fn register_zone() {
    UsfSubstrate::add_zone_type("solid");
    UsfZone::add_supported_phenomenon(
        "solid",
        "phenomenon.demo.surface",
        100,
        1.0,
        "single_primary",
        1,
    );
    UsfZone::set_selection_policy("solid", "weighted_top_priority");
    UsfZone::set_density_profile("solid", 1.0, 0.0, 0.0, 1.0);
}
