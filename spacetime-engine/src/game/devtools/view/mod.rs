//! Resolution of the game camera/cursor into the generic developer interaction view.

use super::*;

pub(super) fn resolve_developer_view(
    window: Single<&Window, With<PrimaryWindow>>,
    camera: Single<(Entity, &Camera, &GlobalTransform), With<PrimaryGameView>>,
    capture: Res<CursorCapture>,
    tools: Res<DeveloperTools>,
    presentation: Res<PrimaryViewPresentation>,
    mut view: ResMut<DeveloperView>,
) {
    let (entity, camera, camera_transform) = camera.into_inner();
    view.set_observer(Some(entity));

    if !tools.enabled() && !presentation.is_embedded() {
        view.set_interaction_ray(None);
        return;
    }

    let space = ViewportSpace::new(camera);
    let target_position = if capture.active() {
        space.target_center()
    } else {
        window
            .cursor_position()
            .filter(|position| space.contains_target_position(*position))
    };

    view.set_interaction_ray(
        target_position.and_then(|position| space.target_to_world_ray(camera_transform, position)),
    );
}
