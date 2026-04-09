fn register_zone(ctx) {
    ctx.register();
    ctx.add_supported_phenomenon(
        "demo_flora_cluster",
        140,
        1.0,
        "single_per_zone",
    );
    ctx.set_selection_policy("weighted_top_priority");
    ctx.set_density_profile(1.08, 0.06, 0.18, 1.0);
}
