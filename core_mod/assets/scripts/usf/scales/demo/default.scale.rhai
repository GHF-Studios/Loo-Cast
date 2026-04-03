fn register_scale(ctx) {
    let scale_level_count = ctx.scale_level_count();
    let dpt_sampler_kernel_id = ctx.default_dpt_sampler_kernel_id();
    let dpt_categorizer_kernel_id = ctx.default_dpt_categorizer_kernel_id();
    ctx.register_dpt_sampler_kernel_id(dpt_sampler_kernel_id);
    ctx.register_dpt_categorizer_kernel_id(dpt_categorizer_kernel_id);
    for scale_index in 0..scale_level_count {
        ctx.set_dpt_schema_from_metric_set(
            scale_index,
            1,
            "empty",
            "demo_mass_density",
        );

        ctx.set_scale(
            scale_index,
            dpt_sampler_kernel_id,
            dpt_categorizer_kernel_id,
            "chunk_store.demo",
        );
    }
}
