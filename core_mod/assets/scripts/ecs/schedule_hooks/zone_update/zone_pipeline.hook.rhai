private fn run_zone_update_pipeline(world, params) {
    let active_scale_index = if params.has_active_scale { params.active_scale_index } else { 35 };
    let one_above_active = active_scale_index - 1;
    let _time_factor = zone_time_factor(active_scale_index, one_above_active);
    zone_update_tick(world, params);
}
