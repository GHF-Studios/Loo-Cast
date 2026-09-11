//! Recursive portal-camera placement.
//!
//! This intentionally reproduces the original known-working renderer's
//! transform mapping and clipping behavior.

use bevy::prelude::*;

use crate::game::portal::{
    domain::{
        Portal,
        PortalPair,
        PortalView,
    },
    topology::mapping::portal_mapping,
};

use super::path::PortalRenderCamera;

pub fn update_portal_cameras(
    mut cameras: ParamSet<(
        Single<
            (
                &Transform,
                &Projection,
                &Camera,
            ),
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
            (
                Without<PortalView>,
                Without<Portal>,
            ),
        >,
    )>,
    pair: Res<PortalPair>,
    portals: Query<
        &Transform,
        With<Portal>,
    >,
) {
    let (
        primary_transform,
        primary_projection,
        primary_clear_color,
    ) = {
        let primary =
            cameras.p0();

        let (
            transform,
            projection,
            camera,
        ) = primary.into_inner();

        (
            *transform,
            projection.clone(),
            camera.clear_color.clone(),
        )
    };

    let Projection::Perspective(
        primary_projection,
    ) = primary_projection
    else {
        return;
    };

    let Ok(first) =
        portals.get(pair.first)
    else {
        return;
    };

    let Ok(second) =
        portals.get(pair.second)
    else {
        return;
    };

    for (
        portal_camera,
        mut transform,
        mut projection,
        mut camera,
    ) in &mut cameras.p1()
    {
        let (
            mapped,
            destination,
        ) = map_camera_path(
            &primary_transform,
            portal_camera.node,
            first,
            second,
        );

        let mut perspective =
            primary_projection.clone();

        perspective.near_clip_plane =
            portal_clip_plane(
                &mapped,
                destination,
            );

        *transform =
            mapped;

        *projection =
            Projection::Perspective(
                perspective,
            );

        // This is the only deliberate improvement over the earliest renderer:
        // portal targets use the same world background as the primary camera.
        camera.clear_color =
            primary_clear_color.clone();
    }
}

/// Replays the portal sequence encoded by `node`.
///
/// This is the exact traversal representation used by the original renderer.
fn map_camera_path<'a>(
    primary: &Transform,
    node: usize,
    first: &'a Transform,
    second: &'a Transform,
) -> (
    Transform,
    &'a Transform,
) {
    let mut transform =
        *primary;

    let mut destination =
        first;

    let depth =
        node_depth(node);

    for shift in
        (0..depth).rev()
    {
        let via_second =
            (node >> shift) & 1 == 1;

        let (
            source,
            target,
        ) = if via_second {
            (second, first)
        } else {
            (first, second)
        };

        transform =
            Transform::from_matrix(
                portal_mapping(
                    source,
                    target,
                )
                    * transform
                    .to_matrix(),
            );

        destination =
            target;
    }

    (
        transform,
        destination,
    )
}

/// Original working destination clipping.
///
/// The portal plane normal is oriented using the virtual camera's actual
/// position relative to the destination portal.
fn portal_clip_plane(
    camera: &Transform,
    destination: &Transform,
) -> Vec4 {
    let mut normal_world =
        destination.rotation
            * Vec3::Z;

    let plane_point =
        destination.translation
            + normal_world * 0.01;

    let camera_to_plane =
        plane_point
            - camera.translation;

    // Bevy requires the projection plane normal to point away from the camera.
    if normal_world
        .dot(camera_to_plane)
        < 0.0
    {
        normal_world =
            -normal_world;
    }

    let view_rotation =
        camera.rotation.inverse();

    let normal_view =
        (
            view_rotation
                * normal_world
        )
            .normalize();

    let point_view =
        view_rotation
            * camera_to_plane;

    normal_view.extend(
        -normal_view.dot(
            point_view,
        ),
    )
}

fn node_depth(
    node: usize,
) -> usize {
    usize::BITS as usize
        - node.leading_zeros()
        as usize
        - 1
}