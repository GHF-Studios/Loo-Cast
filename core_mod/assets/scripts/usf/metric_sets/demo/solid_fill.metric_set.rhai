use core_mod_api::usf::substrate as UsfSubstrate;

fn register_metric_set() {
    UsfSubstrate::set_metric_set("metric_set.demo.solid_fill");
    UsfSubstrate::add_metric_set_metric("metric_set.demo.solid_fill", "solid_fill");
    UsfSubstrate::add_metric_set_metric("metric_set.demo.solid_fill", "root_pos_x");
    UsfSubstrate::add_metric_set_metric("metric_set.demo.solid_fill", "root_pos_y");
    UsfSubstrate::add_metric_set_metric("metric_set.demo.solid_fill", "root_pos_z");
}
