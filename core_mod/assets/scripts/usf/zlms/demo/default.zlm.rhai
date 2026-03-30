fn register_zlm(ctx) {
    let scale_level_count = ctx.scale_level_count();
    for scale_index in 0..scale_level_count {
        ctx.set_scale(scale_index, 1, "empty");

        if scale_index == 0 {
            let spawn_buffer_rule = ctx.add_rule(scale_index, "spawn_buffer");
            ctx.add_metric_band_by_name(scale_index, spawn_buffer_rule, "root_pos_x", 0.45, 0.55);
            ctx.add_metric_band_by_name(scale_index, spawn_buffer_rule, "root_pos_y", 0.45, 0.55);
            ctx.add_metric_band_by_name(scale_index, spawn_buffer_rule, "root_pos_z", 0.45, 0.55);
        }

        let solid_rule = ctx.add_rule(scale_index, "solid");
        ctx.add_metric_band_by_name(scale_index, solid_rule, "demo_mass_density", 0.52, 1.0);
    }
}
