//! Recursive portal-camera placement, clipping and render-target maintenance.

use bevy::{
    prelude::*,
    render::render_resource::Extent3d,
    window::{
        PrimaryWindow,
        WindowResized,
    },
};

use crate::game::portal::{
    Portal,
    PortalConfig,
    PortalPair,
    PortalView,
    geometry::portal_mapping,
};

use super::{
    PortalRenderCamera,
    PortalRenderTargets,
    node_depth,
    render_size,
};

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
        // Portal views share the ordinary world's clear/background behavior.
        camera.clear_color =
            primary_clear_color.clone();

        let (mapped, destination) =
            map_camera_path(
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

        *transform = mapped;

        *projection =
            Projection::Perspective(
                perspective,
            );
    }
}

pub fn resize_render_targets(
    mut events: MessageReader<WindowResized>,
    window: Single<
        &Window,
        With<PrimaryWindow>,
    >,
    config: Res<PortalConfig>,
    targets: Option<
        Res<PortalRenderTargets>,
    >,
    mut images: ResMut<Assets<Image>>,
) {
    if events.read().next().is_none() {
        return;
    }

    let Some(targets) = targets else {
        return;
    };

    let size =
        render_size(
            &window,
            config.render_scale,
        );

    let extent = Extent3d {
        width: size.x,
        height: size.y,
        depth_or_array_layers: 1,
    };

    for handle in &targets.0 {
        if let Some(mut image) =
            images.get_mut(handle)
        {
            image.resize(extent);
        }
    }
}

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

    for shift in (0..depth).rev() {
        let via_second =
            (node >> shift) & 1 == 1;

        let (source, target) =
            if via_second {
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
                    * transform.to_matrix(),
            );

        destination = target;
    }

    (
        transform,
        destination,
    )
}

/// Produces an oblique near plane matching the destination portal.
///
/// The normal is selected based on the actual mapped camera location, so this
/// works from either side and with arbitrary portal orientation.
fn portal_clip_plane(
    camera: &Transform,
    destination: &Transform,
) -> Vec4 {
    let mut normal_world =
        destination.rotation * Vec3::Z;

    let plane_point =
        destination.translation
            + normal_world * 0.01;

    let camera_to_plane =
        plane_point - camera.translation;

    if normal_world.dot(camera_to_plane) < 0.0 {
        normal_world =
            -normal_world;
    }

    let view_rotation =
        camera.rotation.inverse();

    let normal_view =
        (view_rotation * normal_world)
            .normalize();

    let point_view =
        view_rotation * camera_to_plane;

    normal_view.extend(
        -normal_view.dot(point_view),
    )
}