use core_mod_api::usf::zone as UsfZone;

private fn zone_time_factor(active_scale_index, target_scale_index) {
    UsfZone::time_scale_for_scale_indices(active_scale_index, target_scale_index)
}
