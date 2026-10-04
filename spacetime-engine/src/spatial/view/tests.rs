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
fn direct_projection_rejects_scale_stack_escape_from_final_render_chart() {
    let mut view = UsfViewContext::default();
    view.set_continuous_exponent(0.0);

    assert!(
        view.direct_projection_factor(SpatialScale::new(4).unwrap())
            .is_some()
    );
    assert!(
        view.direct_projection_factor(SpatialScale::new(5).unwrap())
            .is_none()
    );
    assert!(
        view.direct_projection_factor(SpatialScale::MAX)
            .is_none()
    );
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
fn contextual_view_stack_keeps_all_coarser_scales_eligible() {
    let mut view = UsfViewContext::default();
    view.set_continuous_exponent(4.49);

    assert!(!view.context_scale_eligible(SpatialScale::new(3).unwrap()));
    assert!(view.context_scale_eligible(SpatialScale::new(4).unwrap()));
    assert!(view.context_scale_eligible(SpatialScale::new(5).unwrap()));
    assert!(view.context_scale_eligible(SpatialScale::MAX));
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

#[test]
fn presentation_probe_filters_passes_without_changing_default_composition() {
    let all = UsfPresentationProbe::default();
    assert!(all.physical_enabled());
    assert!(all.context_enabled());

    let physical = UsfPresentationProbe::Physical;
    assert!(physical.physical_enabled());
    assert!(!physical.context_enabled());

    let context = UsfPresentationProbe::Context;
    assert!(!context.physical_enabled());
    assert!(context.context_enabled());
}

#[test]
fn same_semantic_point_projects_identically_from_different_source_scales() {
    let mut view = UsfViewContext::default();
    view.set_continuous_exponent(2.5);
    view.set_projection_eye_offset_metres(DVec3::new(0.0, 1.7, -4.0));

    let semantic_relative_metres = DVec3::new(12_345.0, -87.0, 456.0);

    let s0 = SpatialScale::ZERO;
    let s3 = SpatialScale::new(3).unwrap();

    let s0_native = Vec3::new(
        semantic_relative_metres.x as f32,
        semantic_relative_metres.y as f32,
        semantic_relative_metres.z as f32,
    );
    let s3_native = Vec3::new(
        (semantic_relative_metres.x / s3.metres_per_native()) as f32,
        (semantic_relative_metres.y / s3.metres_per_native()) as f32,
        (semantic_relative_metres.z / s3.metres_per_native()) as f32,
    );

    let from_s0 = view
        .project_relative_native_from_eye(s0_native, s0)
        .unwrap();
    let from_s3 = view
        .project_relative_native_from_eye(s3_native, s3)
        .unwrap();

    assert!(
        (from_s0 - from_s3).length() < 1.0e-4,
        "representation Scale must not change projected semantic position: {from_s0:?} vs {from_s3:?}",
    );
}

#[test]
fn presentation_exponent_preserves_physical_camera_ray() {
    let eye = DVec3::new(0.25, 1.7, -4.0);
    let point = DVec3::new(23.0, -2.0, 91.0);
    let physical_ray = (point - eye).normalize();

    for exponent in [0.0_f32, 0.5, 3.25, 6.0, 20.0] {
        let mut view = UsfViewContext::default();
        view.set_continuous_exponent(exponent);
        view.set_projection_eye_offset_metres(eye);

        let projected = view
            .project_relative_metres_from_eye(point)
            .unwrap();
        let projected_ray = projected.normalize();

        assert!(
            (projected_ray - physical_ray).length() < 1.0e-12,
            "presentation exponent {exponent} changed the camera ray: {projected_ray:?} vs {physical_ray:?}",
        );
    }
}
