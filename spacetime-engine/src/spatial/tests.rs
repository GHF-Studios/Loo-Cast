use super::*;
use super::rebase::rebase_shift;

#[test]
fn rebase_keeps_small_coordinates_untouched() {
    assert_eq!(rebase_shift(Vec3::new(255.0, -12.0, 0.0)), Vec3::ZERO);
}

#[test]
fn rebase_uses_quantized_local_translation() {
    assert_eq!(
        rebase_shift(Vec3::new(300.0, -700.0, 3.0)),
        Vec3::new(256.0, -512.0, 0.0)
    );
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
        assert!(local.abs().max_element() < REBASE_THRESHOLD_METERS);
    }

    // The semantic path has travelled millions of metres while the runtime
    // chart stayed bounded to ordinary float coordinates after every step.
    let displacement = expected
        .relative_native_bounded(&UsfPosition::default(), 20_000_000.0)
        .unwrap();
    assert_eq!(
        displacement,
        Vec3::new(14_000_017.0, -10_300_019.0, 6_660_023.0)
    );
}
