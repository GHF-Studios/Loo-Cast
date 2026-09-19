//! Recursive directed portal-camera placement.
//!
//! Front and back faces get independent cameras/render targets, but both use
//! the same proven rigid physical portal mapping.

use bevy::prelude::*;

use crate::portal::{
    domain::{Portal, PortalActive, PortalFace, PortalPair, PortalView},
    topology::mapping::map_transform,
};

use super::path::PortalRenderCamera;

pub fn update_portal_cameras(
    mut cameras: ParamSet<(
        Single<
            (&Transform, &Projection, &Camera),
            (
                With<PortalView>,
                Without<PortalRenderCamera>,
                Without<Portal>,
            ),
        >,
        Query<
            (
                &PortalRenderCamera,
                &mut Transform,
                &mut Projection,
                &mut Camera,
            ),
            (Without<PortalView>, Without<Portal>),
        >,
    )>,
    pair: Res<PortalPair>,
    active: Query<&PortalActive, With<Portal>>,
    portals: Query<&Transform, With<Portal>>,
) {
    let pair_active = [pair.first, pair.second]
        .into_iter()
        .all(|entity| active.get(entity).is_ok_and(|active| active.0));
    if !pair_active {
        return;
    }

    let (primary_transform, primary_projection, primary_clear_color) = {
        let primary = cameras.p0();

        let (transform, projection, camera) = primary.into_inner();

        (*transform, projection.clone(), camera.clear_color.clone())
    };

    let Projection::Perspective(primary_projection) = primary_projection else {
        return;
    };

    for (portal_camera, mut transform, mut projection, mut camera) in &mut cameras.p1() {
        let Some((mapped, destination)) =
            map_camera_path(&primary_transform, &portal_camera.path, *pair, &portals)
        else {
            continue;
        };

        let mut perspective = primary_projection.clone();

        // Preserve the exact clipping mechanism from the known-working
        // one-sided renderer.
        perspective.near_clip_plane = portal_clip_plane(&mapped, &destination);

        *transform = mapped;

        *projection = Projection::Perspective(perspective);

        camera.clear_color = primary_clear_color.clone();
    }
}

/// Applies each directed portal traversal.
///
/// Important:
///
/// The rigid world mapping itself is identical from front and back. The side
/// distinction controls which surface owns this render target and which exit
/// surface is suppressed in the child rendering context.
///
/// That lets us retain the exact mapping that already proved correct.
fn map_camera_path(
    primary: &Transform,
    path: &[PortalFace],
    pair: PortalPair,
    portals: &Query<&Transform, With<Portal>>,
) -> Option<(Transform, Transform)> {
    let mut mapped = *primary;

    let mut final_destination = None;

    for face in path {
        let source = *portals.get(pair.entity(face.endpoint)).ok()?;

        let destination = *portals.get(pair.entity(face.endpoint.other())).ok()?;

        // EXACT same rigid mapping used by the working one-sided renderer.
        mapped = map_transform(&mapped, &source, &destination);

        final_destination = Some(destination);
    }

    final_destination.map(|destination| (mapped, destination))
}

/// Exact clipping rule from the known-working one-sided renderer.
fn portal_clip_plane(camera: &Transform, destination: &Transform) -> Vec4 {
    let mut normal_world = destination.rotation * Vec3::Z;

    let plane_point = destination.translation + normal_world * 0.01;

    let camera_to_plane = plane_point - camera.translation;

    if normal_world.dot(camera_to_plane) < 0.0 {
        normal_world = -normal_world;
    }

    let view_rotation = camera.rotation.inverse();

    let normal_view = (view_rotation * normal_world).normalize();

    let point_view = view_rotation * camera_to_plane;

    normal_view.extend(-normal_view.dot(point_view))
}
