use super::*;

#[test]
fn midpoint_between_s0_and_s1_has_reciprocal_adjacent_projection_factors() {
    let mut view = UsfViewContext::default();
    view.set_continuous_exponent(0.5);

    let s1 = SpatialScale::new(1).unwrap();
    assert!((view.projection_factor(SpatialScale::ZERO) - 10.0_f32.powf(-0.5)).abs() < 1.0e-6);
    assert!((view.projection_factor(s1) - 10.0_f32.powf(0.5)).abs() < 1.0e-6);
    assert!((view.contribution(SpatialScale::ZERO) - 0.5).abs() < 1.0e-6);
    assert!((view.contribution(s1) - 0.5).abs() < 1.0e-6);
}

#[test]
fn crossing_an_integer_zoom_boundary_normalizes_to_the_next_scale() {
    let mut view = UsfViewContext::default();
    view.add_zoom(1.0, SpatialScale::ZERO, SpatialScale::new(1).unwrap());

    assert_eq!(view.scale(), SpatialScale::new(1).unwrap());
    assert_eq!(view.zoom(), 0.0);
}

#[test]
fn semantic_and_render_anchors_are_independent() {
    let mut view = UsfViewContext::default();
    view.runtime_anchor = Vec3::new(1.0, 2.0, 3.0);
    view.render_anchor = Vec3::new(10.0, 20.0, 30.0);

    assert_eq!(view.runtime_anchor(), Vec3::new(1.0, 2.0, 3.0));
    assert_eq!(view.render_anchor(), Vec3::new(10.0, 20.0, 30.0));
}

#[test]
fn continuous_view_demand_keeps_both_adjacent_presentations_across_half_zoom() {
    let s4 = SpatialScale::new(4).unwrap();
    let s5 = SpatialScale::new(5).unwrap();
    let mut view = UsfViewContext::default();

    for exponent in [4.49, 4.51] {
        view.set_continuous_exponent(exponent);
        let demands = view.active_scale_demands();
        let lower = demands[0].expect("lower presentation demand");
        let upper = demands[1].expect("upper presentation demand");

        assert_eq!(lower.scale(), s4);
        assert_eq!(upper.scale(), s5);
        assert!(lower.contribution() > 0.0);
        assert!(upper.contribution() > 0.0);
    }
}


#[test]
fn local_presentation_authoring_units_are_explicit_across_scale_slices() {
    let s6 = SpatialScale::new(6).unwrap();

    let native = UsfLocalScalePresentation::scale_native(s6);
    let metres = UsfLocalScalePresentation::metres(s6);

    assert_eq!(native.authored_to_native_scale(), 1.0);
    assert!((metres.authored_to_native_scale() - 1.0e-6).abs() < 1.0e-12);
}


#[test]
fn scale_fallback_owns_only_views_beyond_realization_ladder() {
    let s6 = SpatialScale::new(6).unwrap();
    let s7 = SpatialScale::new(7).unwrap();
    let fallback = UsfScaleFallbackPresentation::new(s6);

    assert!(!fallback.owns_view_scale(SpatialScale::ZERO));
    assert!(!fallback.owns_view_scale(s6));
    assert!(fallback.owns_view_scale(s7));
    assert!(fallback.owns_view_scale(SpatialScale::MAX));
}
