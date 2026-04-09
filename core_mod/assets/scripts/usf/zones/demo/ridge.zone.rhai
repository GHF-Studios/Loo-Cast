fn register_zone(ctx) {
    ctx.register();
    ctx.add_supported_phenomenon(
        "demo_rock_spire",
        145,
        1.0,
        "single_per_zone",
    );
    ctx.set_selection_policy("weighted_top_priority");
    ctx.set_density_profile(1.12, 0.02, 0.22, 1.0);
}
