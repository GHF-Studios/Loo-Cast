fn register_zone(ctx) {
    ctx.register();
    ctx.add_supported_phenomenon(
        "surface",
        100,
        1.0,
        "single_primary",
        1,
    );
    ctx.set_selection_policy("weighted_top_priority");
    ctx.set_density_profile(1.0, 0.0, 0.0, 1.0);
}
