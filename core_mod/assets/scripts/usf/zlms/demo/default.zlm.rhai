fn register_zlm(ctx) {
    let scale_level_count = ctx.scale_level_count();
    let human_scale_index = scale_level_count / 2;
    let human_band_min = if human_scale_index > 0 {
        human_scale_index - 1
    } else {
        0
    };
    let human_band_max = if human_scale_index + 1 < scale_level_count {
        human_scale_index + 1
    } else {
        scale_level_count - 1
    };

    for scale_index in 0..scale_level_count {
        ctx.set_scale(scale_index, 2, "empty");

        if scale_index >= human_band_min && scale_index <= human_band_max {
            if scale_index == human_scale_index {
                let spawn_buffer_rule = ctx.add_rule(scale_index, "spawn_buffer");
                ctx.add_metric_band_by_name(scale_index, spawn_buffer_rule, "root_pos_x", 0.46, 0.54);
                ctx.add_metric_band_by_name(scale_index, spawn_buffer_rule, "root_pos_y", 0.46, 0.54);
                ctx.add_metric_band_by_name(scale_index, spawn_buffer_rule, "root_pos_z", 0.46, 0.54);
            }

            let lush_rule = ctx.add_rule(scale_index, "lush");
            ctx.add_metric_band_by_name(scale_index, lush_rule, "vegetation_density", 0.64, 1.0);
            ctx.add_metric_band_by_name(scale_index, lush_rule, "humidity", 0.56, 1.0);
            ctx.add_metric_band_by_name(scale_index, lush_rule, "elevation", 0.18, 0.72);

            let arid_rule = ctx.add_rule(scale_index, "arid");
            ctx.add_metric_band_by_name(scale_index, arid_rule, "humidity", 0.0, 0.38);
            ctx.add_metric_band_by_name(scale_index, arid_rule, "temperature", 0.58, 1.0);

            let ridge_rule = ctx.add_rule(scale_index, "ridge");
            ctx.add_metric_band_by_name(scale_index, ridge_rule, "elevation", 0.74, 1.0);
            ctx.add_metric_band_by_name(scale_index, ridge_rule, "matter_support", 0.45, 1.0);

            let solid_rule = ctx.add_rule(scale_index, "solid");
            ctx.add_metric_band_by_name(scale_index, solid_rule, "demo_mass_density", 0.50, 1.0);
            continue;
        }

        let solid_rule = ctx.add_rule(scale_index, "solid");
        ctx.add_metric_band_by_name(scale_index, solid_rule, "demo_mass_density", 0.63, 1.0);
    }
}
