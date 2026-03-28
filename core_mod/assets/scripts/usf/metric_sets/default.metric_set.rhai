use core_mod_api::usf::substrate as UsfSubstrate;

fn register_metric_set() {
    UsfSubstrate::set_metric_set("metric_set.debug.default");
    UsfSubstrate::build_metric_set_from_all_metrics("metric_set.debug.default");
}
