use core_mod_api::usf::substrate as UsfSubstrate;

fn register_metric() {
    let scale_min = 0;
    let scale_max = UsfSubstrate::scale_level_count() - 1;
    UsfSubstrate::add_metric_typed(
        0,
        "temperature",
        "f32",
        "climate.temperature.normalized",
        "brick",
        false,
        scale_min,
        scale_max,
    );
    UsfSubstrate::add_metric_typed(
        1,
        "humidity",
        "f32",
        "climate.humidity.normalized",
        "brick",
        false,
        scale_min,
        scale_max,
    );
    UsfSubstrate::add_metric_typed(
        2,
        "elevation",
        "f32",
        "terrain.elevation.normalized",
        "brick",
        false,
        scale_min,
        scale_max,
    );
    UsfSubstrate::add_metric_typed(
        3,
        "vegetation_density",
        "f32",
        "biosphere.vegetation_density.normalized",
        "brick",
        true,
        scale_min,
        scale_max,
    );
}
