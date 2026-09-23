use super::*;

#[test]
fn zoom_intent_ignores_collision_resolved_distance() {
    let mut camera = ThirdPersonCamera {
        base_distance_metres: 4.0,
        zoom_offset_metres: 1.0,
        resolved_distance_metres: 0.75,
        ..default()
    };

    camera.add_zoom_steps(-1.0);

    assert_eq!(camera.desired_distance_metres(), 4.5);
    assert_eq!(camera.zoom_offset_metres, 0.5);
    assert_eq!(camera.resolved_distance_metres, 0.75);
}


#[test]
fn camera_rig_metres_do_not_change_with_interaction_scale() {
    let camera = ThirdPersonCamera {
        base_distance_metres: 14.0,
        ..default()
    };
    let s0 = crate::spatial::SpatialScale::ZERO;
    let s3 = crate::spatial::SpatialScale::new(3).unwrap();

    let at_s0 = camera.desired_distance_native(s0);
    let at_s3 = camera.desired_distance_native(s3);

    assert!((at_s0 - 14.0).abs() < 1.0e-6);
    assert!((at_s3 - 0.014).abs() < 1.0e-6);
}
