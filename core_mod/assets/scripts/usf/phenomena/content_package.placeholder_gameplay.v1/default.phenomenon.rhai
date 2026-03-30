use core_mod_api::usf::phenomenon as UsfPhenomenon;

fn register_phenomenon() {
    let phenomenon_id = "phenomenon.placeholder.metric_surface_debug";
    UsfPhenomenon::add_phenomenon(phenomenon_id, "metric_surface_debug");
    // Field function contract (authoritative content data):
    // meshing/collider/material behavior remains engine-side kernel functionality.
    UsfPhenomenon::set_metric_surface_debug_field(
        phenomenon_id,
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
}
