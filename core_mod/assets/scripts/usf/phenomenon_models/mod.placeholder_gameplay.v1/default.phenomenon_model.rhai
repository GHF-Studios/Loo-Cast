use core_mod_api::usf::phenomenon as UsfPhenomenon;

fn register_phenomenon_model() {
    let model_id = "phenomenon_model.placeholder.metric_surface_debug.primary";
    let phenomenon_id = "phenomenon.placeholder.metric_surface_debug";
    UsfPhenomenon::add_phenomenon_model(
        model_id,
        phenomenon_id,
    );
    // Model-level field policy: this is the script-side execution policy contract
    // for how the solid zone's surface field is shaped.
    UsfPhenomenon::set_metric_surface_debug_model_field(
        model_id,
        320.0,
        128.0,
        0.82,
        0.18,
        0.66,
        3.0,
        0.5,
        608139381523021511,
        2748924730191034777,
    );

    UsfPhenomenon::set_primary_model(phenomenon_id, model_id);
}
