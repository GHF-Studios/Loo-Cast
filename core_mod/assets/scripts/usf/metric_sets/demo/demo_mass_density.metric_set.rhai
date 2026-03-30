fn register_metric_set(ctx) {
    ctx.set_id("demo_mass_density");
    ctx.register();
    ctx.add_metric("demo_mass_density");
    ctx.add_metric("root_pos_x");
    ctx.add_metric("root_pos_y");
    ctx.add_metric("root_pos_z");
}
