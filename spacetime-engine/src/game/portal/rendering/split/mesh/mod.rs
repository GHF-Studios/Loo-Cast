//! Convex mesh construction for portal-partitioned box manifestations.

use super::*;

pub(super) fn full_box_mesh(split_box: SpatialSplitBox) -> Mesh {
    let h = split_box.half_extents;
    convex_polyhedron_mesh(
        &[
            Vec3::new(-h.x, -h.y, -h.z),
            Vec3::new(h.x, -h.y, -h.z),
            Vec3::new(-h.x, h.y, -h.z),
            Vec3::new(h.x, h.y, -h.z),
            Vec3::new(-h.x, -h.y, h.z),
            Vec3::new(h.x, -h.y, h.z),
            Vec3::new(-h.x, h.y, h.z),
            Vec3::new(h.x, h.y, h.z),
        ],
        None,
    )
}

#[derive(Clone, Copy)]
pub(super) struct LocalFacePlane {
    pub(super) normal: Vec3,
    pub(super) distance: f32,
}

/// Builds a flat-shaded triangle mesh from a small convex point cloud.
///
/// Split visuals omit the face lying on the portal cut plane: physics needs a
/// closed convex cap, but visually that face is the open seam whose continuation
/// is rendered by the complementary manifestation.
pub(super) fn convex_polyhedron_mesh(points: &[Vec3], excluded_face: Option<LocalFacePlane>) -> Mesh {
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
                        ((normal.dot(*point) - distance).abs() <= FACE_EPSILON).then_some(index)
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

        let seed = if normal.x.abs() < 0.8 {
            Vec3::X
        } else {
            Vec3::Y
        };
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
