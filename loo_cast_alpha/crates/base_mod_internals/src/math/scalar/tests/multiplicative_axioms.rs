use crate::math::scalar::{
    aliases::UsfOrNormalScalar,
    shared::ScalarCoreOps,
    usf::UsfScalar,
};

use super::utils::random_value_batch;

#[test]
fn zero_scalar_property() {
    let values = random_value_batch(1);
    let zero = UsfScalar::ZERO;

    for value in values.iter() {
        let result = zero.mul(UsfOrNormalScalar::A(value.clone()));

        assert_eq!(result, zero, "zero scalar property failed for {value}");
    }
}

#[test]
fn multiplicative_identity() {
    let values = random_value_batch(1);
    let one = UsfScalar::ONE;

    for value in values.iter() {
        let result = one.mul(UsfOrNormalScalar::A(value.clone()));

        assert_eq!(result, one, "multiplicative identity failed for {value}");
    }
}

#[test]
fn associativity_of_multiplication() {
    let values = random_value_batch(3);
    for a in values.iter() {
        for b in values.iter() {
            for c in values.iter() {
                let ab = a.clone().mul(UsfOrNormalScalar::A(b.clone()));
                let ab_times_c = ab.mul(UsfOrNormalScalar::A(c.clone()));

                let bc = b.clone().mul(UsfOrNormalScalar::A(c.clone()));
                let a_times_bc = a.clone().mul(UsfOrNormalScalar::A(bc.clone()));

                assert_eq!(ab_times_c, a_times_bc, "associativity of multiplication failed for (a: {a}, b: {b}, c: {c})");
            }
        }
    }
}