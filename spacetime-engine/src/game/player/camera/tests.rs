use super::*;

#[test]
fn zoom_intent_ignores_collision_resolved_distance() {
    let mut camera = ThirdPersonCamera {
        base_distance: 4.0,
        zoom_offset: 1.0,
        resolved_distance: 0.75,
        ..default()
    };

    camera.add_zoom_steps(-1.0);

    assert_eq!(camera.desired_distance(), 4.5);
    assert_eq!(camera.zoom_offset, 0.5);
    assert_eq!(camera.resolved_distance, 0.75);
}
