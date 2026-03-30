use core_mod_api::usf::substrate as UsfSubstrate;

fn register_metric_set() {
    UsfSubstrate::set_metric_set("metric_set.placeholder.solid_fill.v1");
    UsfSubstrate::add_metric_set_metric("metric_set.placeholder.solid_fill.v1", "solid_fill");
}
