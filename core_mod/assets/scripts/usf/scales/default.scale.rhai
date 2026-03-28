use core_mod_api::usf::substrate as UsfSubstrate;

fn register_scale() {
    let scale_level_count = UsfSubstrate::scale_level_count();
    for scale_index in 0..scale_level_count {
        UsfSubstrate::set_dpt_schema(scale_index, 1, "void");
        UsfSubstrate::add_dpt_metric(scale_index, 0, "temperature", true);
        UsfSubstrate::add_dpt_metric(scale_index, 1, "humidity", true);
        UsfSubstrate::add_dpt_metric(scale_index, 2, "elevation", true);
        UsfSubstrate::add_dpt_metric(scale_index, 3, "vegetation_density", false);

        UsfSubstrate::set_zlm_scale(scale_index, 1, "void");
        let wetland_rule = UsfSubstrate::add_zlm_rule(scale_index, "wetland");
        UsfSubstrate::add_zlm_metric_band(scale_index, wetland_rule, 1, 0.70, 1.0);
        UsfSubstrate::add_zlm_metric_band(scale_index, wetland_rule, 2, 0.0, 0.35);

        let forest_rule = UsfSubstrate::add_zlm_rule(scale_index, "forest");
        UsfSubstrate::add_zlm_metric_band(scale_index, forest_rule, 1, 0.45, 1.0);
        UsfSubstrate::add_zlm_metric_band(scale_index, forest_rule, 0, 0.20, 0.85);

        let alpine_rule = UsfSubstrate::add_zlm_rule(scale_index, "alpine");
        UsfSubstrate::add_zlm_metric_band(scale_index, alpine_rule, 2, 0.75, 1.0);

        let arid_rule = UsfSubstrate::add_zlm_rule(scale_index, "arid");
        UsfSubstrate::add_zlm_metric_band(scale_index, arid_rule, 1, 0.0, 0.30);
        UsfSubstrate::add_zlm_metric_band(scale_index, arid_rule, 0, 0.45, 1.0);

        UsfSubstrate::set_scale_binding(
            scale_index,
            "dpt_sampler.debug.default.v1",
            "dpt_categorizer.debug.zlm_lookup.v1",
            "chunk_store.default",
        );
    }
}
