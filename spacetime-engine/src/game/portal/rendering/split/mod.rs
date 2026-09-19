//! Visual manifestation of box geometry partitioned by active portals.
//!
//! The visible halves are generated from the exact same local convex point sets
//! used for split physics colliders. For reference box geometry this keeps the
//! rendered cut and physical cut identical, including the portal-plane cap.

use bevy::{asset::RenderAssetUsages, mesh::PrimitiveTopology, prelude::*};

use crate::{
    ecs::UsfPresentationProjectionOf,
    game::portal::{Portal, PortalActive, PortalSplitTraveler, topology::mapping::portal_plane},
    physics::topology::{
        SpatialSplitBox, SpatialSplitPeer, SpatialSplitPeerActive, partition_box_by_plane,
    },
};

const FACE_EPSILON: f32 = 1.0e-4;

/// Marks a child mesh whose geometry follows its parent split manifestation.
#[derive(Component, Debug, Default, Clone, Copy)]
pub struct PortalSplitVisual;

pub(super) fn sync_split_visuals(
    portals: Query<(&PortalActive, &Transform), With<Portal>>,
    authorities: Query<
        (&SpatialSplitBox, &Transform, &PortalSplitTraveler),
        (Without<SpatialSplitPeer>, Without<Portal>),
    >,
    peers: Query<(&SpatialSplitPeer, Option<&SpatialSplitPeerActive>)>,
    mut visuals: Query<
        (&UsfPresentationProjectionOf, &mut Mesh3d, &mut Visibility),
        With<PortalSplitVisual>,
    >,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    for (projection, mesh, mut visibility) in &mut visuals {
        let parent = projection.0;
        let (authority, is_peer) = if authorities.get(parent).is_ok() {
            (parent, false)
        } else if let Ok((peer, active)) = peers.get(parent) {
            if active.is_none() {
                *visibility = Visibility::Hidden;
                continue;
            }
            (peer.authority, true)
        } else {
            continue;
        };

        let Ok((split_box, body, split)) = authorities.get(authority) else {
            continue;
        };

        let Some(mut mesh_asset) = meshes.get_mut(&mesh.0) else {
            continue;
        };

        let Some(active) = split.active else {
            if is_peer {
                *visibility = Visibility::Hidden;
            } else {
                *visibility = Visibility::Inherited;
                *mesh_asset = full_box_mesh(*split_box);
            }
            continue;
        };

        let Ok((source_active, source)) = portals.get(active.source) else {
            if is_peer {
                *visibility = Visibility::Hidden;
            } else {
                *visibility = Visibility::Inherited;
                *mesh_asset = full_box_mesh(*split_box);
            }
            continue;
        };
        if !source_active.0 {
            if is_peer {
                *visibility = Visibility::Hidden;
            } else {
                *visibility = Visibility::Inherited;
                *mesh_asset = full_box_mesh(*split_box);
            }
            continue;
        }

        let Some(plane) = portal_plane(source) else {
            if is_peer {
                *visibility = Visibility::Hidden;
            } else {
                *visibility = Visibility::Inherited;
                *mesh_asset = full_box_mesh(*split_box);
            }
            continue;
        };
        let partition = partition_box_by_plane(*split_box, body, plane);
        if !partition.straddles() {
            if is_peer {
                *visibility = Visibility::Hidden;
            } else {
                *visibility = Visibility::Inherited;
                *mesh_asset = full_box_mesh(*split_box);
            }
            continue;
        }

        let authority_positive = partition.center_distance >= 0.0;
        let points = match (is_peer, authority_positive) {
            (false, true) | (true, false) => &partition.positive_points,
            (false, false) | (true, true) => &partition.negative_points,
        };
        let inverse_rotation = body.rotation.inverse();
        let local_cut_normal = inverse_rotation * plane.normal;
        let local_cut_point = inverse_rotation * (plane.point - body.translation);
        let cut_plane = LocalFacePlane {
            normal: local_cut_normal,
            distance: local_cut_normal.dot(local_cut_point),
        };

        *mesh_asset = convex_polyhedron_mesh(points, Some(cut_plane));
        *visibility = Visibility::Inherited;
    }
}

mod mesh;

use mesh::{LocalFacePlane, convex_polyhedron_mesh, full_box_mesh};

#[cfg(test)]
mod tests;
