use core_mod_api::usf::substrate as UsfSubstrate;

fn register_zone() {
    for zone_type in ["void", "forest", "wetland", "arid", "alpine"] {
        UsfSubstrate::add_zone_type(zone_type);
    }
}
