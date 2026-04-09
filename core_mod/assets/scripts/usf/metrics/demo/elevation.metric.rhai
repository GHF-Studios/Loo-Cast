fn register_metric(ctx) {
    let scale_min = 0;
    let scale_max = ctx.scale_level_count() - 1;
    ctx.register(
        6,
        "f32",
        "terrain.elevation.normalized",
        "brick",
        false,
        scale_min,
        scale_max,
    );
}
