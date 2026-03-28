use core_mod_api::usf::substrate as UsfSubstrate;

fn register_scale() {
    let scale_level_count = UsfSubstrate::scale_level_count();
    for scale_index in 0..scale_level_count {
        UsfSubstrate::set_dpt_schema_from_metric_set(
            scale_index,
            1,
            "void",
            "metric_set.debug.default",
        );

        UsfSubstrate::set_scale_binding(
            scale_index,
            "dpt_sampler.debug.default.v1",
            "dpt_categorizer.debug.zlm_lookup.v1",
            "chunk_store.default",
        );
    }
}
