//! Physical-camera field of view and active Scale-Slice near plane.

use super::*;

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
