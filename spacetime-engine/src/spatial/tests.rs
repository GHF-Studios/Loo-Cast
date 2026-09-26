use super::rebase::rebase_shift;
use super::*;

#[test]
fn rebase_keeps_small_coordinates_untouched() {
    assert_eq!(rebase_shift(Vec3::new(99.0, -12.0, 0.0)), Vec3::ZERO);
}

#[test]
fn rebase_uses_decimal_usf_quantization() {
    assert_eq!(
        rebase_shift(Vec3::new(350.0, -725.0, 3.0)),
        Vec3::new(300.0, -700.0, 0.0)
    );
}

#[test]
fn chart_delta_preserves_passive_scale_semantics() {
    let s0 = SpatialScale::ZERO;
    let s1 = SpatialScale::new(1).unwrap();
    let origin = UsfPosition::zero(SpatialScale::MIN);
    let passive_local = Vec3::new(10.0, -3.0, 0.5);
    let before = origin
        .translated_at_scale(s1, passive_local)
        .expect("passive position is canonical");

    let delta = UsfChartDelta::new(s0, Vec3::new(100.0, 0.0, 0.0));
    assert_eq!(delta.at_scale(s1).unwrap(), Vec3::new(10.0, 0.0, 0.0));

    let rebased_origin = origin
        .translated_at_scale(s0, delta.local_shift())
        .expect("chart origin can rebase");
    let rebased_local = passive_local - delta.at_scale(s1).unwrap();
    let after = rebased_origin
        .translated_at_scale(s1, rebased_local)
        .expect("passive position remains canonical");

    assert_eq!(before, after);
}

#[test]
fn repeated_rebases_preserve_semantic_position_over_large_fixed_scale_travel() {
    let mut frame_origin = UsfPosition::default();
    let mut local = Vec3::new(17.0, -19.0, 23.0);
    let step = Vec3::new(700.0, -515.0, 333.0);
    let mut expected = frame_origin.translated_native(local).unwrap();

    for _ in 0..20_000 {
        local += step;
        expected = expected.translated_native(step).unwrap();

        let before_rebase = frame_origin.translated_native(local).unwrap();
        assert_eq!(before_rebase, expected);

        let shift = rebase_shift(local);
        assert_ne!(shift, Vec3::ZERO);
        frame_origin = frame_origin.translated_native(shift).unwrap();
        local -= shift;

        let after_rebase = frame_origin.translated_native(local).unwrap();
        assert_eq!(after_rebase, expected);
        assert!(local.abs().max_element() < REBASE_THRESHOLD_NATIVE);
    }

    let displacement = expected
        .relative_native_bounded(&UsfPosition::default(), 20_000_000.0)
        .unwrap();
    assert_eq!(
        displacement,
        Vec3::new(14_000_017.0, -10_300_019.0, 6_660_023.0)
    );
}
