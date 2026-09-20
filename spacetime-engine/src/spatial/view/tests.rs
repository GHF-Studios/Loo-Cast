use super::*;

#[test]
fn midpoint_between_s0_and_s1_has_reciprocal_adjacent_projection_factors() {
    let mut view = UsfViewFrame::default();
    view.set_continuous_exponent(0.5);

    let s1 = SpatialScale::new(1).unwrap();
    assert!((view.projection_factor(SpatialScale::ZERO) - 10.0_f32.powf(-0.5)).abs() < 1.0e-6);
    assert!((view.projection_factor(s1) - 10.0_f32.powf(0.5)).abs() < 1.0e-6);
    assert!((view.contribution(SpatialScale::ZERO) - 0.5).abs() < 1.0e-6);
    assert!((view.contribution(s1) - 0.5).abs() < 1.0e-6);
}

#[test]
fn crossing_an_integer_zoom_boundary_normalizes_to_the_next_scale() {
    let mut view = UsfViewFrame::default();
    view.add_zoom(1.0, SpatialScale::ZERO, SpatialScale::new(1).unwrap());

    assert_eq!(view.scale(), SpatialScale::new(1).unwrap());
    assert_eq!(view.zoom(), 0.0);
}


#[test]
fn semantic_and_render_anchors_are_independent() {
    let mut view = UsfViewFrame::default();
    view.runtime_anchor = Vec3::new(1.0, 2.0, 3.0);
    view.render_anchor = Vec3::new(10.0, 20.0, 30.0);

    assert_eq!(view.runtime_anchor(), Vec3::new(1.0, 2.0, 3.0));
    assert_eq!(view.render_anchor(), Vec3::new(10.0, 20.0, 30.0));
}
