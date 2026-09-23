//! Runtime camera transform, FOV and self-presentation policy.

use bevy::camera::visibility::RenderLayers;

use crate::{
    ecs::{UsfManifestationOf, UsfPresentationProjectionOf},
    portal::DERIVED_VIEW_LAYER,
    view::ViewSubjectPresentation,
};

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
    spatial_query: SpatialQuery,
    physics_charts: UsfPhysicsCharts,
    controller: Single<&PlayerAim, With<Player>>,
    subject: Single<
        (
            Entity,
            &Transform,
            &CharacterControlFrame,
            Option<&CharacterStance>,
            &UsfManifestationOf,
            &UsfScaleLayer,
            &mut ViewCameraProfile,
        ),
        (
            With<LocalViewTarget>,
            With<UsfLogicalProjection>,
            Without<PlayerCamera>,
        ),
    >,
    camera: Single<
        (&PlayerCamera, &mut Transform),
        (With<PlayerCamera>, Without<LocalViewTarget>),
    >,
    semantic_entities: Query<&UsfManifestations>,
    portals: Query<
        (Entity, &Portal, &PortalActive, &Transform),
        (With<Portal>, Without<PlayerCamera>),
    >,
) {
    let aim = controller.into_inner();
    let (
        subject_entity,
        body,
        control,
        stance,
        manifestation,
        layer,
        mut profile,
    ) = subject.into_inner();
    let (camera, mut camera_transform) = camera.into_inner();

    let view_rotation = camera.view_rotation(control, aim);
    let eye = body.translation
        + control.rotation() * profile.eye_offset_native(stance, layer.scale());

    *camera_transform = match camera.mode {
        CameraMode::FirstPerson => Transform {
            translation: eye,
            rotation: view_rotation,
            ..default()
        },
        CameraMode::ThirdPerson => {
            let pivot = eye
                + control.rotation()
                    * Vec3::Y
                    * layer
                        .scale()
                        .metres_to_native_f32(profile.third_person.pivot_height_metres);
            let resolved = resolve_third_person_boom(
                &spatial_query,
                &physics_charts,
                &semantic_entities,
                &portals,
                subject_entity,
                manifestation,
                layer.scale(),
                pivot,
                view_rotation,
                &profile.third_person,
            );
            profile.third_person.resolved_distance_metres =
                (f64::from(resolved.distance) * layer.scale().scale0_units_per_native()) as f32;
            resolved.transform
        }
    };
}

/// Mirrors local view orientation/projection into the dedicated USF pass while
/// pinning translation to the semantic subject anchor.
///
/// The local camera may move through cockpit/third-person rig space. The USF
/// projection camera may NOT: translating it would make presentation-scale
/// compression observable as fake geometry.
pub(in crate::game::player) fn sync_usf_projection_camera(
    target: Single<
        &Transform,
        (
            With<LocalViewTarget>,
            Without<PlayerCamera>,
            Without<UsfProjectionCamera>,
        ),
    >,
    local: Single<
        (&Transform, &Projection, &Camera, &bevy::camera::RenderTarget),
        (With<PlayerCamera>, Without<UsfProjectionCamera>),
    >,
    far: Single<
        (
            &mut Transform,
            &mut Projection,
            &mut Camera,
            &mut bevy::camera::RenderTarget,
        ),
        (With<UsfProjectionCamera>, Without<PlayerCamera>),
    >,
) {
    let target = target.into_inner();
    let (local_transform, local_projection, local_camera, local_target) = local.into_inner();
    let (mut far_transform, mut far_projection, mut far_camera, mut far_target) =
        far.into_inner();

    far_transform.translation = target.translation;
    far_transform.rotation = local_transform.rotation;
    far_transform.scale = Vec3::ONE;

    *far_projection = local_projection.clone();
    far_camera.viewport = local_camera.viewport.clone();
    far_camera.is_active = local_camera.is_active;
    *far_target = local_target.clone();
}

/// Self-visibility is primary-view policy, not model identity or portal policy.
///
/// All manifestations of the viewed semantic subject are hidden from the
/// primary first-person camera by moving body presentations onto the derived
/// view layer. Portal cameras intentionally include that layer. Presentations
/// of previous/unrelated view subjects are restored to ordinary world layers.
pub(in crate::game::player) fn sync_view_subject_presentations(
    camera: Single<&PlayerCamera>,
    target: Single<&UsfManifestationOf, With<LocalViewTarget>>,
    manifestations: Query<&UsfManifestationOf>,
    mut presentations: Query<
        (&UsfPresentationProjectionOf, &mut RenderLayers),
        With<ViewSubjectPresentation>,
    >,
) {
    let viewed_semantic = target.0;

    for (projection, mut layers) in &mut presentations {
        let is_self = manifestations
            .get(projection.0)
            .is_ok_and(|manifestation| manifestation.0 == viewed_semantic);

        let desired = if is_self && camera.mode == CameraMode::FirstPerson {
            RenderLayers::layer(DERIVED_VIEW_LAYER)
        } else {
            RenderLayers::default()
        };

        if *layers != desired {
            *layers = desired;
        }
    }
}

/// Bevy stores perspective FOV vertically. Keep the requested gameplay FOV
/// horizontal and derive the vertical value from the logical game-view aspect.
pub(in crate::game::player) fn sync_player_fov(
    camera: Single<(&PlayerCamera, &Camera, &mut Projection)>,
) {
    let (settings, camera, mut projection) = camera.into_inner();
    let Projection::Perspective(perspective) = projection.as_mut() else {
        return;
    };

    perspective.near = 0.001;
    perspective.far = 100_000.0;

    let Some(size) = camera
        .logical_viewport_size()
        .or_else(|| camera.logical_target_size())
    else {
        return;
    };
    if size.y <= 0.0 {
        return;
    }

    let aspect = size.x / size.y;
    if aspect <= 0.0 {
        return;
    }

    let horizontal = settings
        .horizontal_fov_degrees
        .clamp(1.0, 179.0)
        .to_radians();

    perspective.fov = 2.0 * ((horizontal * 0.5).tan() / aspect).atan();
}
