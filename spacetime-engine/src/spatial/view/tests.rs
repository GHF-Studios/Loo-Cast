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
fn persistent_scale_stack_does_not_hard_switch_at_half_zoom() {
    let s4 = SpatialScale::new(4).unwrap();
    let s5 = SpatialScale::new(5).unwrap();
    let s3 = SpatialScale::new(3).unwrap();

    let mut view = UsfViewContext::default();

    view.set_continuous_exponent(4.51);
    assert!(view.requests_scale_stack_layer(s5));
    assert!(view.requests_scale_stack_layer(s4));
    assert!(!view.requests_scale_stack_layer(s3));

    view.set_continuous_exponent(4.49);
    assert!(view.requests_scale_stack_layer(s5));
    assert!(view.requests_scale_stack_layer(s4));
    assert!(!view.requests_scale_stack_layer(s3));
}

#[test]
fn persistent_stack_keeps_all_coarser_layers_when_refined() {
    let mut view = UsfViewContext::default();
    view.set_continuous_exponent(2.25);

    for raw in 3..=5 {
        assert!(
            view.requests_scale_stack_layer(SpatialScale::new(raw).unwrap()),
            "S+{raw} should remain part of the additive stack"
        );
    }

    assert!(view.requests_scale_stack_layer(SpatialScale::new(2).unwrap()));
    assert!(!view.requests_scale_stack_layer(SpatialScale::new(1).unwrap()));
}
