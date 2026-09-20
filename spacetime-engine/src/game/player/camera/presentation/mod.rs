//! Runtime camera transform, FOV and player-model presentation.

use super::*;

/// Resolves camera presentation after simulation/topology.
///
/// Third person treats the boom as a short path through portal topology rather
/// than one Euclidean segment. The camera can therefore cross a portal before
/// the player, or remain through the portal behind the player after the player
/// crosses. Each path segment still uses a sphere cast so ordinary walls and
/// corners push the camera inward.
pub(in crate::game::player) fn sync_player_camera(
    spatial_query: SpatialQuery,
    physics_charts: UsfPhysicsCharts,
    player: Single<
        (
            Entity,
            &Transform,
            &CharacterControlFrame,
            &PlayerAim,
            &PlayerStance,
            &UsfManifestationOf,
            &UsfScaleLayer,
        ),
        (
            With<Player>,
            With<UsfLogicalProjection>,
            Without<PlayerCamera>,
        ),
    >,
    camera: Single<(&mut PlayerCamera, &mut Transform), (With<PlayerCamera>, Without<Player>)>,
    semantic_entities: Query<&UsfManifestations>,
    portals: Query<
        (Entity, &Portal, &PortalActive, &Transform),
        (With<Portal>, Without<PlayerCamera>),
    >,
) {
    let (player_entity, body, control, aim, stance, manifestation, layer) =
        player.into_inner();
    let (mut camera, mut camera_transform) = camera.into_inner();

    // Camera rig dimensions are presentation-space values. They remain
    // visually useful across scale changes while USF projection keeps
    // scenery observer-relative to the actual camera.
    let view_rotation = camera.view_rotation(control, aim);
    let eye = camera.eye_position(body, control, stance);

    *camera_transform = match camera.mode {
        CameraMode::FirstPerson => Transform {
            translation: eye,
            rotation: view_rotation,
            ..default()
        },
        CameraMode::ThirdPerson => {
            let pivot =
                eye + control.rotation() * Vec3::Y * camera.third_person.pivot_height;
            let resolved = resolve_third_person_boom(
                &spatial_query,
                &physics_charts,
                &semantic_entities,
                &portals,
                player_entity,
                manifestation,
                layer.scale(),
                pivot,
                view_rotation,
                &camera.third_person,
            );
            camera.third_person.resolved_distance = resolved.distance;
            resolved.transform
        }
    };
}

/// Bevy stores perspective FOV vertically. Keep the requested gameplay FOV
/// horizontal and derive the vertical value from the logical game-view aspect,
/// not from the containing window. This remains correct when the game is embedded.
pub(in crate::game::player) fn sync_player_fov(camera: Single<(&PlayerCamera, &Camera, &mut Projection)>) {
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

pub(in crate::game::player) fn sync_player_model(
    camera: Single<&PlayerCamera>,
    mut models: Query<&mut RenderLayers, With<PlayerModel>>,
) {
    for mut render_layers in &mut models {
        *render_layers = match camera.mode {
            CameraMode::FirstPerson => RenderLayers::layer(DERIVED_VIEW_LAYER),
            CameraMode::ThirdPerson => RenderLayers::default(),
        };
    }
}
