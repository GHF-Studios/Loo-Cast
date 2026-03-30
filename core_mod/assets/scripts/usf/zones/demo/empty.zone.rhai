fn register_zone(ctx) {
    ctx.register();
    ctx.set_selection_policy("weighted_top_priority");
    ctx.set_density_profile(0.0, 0.0, 0.0, 0.0);
}
