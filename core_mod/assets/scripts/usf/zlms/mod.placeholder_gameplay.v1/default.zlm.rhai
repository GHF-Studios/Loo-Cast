use core_mod_api::usf::substrate as UsfSubstrate;

fn register_zlm() {
    let scale_level_count = UsfSubstrate::scale_level_count();
    for scale_index in 0..scale_level_count {
        UsfSubstrate::set_zlm_scale(scale_index, 1, "empty");
        let solid_rule = UsfSubstrate::add_zlm_rule(scale_index, "solid");
        UsfSubstrate::add_zlm_metric_band(scale_index, solid_rule, 0, 0.52, 1.0);
    }
}
