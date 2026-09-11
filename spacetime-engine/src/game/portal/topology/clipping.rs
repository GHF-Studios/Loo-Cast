//! Oblique destination-plane clipping for portal render cameras.

use bevy::prelude::*;

/// Produces a view-space near clip plane for the destination aperture.
///
/// `margin` moves the clip plane slightly toward the virtual camera. This keeps
/// the destination portal frame, which has finite depth around the mathematical
/// portal plane, from being shaved by the oblique clip plane.
pub(crate) fn destination_clip_plane(
    camera: &Transform,
    destination: &Transform,
    margin: f32,
) -> Vec4 {
    let portal_normal =
        destination.rotation * Vec3::Z;

    let camera_forward =
        camera.rotation * Vec3::NEG_Z;

    // Bevy requires the view-space plane normal to point away from the camera.
    // Pick whichever physical orientation of the portal plane agrees with the
    // virtual camera's forward direction.
    let normal_world =
        if portal_normal.dot(camera_forward) >= 0.0 {
            portal_normal
        } else {
            -portal_normal
        };

    // Pull the mathematical clipping plane slightly toward the camera so the
    // frame's thickness remains on the rendered side of the plane.
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
            * (plane_point - camera.translation);

    normal_view.extend(
        -normal_view.dot(point_view),
    )
}
