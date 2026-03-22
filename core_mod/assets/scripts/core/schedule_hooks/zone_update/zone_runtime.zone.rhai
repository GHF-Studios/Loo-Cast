use core_mod_api::usf::zone as UsfZone;

private fn zone_update_tick(world, params) {
    // Coarser-than-active scales can use explicit time-scaling logic in scripts.
    let active_scale_index = if params.has_active_scale { params.active_scale_index } else { 35 };
    let one_above_active = active_scale_index - 1;
    let _time_scale = UsfZone::time_scale_for_scale_indices(active_scale_index, one_above_active);

    // Zone scripts can issue direct ECS operations through the existing World bindings.
    // This file owns zone-domain runtime behavior (`*.zone.rhai`).
    // params exposes zone-domain typed context, e.g.:
    // params.loaded_zone_count / params.realized_zone_count.
}
