use ctx::math::scalar as scalar;

fn define_scale() {
    let exponent = 35.0;
    let factor = scalar::pow(10.0, exponent);
    #{
        id: "35",
        scale_factor_exponent: 35,
        scale_index_from_top: 0,
        scale_factor: factor,
    }
}
