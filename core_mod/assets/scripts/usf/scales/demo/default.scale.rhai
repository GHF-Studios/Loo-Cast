fn register_scale(ctx) {
    let scale_level_count = ctx.scale_level_count();
    let metric_sampler_kernel_id = ctx.default_metric_sampler_kernel_id();
    let metric_categorizer_kernel_id = ctx.default_metric_categorizer_kernel_id();
    ctx.register_metric_sampler_kernel_id(metric_sampler_kernel_id);
    ctx.register_metric_categorizer_kernel_id(metric_categorizer_kernel_id);
    for scale_index in 0..scale_level_count {
        ctx.set_metric_container_layout_from_metric_set(
            scale_index,
            1,
            "empty",
            "demo_mass_density",
        );

        ctx.set_scale(
            scale_index,
            metric_sampler_kernel_id,
            metric_categorizer_kernel_id,
            "chunk_store.demo",
        );
    }
}
