use super::types::{UsfFloat, UsfFloatDomain, UsfFloatPolicy};

#[test]
fn multiplicative_usf_float_folds_large_values_down_with_cycles() {
    let mut value = UsfFloat::new(125.0);
    let policy = UsfFloatPolicy {
        local_min: 0.1,
        local_max: 10.0,
        commit_buffer_ratio: 0.1,
        domain: UsfFloatDomain::Multiplicative { pivot_factor: 10.0 },
    };

    let pivot = value.fold_with_policy(policy);

    assert_eq!(pivot.upper_crossings, 2);
    assert_eq!(pivot.lower_crossings, 0);
    assert!((value.local - 1.25).abs() < 1e-9);
    assert_eq!(value.canonical_cycles, 2);
}

#[test]
fn multiplicative_usf_float_folds_small_values_up_with_cycles() {
    let mut value = UsfFloat::new(0.00125);
    let policy = UsfFloatPolicy {
        local_min: 0.1,
        local_max: 10.0,
        commit_buffer_ratio: 0.1,
        domain: UsfFloatDomain::Multiplicative { pivot_factor: 10.0 },
    };

    let pivot = value.fold_with_policy(policy);

    assert_eq!(pivot.upper_crossings, 0);
    assert_eq!(pivot.lower_crossings, 2);
    assert!((value.local - 0.125).abs() < 1e-9);
    assert_eq!(value.canonical_cycles, -2);
}

#[test]
fn linear_usf_float_wraps_rotation_and_tracks_cycles() {
    let mut value = UsfFloat::new(4.0);
    let policy = UsfFloatPolicy {
        local_min: -std::f64::consts::PI,
        local_max: std::f64::consts::PI,
        commit_buffer_ratio: 0.1,
        domain: UsfFloatDomain::Linear {
            wrap_size: std::f64::consts::TAU,
        },
    };

    let pivot = value.fold_with_policy(policy);

    assert_eq!(pivot.upper_crossings, 1);
    assert_eq!(pivot.lower_crossings, 0);
    assert!(value.local < std::f64::consts::PI);
    assert_eq!(value.canonical_cycles, 1);
}

