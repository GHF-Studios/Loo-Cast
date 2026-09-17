//! Visual manifestation of box geometry partitioned by active portals.
//!
//! The visible halves are generated from the exact same local convex point sets
//! used for split physics colliders. For reference box geometry this keeps the
//! rendered cut and physical cut identical, including the portal-plane cap.

use bevy::{
    asset::RenderAssetUsages,
    mesh::PrimitiveTopology,
    prelude::*,
};

use crate::{
    ecs::UsfPresentationProjectionOf,
    game::portal::{
        Portal, PortalActive, PortalSplitTraveler,
        topology::mapping::portal_plane,
    },
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

fn full_box_mesh(split_box: SpatialSplitBox) -> Mesh {
    let h = split_box.half_extents;
    convex_polyhedron_mesh(&[
        Vec3::new(-h.x, -h.y, -h.z),
        Vec3::new(h.x, -h.y, -h.z),
        Vec3::new(-h.x, h.y, -h.z),
        Vec3::new(h.x, h.y, -h.z),
        Vec3::new(-h.x, -h.y, h.z),
        Vec3::new(h.x, -h.y, h.z),
        Vec3::new(-h.x, h.y, h.z),
        Vec3::new(h.x, h.y, h.z),
    ], None)
}

#[derive(Clone, Copy)]
struct LocalFacePlane {
    normal: Vec3,
    distance: f32,
}

/// Builds a flat-shaded triangle mesh from a small convex point cloud.
///
/// Split visuals omit the face lying on the portal cut plane: physics needs a
/// closed convex cap, but visually that face is the open seam whose continuation
/// is rendered by the complementary manifestation.
fn convex_polyhedron_mesh(points: &[Vec3], excluded_face: Option<LocalFacePlane>) -> Mesh {
    let mut faces: Vec<(Vec3, f32, Vec<usize>)> = Vec::new();

    for i in 0..points.len() {
        for j in (i + 1)..points.len() {
            for k in (j + 1)..points.len() {
                let raw = (points[j] - points[i]).cross(points[k] - points[i]);
                if raw.length_squared() <= FACE_EPSILON * FACE_EPSILON {
                    continue;
                }

                let mut normal = raw.normalize();
                let mut distance = normal.dot(points[i]);
                let mut positive = false;
                let mut negative = false;

                for point in points {
                    let side = normal.dot(*point) - distance;
                    positive |= side > FACE_EPSILON;
                    negative |= side < -FACE_EPSILON;
                }

                if positive && negative {
                    continue;
                }
                if positive {
                    normal = -normal;
                    distance = -distance;
                }

                if faces.iter().any(|(existing_normal, existing_distance, _)| {
                    existing_normal.dot(normal) > 1.0 - FACE_EPSILON
                        && (*existing_distance - distance).abs() <= FACE_EPSILON
                }) {
                    continue;
                }

                let coplanar: Vec<usize> = points
                    .iter()
                    .enumerate()
                    .filter_map(|(index, point)| {
                        ((normal.dot(*point) - distance).abs() <= FACE_EPSILON)
                            .then_some(index)
                    })
                    .collect();

                let excluded = excluded_face.is_some_and(|plane| {
                    coplanar.iter().all(|index| {
                        (plane.normal.dot(points[*index]) - plane.distance).abs() <= FACE_EPSILON
                    })
                });

                if coplanar.len() >= 3 && !excluded {
                    faces.push((normal, distance, coplanar));
                }
            }
        }
    }

    let mut positions: Vec<[f32; 3]> = Vec::new();
    let mut normals: Vec<[f32; 3]> = Vec::new();

    for (normal, _, mut face) in faces {
        let center = face
            .iter()
            .map(|index| points[*index])
            .fold(Vec3::ZERO, |sum, point| sum + point)
            / face.len() as f32;

        let seed = if normal.x.abs() < 0.8 { Vec3::X } else { Vec3::Y };
        let right = (seed - normal * seed.dot(normal)).normalize_or_zero();
        if right == Vec3::ZERO {
            continue;
        }
        let up = normal.cross(right).normalize_or_zero();
        if up == Vec3::ZERO {
            continue;
        }

        face.sort_by(|a, b| {
            let pa = points[*a] - center;
            let pb = points[*b] - center;
            pa.dot(up)
                .atan2(pa.dot(right))
                .total_cmp(&pb.dot(up).atan2(pb.dot(right)))
        });

        for triangle in 1..(face.len() - 1) {
            for index in [face[0], face[triangle], face[triangle + 1]] {
                positions.push(points[index].to_array());
                normals.push(normal.to_array());
            }
        }
    }

    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
    .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn box_mesh_contains_twelve_triangles() {
        let mesh = full_box_mesh(SpatialSplitBox::from_size(Vec3::splat(2.0)));
        let positions = mesh.attribute(Mesh::ATTRIBUTE_POSITION).unwrap();
        assert_eq!(positions.len(), 36);
    }
}
