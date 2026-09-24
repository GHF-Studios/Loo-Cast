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
fn refinement_is_independent_from_generic_interest_extent() {
    let source = SpatialDemandSource::cuboid(Vec3::splat(96.0));
    let mut refinement = SpatialRefinementDemand::cuboid(Vec3::splat(32.0));
    let s4 = SpatialScale::new(4).unwrap();

    assert_eq!(source.half_extent_native(), Vec3::splat(96.0));
    assert_eq!(refinement.half_extent_native(), Vec3::splat(32.0));
    assert_eq!(refinement.minimum_scale(), None);

    refinement.request_through(s4);
    assert_eq!(refinement.minimum_scale(), Some(s4));
    assert_eq!(source.half_extent_native(), Vec3::splat(96.0));

    refinement.clear();
    assert_eq!(refinement.minimum_scale(), None);
}
