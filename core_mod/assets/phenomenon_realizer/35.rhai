use ctx::math::scalar as scalar;
use ctx::math::vector as vector;

fn define_phenomenon_realizer() {
    #{
        id: "35",
        scale_id: "35",
        phenomenon_id: "test_phenomenon",
    }
}

fn realize_metric_excitation(excitation) {
    let axis = vector::vec3(0.0, 1.0, 0.0);
    let projected = vector::dot(excitation, axis);
    let intensity = scalar::abs(projected);
    scalar::mul(intensity, 1000.0)
}
