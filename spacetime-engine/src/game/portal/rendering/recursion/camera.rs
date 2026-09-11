//! Per-frame recursive virtual-camera placement and portal-plane clipping.

use bevy::prelude::*;

use crate::game::portal::{
    domain::{
        Portal,
        PortalPair,
        PortalView,
    },
    rendering::layout::CLIP_MARGIN,
    topology::{
        clipping::destination_clip_plane,
        mapping::map_transform,
    },
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

    for (
        render_camera,
        mut transform,
        mut projection,
        mut camera,
    ) in &mut cameras.p1()
    {
        let Some((
            mapped,
            destination,
        )) = map_render_path(
            &primary_transform,
            &render_camera.path,
            *pair,
            &portals,
        )
        else {
            continue;
        };

        let mut perspective =
            primary_projection.clone();

        perspective.near_clip_plane =
            destination_clip_plane(
                &mapped,
                &destination,
                CLIP_MARGIN,
            );

        *transform = mapped;

        *projection =
            Projection::Perspective(
                perspective,
            );

        camera.clear_color =
            primary_clear_color.clone();
    }
}

fn map_render_path(
    primary: &Transform,
    path: &[crate::game::portal::domain::PortalEndpoint],
    pair: PortalPair,
    portals: &Query<
        &Transform,
        With<Portal>,
    >,
) -> Option<(Transform, Transform)> {
    let mut mapped =
        *primary;

    let mut final_destination =
        None;

    for endpoint in path {
        let source =
            *portals
                .get(
                    pair.entity(*endpoint),
                )
                .ok()?;

        let destination =
            *portals
                .get(
                    pair.entity(
                        endpoint.other(),
                    ),
                )
                .ok()?;

        mapped =
            map_transform(
                &mapped,
                &source,
                &destination,
            );

        final_destination =
            Some(destination);
    }

    final_destination.map(
        |destination| (
            mapped,
            destination,
        ),
    )
}
