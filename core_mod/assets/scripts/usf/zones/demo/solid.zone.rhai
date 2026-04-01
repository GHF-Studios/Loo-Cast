fn register_zone(ctx) {
    ctx.register();
    ctx.add_supported_phenomenon(
        "demo_manifestation_density",
        100,
        1.0,
        "single_per_zone",
        1,
    );
    ctx.set_selection_policy("weighted_top_priority");
    ctx.set_density_profile(1.0, 0.0, 0.0, 1.0);
}
