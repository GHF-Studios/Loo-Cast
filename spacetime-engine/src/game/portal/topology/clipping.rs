//! Oblique destination-plane clipping for portal render cameras.

use bevy::prelude::*;

/// Produces a view-space near clip plane for the destination aperture.
///
/// The plane normal is selected from the virtual camera's actual position
/// relative to the destination plane, rather than from the camera's forward
/// direction.
///
/// Those are deliberately different concepts: a portal remains valid while
/// viewed at an arbitrary angle.
///
/// `margin` moves the clipping plane slightly toward the virtual camera so the
/// destination frame's finite thickness is not shaved by the clip plane.
pub(crate) fn destination_clip_plane(
    camera: &Transform,
    destination: &Transform,
    margin: f32,
) -> Vec4 {
    let mut normal_world =
        destination.rotation * Vec3::Z;

    let camera_to_portal =
        destination.translation
            - camera.translation;

    // Bevy requires the clip-plane normal to point away from the camera.
    //
    // Therefore choose the physical orientation of the portal plane that
    // points from the camera toward the portal.
    if normal_world.dot(camera_to_portal) < 0.0 {
        normal_world =
            -normal_world;
    }

    // Move slightly toward the camera.
    //
    // Since `normal_world` points camera -> portal, subtracting it moves the
    // clip plane back toward the camera.
    let plane_point =
        destination.translation
            - normal_world * margin;

    let view_rotation =
        camera.rotation.inverse();

    let normal_view =
        (view_rotation * normal_world)
            .normalize();

    let point_view =
        view_rotation
            * (plane_point
            - camera.translation);

    normal_view.extend(
        -normal_view.dot(point_view),
    )
}