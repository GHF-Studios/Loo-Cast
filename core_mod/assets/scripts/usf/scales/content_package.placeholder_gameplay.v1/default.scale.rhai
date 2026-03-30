use core_mod_api::usf::substrate as UsfSubstrate;

fn register_scale() {
    let scale_level_count = UsfSubstrate::scale_level_count();
    let dpt_sampler_kernel_id = UsfSubstrate::default_dpt_sampler_kernel_id();
    let dpt_categorizer_kernel_id = UsfSubstrate::default_dpt_categorizer_kernel_id();
    for scale_index in 0..scale_level_count {
        UsfSubstrate::set_dpt_schema_from_metric_set(
            scale_index,
            1,
            "void",
            "metric_set.debug.default",
        );

        UsfSubstrate::set_scale_binding_with_usf_content_profile(
            scale_index,
            dpt_sampler_kernel_id,
            dpt_categorizer_kernel_id,
            "chunk_store.default",
            "content_profile.placeholder_gameplay.v1",
        );
    }
}
