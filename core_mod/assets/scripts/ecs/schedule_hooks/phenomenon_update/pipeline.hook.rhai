private fn run_phenomenon_update_pipeline(world, params) {
    let registry = phenomenon_catalog();
    let active_bucket = if params.has_active_scale { params.active_scale_index } else { 35 };
    let active_registry = phn_filter_by_scale_bucket(registry, active_bucket);

    // Future engine API (mocked on purpose):
    // Phenomena::set_catalog(registry);
    // Phenomena::set_active_slice(active_registry);

    let _catalog_count = len(registry);
    let _active_count = len(active_registry);
}
