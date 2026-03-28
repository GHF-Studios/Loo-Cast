use core_mod_api::usf::substrate as UsfSubstrate;

fn register_metric() {
    UsfSubstrate::add_metric(0, "temperature", true);
    UsfSubstrate::add_metric(1, "humidity", true);
    UsfSubstrate::add_metric(2, "elevation", true);
    UsfSubstrate::add_metric(3, "vegetation_density", false);
}
