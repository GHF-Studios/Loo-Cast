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


#[test]
fn large_float_translation_keeps_local_offset_canonical() {
    // Large enough that an integer chunk carry is no longer exactly
    // representable as f32. The old normalize() converted the carry back to
    // f32 before subtracting it, which could manufacture an invalid remainder.
    let translations = [
        16_777_216_000.0_f32,
        67_108_864_000.0_f32,
        -16_777_216_000.0_f32,
        -67_108_864_000.0_f32,
    ];

    for x in translations {
        let position = UsfPosition::default()
            .translated_native(Vec3::new(x, 0.0, 0.0))
            .unwrap();

        assert!(
            position.offset().x >= USF_LOCAL_MIN,
            "x={x:e}, local={}",
            position.offset().x,
        );
        assert!(
            position.offset().x < USF_LOCAL_MAX_EXCLUSIVE,
            "x={x:e}, local={}",
            position.offset().x,
        );
    }
}

#[test]
fn normalization_keeps_positive_boundary_half_open() {
    let just_below = f32::from_bits(500.0_f32.to_bits() - 1);
    let position = UsfPosition::default()
        .translated_native(Vec3::new(1_000.0 + just_below, 0.0, 0.0))
        .unwrap();

    assert!(position.offset().x >= USF_LOCAL_MIN);
    assert!(position.offset().x < USF_LOCAL_MAX_EXCLUSIVE);
}


#[test]
fn coarse_runtime_translation_preserves_existing_fine_identity() {
    let s8 = SpatialScale::new(8).unwrap();

    let original = UsfPosition::default()
        .translated_whole_native([384_400_000, 18_000_000, 22_000_000])
        .unwrap()
        .translated_native(Vec3::new(12.25, -3.5, 8.125))
        .unwrap();

    let expected = original
        .translated_whole_native([100_000_000, 0, 0])
        .unwrap();

    let translated = original
        .translated_at_scale(s8, Vec3::new(1.0, 0.0, 0.0))
        .unwrap();

    assert_eq!(translated, expected);
}

#[test]
fn fractional_coarse_runtime_translation_preserves_existing_fine_identity() {
    let s8 = SpatialScale::new(8).unwrap();

    let original = UsfPosition::default()
        .translated_whole_native([384_400_000, 18_000_000, 22_000_000])
        .unwrap()
        .translated_native(Vec3::new(12.25, -3.5, 8.125))
        .unwrap();

    let expected = original
        .translated_whole_native([50_000_000, -25_000_000, 12_500_000])
        .unwrap();

    let translated = original
        .translated_at_scale(s8, Vec3::new(0.5, -0.25, 0.125))
        .unwrap();

    assert_eq!(translated, expected);
}

#[test]
fn zero_translation_at_any_coarser_scale_is_exact_identity() {
    let original = UsfPosition::default()
        .translated_whole_native([384_400_000, 18_000_000, 22_000_000])
        .unwrap()
        .translated_native(Vec3::new(12.25, -3.5, 8.125))
        .unwrap();

    for raw in SpatialScale::ZERO.exponent()..=SpatialScale::MAX.exponent() {
        let scale = SpatialScale::new(raw).unwrap();
        assert_eq!(
            original.translated_at_scale(scale, Vec3::ZERO).unwrap(),
            original
        );
    }
}


#[test]
fn finer_scale_zero_translation_refines_without_moving() {
    let s8 = SpatialScale::new(8).unwrap();
    let s7 = SpatialScale::new(7).unwrap();

    let coarse = UsfPosition::zero(s8)
        .translated_native(Vec3::new(3.844, 0.18, 0.22))
        .unwrap();

    let refined = coarse
        .translated_at_scale(s7, Vec3::ZERO)
        .unwrap();

    assert_eq!(refined.leaf_scale(), s7);

    let coarse_again = refined.reexpressed_at(s8).unwrap();
    let delta = coarse_again
        .relative_native_bounded(&coarse, 0.001)
        .unwrap();
    assert!(delta.length() < 1.0e-4);
}

#[test]
fn finer_scale_translation_refines_then_moves_without_coarsening() {
    let s8 = SpatialScale::new(8).unwrap();
    let s7 = SpatialScale::new(7).unwrap();

    let coarse = UsfPosition::zero(s8)
        .translated_native(Vec3::new(3.844, 0.18, 0.22))
        .unwrap();

    let moved = coarse
        .translated_at_scale(s7, Vec3::new(1.0, 0.0, 0.0))
        .unwrap();

    assert_eq!(moved.leaf_scale(), s7);

    let expected = coarse
        .reexpressed_at(s7)
        .unwrap()
        .translated_native(Vec3::new(1.0, 0.0, 0.0))
        .unwrap();

    assert_eq!(moved, expected);
}


#[test]
fn scale_relative_projection_accepts_different_leaf_precisions() {
    let s8 = SpatialScale::new(8).unwrap();
    let s5 = SpatialScale::new(5).unwrap();
    let s0 = SpatialScale::ZERO;

    let center = UsfPosition::zero(s8)
        .translated_native(Vec3::new(3.844, 0.18, 0.22))
        .unwrap();

    let observer = center
        .reexpressed_at(s0)
        .unwrap()
        .translated_whole_native([12_345, -678, 90])
        .unwrap();

    let coarse_center = center.reexpressed_at(s5).unwrap();
    let relative = observer
        .relative_at_scale_bounded(&coarse_center, s5, 16_384.0)
        .unwrap();

    assert!((relative.x - 0.12345).abs() < 1.0e-5);
    assert!((relative.y + 0.00678).abs() < 1.0e-5);
    assert!((relative.z - 0.00090).abs() < 1.0e-5);
}


#[test]
fn authored_s8_f64_refines_without_source_f32_quantization() {
    let s8 = SpatialScale::new(8).unwrap();
    let authored = DVec3::new(3.844, 0.11629, 0.22);

    let precise =
        UsfPosition::from_scale_native_f64(authored, s8, SpatialScale::ZERO).unwrap();

    let expected = UsfPosition::zero(SpatialScale::ZERO)
        .translated_whole_native([384_400_000, 11_629_000, 22_000_000])
        .unwrap();

    assert_eq!(precise, expected);
}
