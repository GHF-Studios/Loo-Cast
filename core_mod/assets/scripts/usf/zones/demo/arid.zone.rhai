fn register_zone(ctx) {
    ctx.register();
    ctx.add_supported_phenomenon(
        "demo_dust_field",
        135,
        1.0,
        "single_per_zone",
    );
    ctx.set_selection_policy("weighted_top_priority");
    ctx.set_density_profile(0.92, -0.03, 0.0, 0.72);
}
