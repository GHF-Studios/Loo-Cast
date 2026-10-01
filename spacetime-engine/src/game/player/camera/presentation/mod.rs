//! Runtime camera transform, FOV and self-presentation policy.

use bevy::camera::visibility::RenderLayers;

use crate::{
    ecs::{UsfLogicalRealizationOf, UsfPresentationProjectionOf},
    portal::DERIVED_VIEW_LAYER,
    physics::topology::UsfRuntimeOwnershipQuery,
    view::ViewSubjectPresentation,
};

use super::*;

/// Contextual USF projection uses a bounded presentation domain, not the active
/// physical Scale Slice. Do not inherit subject-metre clip conversion here.
const USF_PROJECTION_NEAR_UNITS: f32 = 0.001;
const USF_PROJECTION_FAR_UNITS: f32 = 100_000.0;

fn apply_usf_projection_clip_domain(projection: &mut Projection) {
    let Projection::Perspective(perspective) = projection else {
        return;
    };
    perspective.near = USF_PROJECTION_NEAR_UNITS;
    perspective.far = USF_PROJECTION_FAR_UNITS;
}

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
    physics_charts: UsfPhysicsSlices,
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
    camera: Single<
        (&PlayerCamera, &mut Transform),
        (With<PlayerCamera>, Without<LocalViewTarget>),
    >,
    portals: Query<
        (Entity, &Portal, &PortalActive, &Transform),
        (With<Portal>, Without<PlayerCamera>),
    >,
) {
    if freecam.enabled() {
        return;
    }

    let aim = controller.into_inner();
    let (
        subject_entity,
        body,
        control,
        stance,
        layer,
        mut profile,
    ) = subject.into_inner();
    let (camera, mut camera_transform) = camera.into_inner();

    let rig_rotation = profile.rig_rotation(body, control);
    let view_rotation = profile.view_rotation(body, control, aim);
    let eye = body.translation
        + rig_rotation * profile.eye_offset_native(stance, layer.scale());

    *camera_transform = match camera.mode {
        CameraMode::FirstPerson => Transform {
            translation: eye,
            rotation: view_rotation,
            ..default()
        },
        CameraMode::ThirdPerson => {
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
    freecam: Res<DebugFreecam>,
    observation: Res<crate::spatial::UsfViewObservationOverride>,
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

    far_camera.viewport = local_camera.viewport.clone();
    *far_target = local_target.clone();
    far_transform.scale = Vec3::ONE;

    if !freecam.enabled() {
        far_transform.translation = target.translation;
        far_transform.rotation = local_transform.rotation;
        *far_projection = local_projection.clone();
        apply_usf_projection_clip_domain(&mut far_projection);
        far_camera.is_active = local_camera.is_active;
        return;
    }

    match freecam.projection_policy() {
        FreecamProjectionPolicy::Follow => {
            far_transform.translation = observation
                .current()
                .map_or(target.translation, |(_, runtime)| runtime);
            far_transform.rotation = local_transform.rotation;
            *far_projection = local_projection.clone();
        apply_usf_projection_clip_domain(&mut far_projection);
            far_camera.is_active = local_camera.is_active;
        }
        FreecamProjectionPolicy::Frozen => {
            if let Some((_, runtime)) = observation.current() {
                far_transform.translation = runtime;
            }
            far_camera.is_active = local_camera.is_active;
        }
        FreecamProjectionPolicy::Disabled => {
            far_camera.is_active = false;
        }
    }
}

/// Self-visibility is primary-view policy, not model identity or portal policy.
///
/// All manifestations of the viewed semantic subject are hidden from the
/// primary first-person camera by moving body presentations onto the derived
/// view layer. Portal cameras intentionally include that layer. Presentations
/// of previous/unrelated view subjects are restored to ordinary world layers.
pub(in crate::game::player) fn sync_view_subject_presentations(
    freecam: Res<DebugFreecam>,
    camera: Single<&PlayerCamera>,
    runtime_ownership: UsfRuntimeOwnershipQuery,
    target: Single<Entity, With<LocalViewTarget>>,
    mut presentations: Query<
        (&UsfPresentationProjectionOf, &mut RenderLayers),
        With<ViewSubjectPresentation>,
    >,
) {
    let viewed_semantic = runtime_ownership.semantic_of(target.into_inner());

    for (projection, mut layers) in &mut presentations {
        let is_self = viewed_semantic.is_some()
            && runtime_ownership.semantic_of(projection.0) == viewed_semantic;

        let desired = if is_self
            && !freecam.enabled()
            && camera.mode == CameraMode::FirstPerson
        {
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
    target: Single<(&ViewCameraProfile, &UsfScaleLayer), With<LocalViewTarget>>,
    camera: Single<(&PlayerCamera, &Camera, &mut Projection)>,
) {
    let (profile, layer) = target.into_inner();
    let (settings, camera, mut projection) = camera.into_inner();
    let Projection::Perspective(perspective) = projection.as_mut() else {
        return;
    };

    // This camera is expressed in active interaction-chart units. The old raw
    // 0.001 meant 1 mm at S0, 1 m at S+3, 10 m at S+4 and 1 km at S+6.
    perspective.near = profile.near_clip_native(layer.scale());

    // Bevy perspective depth is infinite reverse-Z; keep `far` as the bounded
    // runtime-chart visibility/culling horizon instead of semantic metres.
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
