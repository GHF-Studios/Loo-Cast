use core_mod_api::usf::substrate as UsfSubstrate;

fn register_metric() {
    let scale_min = 0;
    let scale_max = UsfSubstrate::scale_level_count() - 1;
    UsfSubstrate::add_metric_typed(
        2,
        "root_pos_y",
        "f32",
        "position.root.y.normalized",
        "brick",
        true,
        scale_min,
        scale_max,
    );
}
