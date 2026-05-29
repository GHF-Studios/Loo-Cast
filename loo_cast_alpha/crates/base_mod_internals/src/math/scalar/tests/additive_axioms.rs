use crate::math::scalar::{
    aliases::UsfOrNormalScalar,
    shared::ScalarCoreOps,
    usf::UsfScalar,
};

use super::utils::random_value_batch;

#[test]
fn commutativity_of_addition() {
    let values = random_value_batch(2);
    for a in values.iter() {
        for b in values.iter() {
            let lhs = a.clone().add(UsfOrNormalScalar::A(b.clone()));
            let rhs = b.clone().add(UsfOrNormalScalar::A(a.clone()));
            assert_eq!(lhs, rhs, "add commutativity failed for (a: {a}, b: {b})");
        }
    }
}

#[test]
fn associativity_of_addition() {
    let values = random_value_batch(3);
    for a in values.iter() {
        for b in values.iter() {
            for c in values.iter() {
                let ab = a.clone().add(UsfOrNormalScalar::A(b.clone()));
                let lhs = ab.add(UsfOrNormalScalar::A(c.clone()));

                let bc = b.clone().add(UsfOrNormalScalar::A(c.clone()));
                let rhs = a.clone().add(UsfOrNormalScalar::A(bc));

                assert_eq!(lhs, rhs, "add associativity failed for (a: {a}, b: {b}, c: {c})");
            }
        }
    }
}

#[test]
fn additive_identity() {
    let values = random_value_batch(1);
    let zero = UsfScalar::ZERO;

    for value in values.iter() {
        let left = zero.clone().add(UsfOrNormalScalar::A(value.clone()));
        let right = value.clone().add(UsfOrNormalScalar::A(zero.clone()));

        assert_eq!(left, *value, "left identity failed for {value}");
        assert_eq!(right, *value, "right identity failed for {value}");
    }
}

#[test]
fn additive_inverse() {
    let values = random_value_batch(1);
    let zero = UsfScalar::ZERO;

    for value in values.iter() {
        // Safe removal of unnecessary clones on final vector consume steps
        let inverse = zero.sub(UsfOrNormalScalar::A(value.clone()));
        let left = value.add(UsfOrNormalScalar::A(inverse.clone()));
        let right = inverse.add(UsfOrNormalScalar::A(value.clone()));

        assert_eq!(left, zero, "lhs inverse failed for {value}");
        assert_eq!(right, zero, "rhs inverse failed for {value}");
    }
}