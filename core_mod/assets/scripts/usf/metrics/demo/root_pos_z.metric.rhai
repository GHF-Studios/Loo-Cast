use core_mod_api::usf::substrate as UsfSubstrate;

fn register_metric() {
    let scale_min = 0;
    let scale_max = UsfSubstrate::scale_level_count() - 1;
    UsfSubstrate::add_metric_typed(
        3,
        "root_pos_z",
        "f32",
        "position.root.z.normalized",
        "brick",
        true,
        scale_min,
        scale_max,
    );
}
