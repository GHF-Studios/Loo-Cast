use std::f64::consts::TAU;

use super::types::{UsfFloat, UsfFloatDomain, UsfFloatPolicy, UsfRotation, UsfTransform};

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

#[test]
fn usf_rotation_fold_tracks_multi_turn_cycles_in_both_directions() {
    let mut rotation = UsfRotation::default();
    rotation.add_local_delta(crate::bevy::prelude::Vec3::new(0.0, 0.0, (TAU * 3.0 + 1.0) as f32));

    let first = rotation.fold();
    assert_eq!(first.z, 3);
    assert!(rotation.z.local < std::f64::consts::PI);
    assert!(rotation.z.local > -std::f64::consts::PI);
    assert_eq!(rotation.z.canonical_cycles, 3);

    rotation.add_local_delta(crate::bevy::prelude::Vec3::new(0.0, 0.0, (-(TAU * 5.0 + 0.5)) as f32));

    let second = rotation.fold();
    assert_eq!(second.z, -5);
    assert!(rotation.z.local < std::f64::consts::PI);
    assert!(rotation.z.local > -std::f64::consts::PI);
    assert_eq!(rotation.z.canonical_cycles, -2);
}

#[test]
fn usf_transform_scale_and_rotation_folds_are_independent() {
    let mut transform = UsfTransform::default();
    transform.scale.uniform.local = 125.0;
    transform.rotation.z.local = 4.0;

    let scale_pivot = transform.scale.fold();
    let rotation_cycle_delta = transform.rotation.fold();

    assert_eq!(scale_pivot.upper_crossings, 2);
    assert_eq!(scale_pivot.lower_crossings, 0);
    assert_eq!(transform.scale.uniform.canonical_cycles, 2);

    assert_eq!(rotation_cycle_delta.z, 1);
    assert_eq!(transform.rotation.z.canonical_cycles, 1);
}

#[test]
fn usf_transform_translation_and_rotation_folds_are_independent() {
    let mut transform = UsfTransform::default();
    transform.translation.x.local = 1600.0;
    transform.rotation.x.local = -4.0;

    let translation_cycle_delta = transform.translation.fold();
    let rotation_cycle_delta = transform.rotation.fold();

    assert_eq!(translation_cycle_delta.x, 2);
    assert_eq!(transform.translation.x.canonical_cycles, 2);
    assert_eq!(rotation_cycle_delta.x, -1);
    assert_eq!(transform.rotation.x.canonical_cycles, -1);
}
