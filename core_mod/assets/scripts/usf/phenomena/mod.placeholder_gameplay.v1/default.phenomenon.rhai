use core_mod_api::usf::phenomenon as UsfPhenomenon;

fn register_phenomenon() {
    let phenomenon_id = "phenomenon.placeholder.metric_surface_debug";
    UsfPhenomenon::add_phenomenon(phenomenon_id, "metric_surface_debug");
}
