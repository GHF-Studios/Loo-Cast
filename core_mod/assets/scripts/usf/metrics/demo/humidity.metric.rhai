fn register_metric(ctx) {
    let scale_min = 0;
    let scale_max = ctx.scale_level_count() - 1;
    ctx.register(
        5,
        "f32",
        "climate.humidity.normalized",
        "brick",
        false,
        scale_min,
        scale_max,
    );
}
