use core_mod_api::usf::phenomenon as UsfPhenomenon;

fn register_phenomenon_model() {
    let model_id = "phenomenon_model.demo.surface.primary";
    let phenomenon_id = "phenomenon.demo.surface";
    UsfPhenomenon::add_phenomenon_model(
        model_id,
        phenomenon_id,
    );
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
