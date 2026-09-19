use super::*;

#[test]
fn spatial_scale_stack_has_seventy_one_levels() {
    assert_eq!(SPATIAL_SCALE_COUNT, 71);
    assert_eq!(SpatialScale::MAX.exponent(), 35);
    assert_eq!(SpatialScale::MIN.exponent(), -35);
}

#[test]
fn unit_offset_carries_into_scale_zero_chunk_digit() {
    let position = UsfPosition::from_scale0_local(Vec3::new(600.0, 0.0, 0.0)).unwrap();
    assert_eq!(position.offset().x, -400.0);
    assert_eq!(position.digit(SpatialScale::ZERO).x, 1);
}

#[test]
fn ten_scale_zero_chunks_carry_once_into_scale_one() {
    let position = UsfPosition::from_scale0_local(Vec3::new(10_000.0, 0.0, 0.0)).unwrap();
    assert_eq!(position.offset().x, 0.0);
    assert_eq!(position.digit(SpatialScale::ZERO).x, 0);
    assert_eq!(position.digit(SpatialScale::new(1).unwrap()).x, 1);
}

#[test]
fn whole_native_translation_matches_bounded_float_translation() {
    let exact = UsfPosition::default()
        .translated_whole_native([1_280, -600, 42])
        .unwrap();
    let bounded = UsfPosition::default()
        .translated_native(Vec3::new(1_280.0, -600.0, 42.0))
        .unwrap();

    assert_eq!(exact, bounded);
}

#[test]
fn scale_relative_projection_does_not_require_a_giant_leaf_coordinate() {
    let origin = UsfPosition::default();
    let point = origin
        .translated_whole_native([12_345, -6_780, 50])
        .unwrap();
    let s1 = SpatialScale::new(1).unwrap();

    let delta = point
        .relative_at_scale_bounded(&origin, s1, 2_000.0)
        .unwrap();
    assert!((delta.x - 1_234.5).abs() < 1.0e-4);
    assert!((delta.y + 678.0).abs() < 1.0e-4);
    assert!((delta.z - 5.0).abs() < 1.0e-4);
}

#[test]
fn bounded_relative_position_crosses_balanced_digit_carry_exactly() {
    let origin = UsfPosition::from_scale0_local(Vec3::new(499.0, 0.0, 0.0)).unwrap();
    let point = origin.translated_native(Vec3::new(4.0, -3.0, 2.0)).unwrap();

    assert_eq!(
        point.relative_native_bounded(&origin, 8.0).unwrap(),
        Vec3::new(4.0, -3.0, 2.0)
    );
}

#[test]
fn bounded_relative_position_rejects_far_semantic_points() {
    let origin = UsfPosition::default();
    let far = origin.translated_whole_native([20_000, 0, 0]).unwrap();

    assert_eq!(
        far.relative_native_bounded(&origin, 512.0),
        Err(UsfPositionError::RelativePositionOutsideBound)
    );
}

#[test]
fn bounded_relative_position_survives_a_long_decimal_carry_chain() {
    let mut origin = UsfPosition::default();
    let mut point = UsfPosition::default();

    // 0444...444 + one scale-0 USF chunk = 1(-5)(-5)...(-5)
    // in balanced decimal representation. Difference normalization must
    // happen before bounded float projection; a raw prefix accumulator would
    // grow enormous before the low digit differences cancel back to +1.
    for raw_scale in SpatialScale::ZERO.exponent()..SPATIAL_SCALE_MAX {
        let scale = SpatialScale::new(raw_scale).unwrap();
        origin.digits[scale.index_from_top()] = IVec3::new(4, 0, 0);
        point.digits[scale.index_from_top()] = IVec3::new(-5, 0, 0);
    }
    point.digits[SpatialScale::MAX.index_from_top()] = IVec3::new(1, 0, 0);

    assert_eq!(
        point.relative_native_bounded(&origin, 2_000.0).unwrap(),
        Vec3::new(1_000.0, 0.0, 0.0)
    );
}

#[test]
fn root_carry_wraps_to_the_opposite_world_edge() {
    let mut before = UsfPosition::default();
    before.offset.x = 499.0;
    for raw_scale in SpatialScale::ZERO.exponent()..=SPATIAL_SCALE_MAX {
        let scale = SpatialScale::new(raw_scale).unwrap();
        before.digits[scale.index_from_top()].x = 4;
    }

    let after = before.translated_native(Vec3::new(2.0, 0.0, 0.0)).unwrap();
    assert_eq!(after.offset.x, -499.0);
    for raw_scale in SpatialScale::ZERO.exponent()..=SPATIAL_SCALE_MAX {
        let scale = SpatialScale::new(raw_scale).unwrap();
        assert_eq!(after.digit(scale).x, -5);
    }
    assert_eq!(
        after.relative_native_bounded(&before, 4.0).unwrap(),
        Vec3::new(2.0, 0.0, 0.0)
    );
}

#[test]
fn root_borrow_wraps_to_the_opposite_world_edge() {
    let mut before = UsfPosition::default();
    before.offset.x = -499.0;
    for raw_scale in SpatialScale::ZERO.exponent()..=SPATIAL_SCALE_MAX {
        let scale = SpatialScale::new(raw_scale).unwrap();
        before.digits[scale.index_from_top()].x = -5;
    }

    let after = before.translated_native(Vec3::new(-2.0, 0.0, 0.0)).unwrap();
    assert_eq!(after.offset.x, 499.0);
    for raw_scale in SpatialScale::ZERO.exponent()..=SPATIAL_SCALE_MAX {
        let scale = SpatialScale::new(raw_scale).unwrap();
        assert_eq!(after.digit(scale).x, 4);
    }
    assert_eq!(
        after.relative_native_bounded(&before, 4.0).unwrap(),
        Vec3::new(-2.0, 0.0, 0.0)
    );
}

#[test]
fn negative_translation_uses_balanced_decimal_carry() {
    let position = UsfPosition::from_scale0_local(Vec3::new(-600.0, 0.0, 0.0)).unwrap();
    assert_eq!(position.offset().x, 400.0);
    assert_eq!(position.digit(SpatialScale::ZERO).x, -1);
}
}

#[test]
fn reexpressing_position_across_scales_preserves_location() {
    let s8 = SpatialScale::new(8).unwrap();
    let original = UsfPosition::from_scale0_local(Vec3::new(12.25, -3.5, 8.0)).unwrap();

    let coarse = original.reexpressed_at(s8).unwrap();
    let round_trip = coarse.reexpressed_at(SpatialScale::ZERO).unwrap();

    let delta = round_trip
        .relative_native_bounded(&original, 0.001)
        .unwrap();
    assert!(delta.length() < 1.0e-4);
}

#[test]
fn scale_coordinate_projection_uses_requested_native_units() {
    let s8 = SpatialScale::new(8).unwrap();
    let position = UsfPosition::zero(s8)
        .translated_native(Vec3::new(3.844, 0.18, 0.22))
        .unwrap();

    let coordinate = position.coordinate_at_scale_f64(s8).unwrap();
    assert!((coordinate.x - 3.844).abs() < 1.0e-5);
    assert!((coordinate.y - 0.18).abs() < 1.0e-5);
    assert!((coordinate.z - 0.22).abs() < 1.0e-5);
}
