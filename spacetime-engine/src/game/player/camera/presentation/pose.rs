//! Subject profile handoff and resolved local camera pose.

use super::*;

/// Applies the preferred mode only when the viewed manifestation changes.
///
/// User F5 intent is persistent while viewing one subject, but a control/view
/// transfer starts from the new subject's authored camera policy.
pub(in crate::game::player) fn sync_view_camera_profile(
    target: Single<(Entity, &ViewCameraProfile), With<LocalViewTarget>>,
    mut camera: Single<&mut PlayerCamera>,
    mut previous: Local<Option<Entity>>,
) {
    let (entity, profile) = target.into_inner();
    if *previous == Some(entity) {
        return;
    }

    camera.mode = profile.preferred_mode;
    *previous = Some(entity);
}

/// Resolves camera presentation after simulation/topology.
///
/// Controller aim and viewed subject are intentionally independent. The local
/// player supplies aim intent; LocalViewTarget supplies physical pose.
pub(in crate::game::player) fn sync_player_camera(
    freecam: Res<DebugFreecam>,
    spatial_query: SpatialQuery,
    physics_charts: UsfPhysicsSliceQuery,
    runtime_ownership: UsfRuntimeOwnershipQuery,
    controller: Single<&PlayerAim, With<Player>>,
    subject: Single<
        (
            Entity,
            &Transform,
            &CharacterControlFrame,
            Option<&CharacterStance>,
            &UsfScaleLayer,
            &mut ViewCameraProfile,
        ),
        (
            With<LocalViewTarget>,
            With<UsfLogicalRealizationOf>,
            Without<PlayerCamera>,
        ),
    >,
    camera: Single<(&PlayerCamera, &mut Transform), (With<PlayerCamera>, Without<LocalViewTarget>)>,
    portals: Query<
        (Entity, &Portal, &PortalActive, &Transform),
        (With<Portal>, Without<PlayerCamera>),
    >,
) {
    if freecam.enabled() {
        return;
    }

    let aim = controller.into_inner();
    let (subject_entity, body, control, stance, layer, mut profile) = subject.into_inner();
    let (camera, mut camera_transform) = camera.into_inner();

    let rig_rotation = profile.rig_rotation(body, control);
    let view_rotation = profile.view_rotation(body, control, aim, camera);
    let eye = body.translation + rig_rotation * profile.eye_offset_native(stance, layer.scale());

    *camera_transform = match camera.mode {
        CameraMode::FirstPerson => Transform {
            translation: eye,
            rotation: view_rotation,
            ..default()
        },
        CameraMode::ThirdPerson | CameraMode::Orbit => {
            let pivot = eye
                + rig_rotation
                    * Vec3::Y
                    * layer
                        .scale()
                        .metres_to_native_f32(profile.third_person.pivot_height_metres);
            let resolved = resolve_third_person_boom(
                &spatial_query,
                &physics_charts,
                &runtime_ownership,
                &portals,
                subject_entity,
                layer.scale(),
                pivot,
                view_rotation,
                &profile.third_person,
            );
            profile.third_person.resolved_distance_metres =
                (f64::from(resolved.distance) * layer.scale().metres_per_native()) as f32;
            resolved.transform
        }
    };
}
