use super::*;

#[test]
fn source_extent_is_bounded_and_toggleable() {
    let mut source = SpatialDemandSource::cuboid(Vec3::new(-5.0, 12.0, 20.0)).with_priority(7);
    assert_eq!(source.half_extent_native(), Vec3::new(0.0, 12.0, 20.0));
    assert_eq!(source.priority(), 7);
    assert!(source.enabled());
    assert!(!source.toggle());
    assert!(!source.enabled());
}

#[test]
fn ancestor_extent_drops_by_one_decade_per_scale() {
    let half = Vec3::splat(96.0);
    assert_eq!(half * 10.0_f32.powi(-1), Vec3::splat(9.6));
    assert!((half.x * 10.0_f32.powi(-2) - 0.96).abs() < 1.0e-6);
}
