use ctx::math::scalar as scalar;
use ctx::math::vector as vector;

fn define_metric() {
    let basis = vector::vec3(2.0, 1.0, 0.5);
    let baseline = scalar::mul(vector::length(basis), 4.0);

    #{
        id: "test_metric",
        scale_id: "35",
        baseline: baseline,
    }
}
