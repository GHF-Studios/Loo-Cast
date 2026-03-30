use core_mod_api::usf::substrate as UsfSubstrate;

fn register_metric_set() {
    UsfSubstrate::set_metric_set("metric_set.debug.default");
    UsfSubstrate::add_metric_set_metric("metric_set.debug.default", "temperature");
    UsfSubstrate::add_metric_set_metric("metric_set.debug.default", "humidity");
    UsfSubstrate::add_metric_set_metric("metric_set.debug.default", "elevation");
    UsfSubstrate::add_metric_set_metric("metric_set.debug.default", "vegetation_density");
}
