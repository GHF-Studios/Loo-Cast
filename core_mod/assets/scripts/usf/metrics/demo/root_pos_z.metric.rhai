fn register_metric(ctx) {
    let scale_min = 0;
    let scale_max = ctx.scale_level_count() - 1;
    ctx.register(
        3,
        "f32",
        "position.root.z.normalized",
        "brick",
        true,
        scale_min,
        scale_max,
    );
}
