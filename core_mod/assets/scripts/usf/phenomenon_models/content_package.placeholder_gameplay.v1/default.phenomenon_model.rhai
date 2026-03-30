use core_mod_api::usf::phenomenon as UsfPhenomenon;

fn register_phenomenon_model() {
    UsfPhenomenon::add_phenomenon_model(
        "phenomenon_model.placeholder.metric_surface_debug.primary",
        "phenomenon.placeholder.metric_surface_debug",
    );

    UsfPhenomenon::set_primary_model(
        "phenomenon.placeholder.metric_surface_debug",
        "phenomenon_model.placeholder.metric_surface_debug.primary",
    );
}
