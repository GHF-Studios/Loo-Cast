fn register_metric(ctx) {
    let scale_min = 0;
    let scale_max = ctx.scale_level_count() - 1;
    ctx.register(
        7,
        "f32",
        "biosphere.vegetation_density.normalized",
        "brick",
        true,
        scale_min,
        scale_max,
    );
}
