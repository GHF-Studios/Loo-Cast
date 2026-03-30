use core_mod_api::usf::substrate as UsfSubstrate;

fn register_metric() {
    let scale_min = 0;
    let scale_max = UsfSubstrate::scale_level_count() - 1;
    UsfSubstrate::add_metric_typed(
        1,
        "root_pos_x",
        "f32",
        "position.root.x.normalized",
        "brick",
        true,
        scale_min,
        scale_max,
    );
}
