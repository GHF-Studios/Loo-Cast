use ctx::math::scalar as scalar;
use ctx::math::vector as vector;

fn define_phenomenon() {
    #{
        id: "test_phenomenon",
        metric_id: "test_metric",
    }
}

fn emit_excitation() {
    let raw = vector::vec3(3.0, 4.0, 12.0);
    let inv_len = scalar::div(1.0, vector::length(raw));
    vector::scale(raw, inv_len)
}
