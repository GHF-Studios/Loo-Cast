fn register_metric(ctx) {
    let scale_min = 0;
    let scale_max = ctx.scale_level_count() - 1;
    ctx.register(
        4,
        "f32",
        "climate.temperature.normalized",
        "brick",
        false,
        scale_min,
        scale_max,
    );
}
