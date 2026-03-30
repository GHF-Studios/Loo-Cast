use core_mod_api::usf::phenomenon as UsfPhenomenon;

fn register_phenomenon() {
    UsfPhenomenon::add_phenomenon("phenomenon.demo.surface", "metric_surface_debug");
}
