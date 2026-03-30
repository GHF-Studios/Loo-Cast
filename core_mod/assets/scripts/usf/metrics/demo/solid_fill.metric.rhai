use core_mod_api::usf::substrate as UsfSubstrate;

fn register_metric() {
    let scale_min = 0;
    let scale_max = UsfSubstrate::scale_level_count() - 1;
    UsfSubstrate::add_metric_typed(
        0,
        "solid_fill",
        "f32",
        "terrain.solid_fill.normalized",
        "brick",
        false,
        scale_min,
        scale_max,
    );
}
