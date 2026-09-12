//! Pure convex clipping/decomposition used by collision topology.

use avian3d::prelude::Collider;
use bevy::prelude::*;

const PLANE_EPSILON: f32 = 1.0e-5;
const UNIT_SCALE_EPSILON: f32 = 1.0e-4;

#[derive(Debug, Clone, Copy)]
pub(super) struct RectangularCut {
    pub transform: Transform,
    pub half_size: Vec2,
    pub clearance: f32,
}

/// Subtracts world-space rectangular prisms from one local-space cuboid and
/// returns an Avian compound of the remaining convex pieces.
pub(super) fn subtract_rectangular_cuts_from_cuboid(
    half_extents: Vec3,
    host: &Transform,
    cuts: impl IntoIterator<Item = RectangularCut>,
) -> Option<Collider> {
    if (host.scale - Vec3::ONE).length_squared() > UNIT_SCALE_EPSILON * UNIT_SCALE_EPSILON {
        return None;
    }

    let mut pieces = vec![ConvexPolyhedron::cuboid(half_extents)];

    for cut in cuts {
        let planes = cut_planes_in_host_space(half_extents, host, cut)?;
        pieces = subtract_convex_volume(pieces, &planes);
        if pieces.is_empty() {
            return None;
        }
    }

    let mut colliders = Vec::with_capacity(pieces.len());
    for piece in pieces {
        let points = piece.unique_points();
        if points.len() < 4 {
            continue;
        }
        if let Some(collider) = Collider::convex_hull(points) {
            colliders.push(collider);
        }
    }

    match colliders.len() {
        0 => None,
        1 => colliders.pop(),
        _ => Some(Collider::compound(
            colliders
                .into_iter()
                .map(|collider| (Vec3::ZERO, Quat::IDENTITY, collider))
                .collect(),
        )),
    }
}

fn cut_planes_in_host_space(
    host_half_extents: Vec3,
    host: &Transform,
    cut: RectangularCut,
) -> Option<[ClipPlane; 6]> {
    let inverse_rotation = host.rotation.inverse();
    let center = inverse_rotation * (cut.transform.translation - host.translation);
    let right = (inverse_rotation * (cut.transform.rotation * Vec3::X)).normalize_or_zero();
    let up = (inverse_rotation * (cut.transform.rotation * Vec3::Y)).normalize_or_zero();
    let normal = (inverse_rotation * (cut.transform.rotation * Vec3::Z)).normalize_or_zero();
    if right == Vec3::ZERO || up == Vec3::ZERO || normal == Vec3::ZERO {
        return None;
    }

    let half_x = cut.half_size.x + cut.clearance;
    let half_y = cut.half_size.y + cut.clearance;
    // The cut plane sits on a host surface. A depth larger than the host's
    // diagonal guarantees that the subtractive prism reaches through it.
    let half_depth = host_half_extents.length() * 2.0 + 1.0;

    Some([
        ClipPlane::inside(center, right, half_x),
        ClipPlane::inside(center, -right, half_x),
        ClipPlane::inside(center, up, half_y),
        ClipPlane::inside(center, -up, half_y),
        ClipPlane::inside(center, normal, half_depth),
        ClipPlane::inside(center, -normal, half_depth),
    ])
}

/// Subtracts one convex clipping volume from a set of convex pieces.
///
/// For each clipping plane, the outside half is finalized while the inside half
/// continues to the next plane. What remains inside every plane at the end is
/// exactly the volume being removed.
fn subtract_convex_volume(
    pieces: Vec<ConvexPolyhedron>,
    clip_planes: &[ClipPlane],
) -> Vec<ConvexPolyhedron> {
    let mut candidates = pieces;
    let mut kept = Vec::new();

    for &plane in clip_planes {
        let mut next_candidates = Vec::new();
        for piece in candidates {
            let (inside, outside) = piece.split(plane);
            if let Some(outside) = outside {
                kept.push(outside);
            }
            if let Some(inside) = inside {
                next_candidates.push(inside);
            }
        }
        candidates = next_candidates;
        if candidates.is_empty() {
            break;
        }
    }

    kept
}

#[derive(Debug, Clone, Copy)]
struct ClipPlane {
    normal: Vec3,
    offset: f32,
}

impl ClipPlane {
    /// Plane whose inside half-space satisfies `signed_distance <= 0`.
    fn inside(center: Vec3, normal: Vec3, extent: f32) -> Self {
        Self {
            normal,
            offset: center.dot(normal) + extent,
        }
    }

    fn signed_distance(self, point: Vec3) -> f32 {
        point.dot(self.normal) - self.offset
    }
}

#[derive(Debug, Clone)]
struct ConvexPolyhedron {
    faces: Vec<Vec<Vec3>>,
}

impl ConvexPolyhedron {
    fn cuboid(h: Vec3) -> Self {
        let p = [
            Vec3::new(-h.x, -h.y, -h.z),
            Vec3::new(h.x, -h.y, -h.z),
            Vec3::new(h.x, h.y, -h.z),
            Vec3::new(-h.x, h.y, -h.z),
            Vec3::new(-h.x, -h.y, h.z),
            Vec3::new(h.x, -h.y, h.z),
            Vec3::new(h.x, h.y, h.z),
            Vec3::new(-h.x, h.y, h.z),
        ];
        Self {
            faces: vec![
                vec![p[0], p[1], p[2], p[3]],
                vec![p[4], p[7], p[6], p[5]],
                vec![p[0], p[4], p[5], p[1]],
                vec![p[3], p[2], p[6], p[7]],
                vec![p[0], p[3], p[7], p[4]],
                vec![p[1], p[5], p[6], p[2]],
            ],
        }
    }

    /// Returns `(inside, outside)` relative to `plane`.
    fn split(self, plane: ClipPlane) -> (Option<Self>, Option<Self>) {
        let mut inside_faces = Vec::new();
        let mut outside_faces = Vec::new();
        let mut intersections = Vec::new();

        for face in &self.faces {
            let (inside, crossings_a) = clip_polygon(face, plane, true);
            let (outside, crossings_b) = clip_polygon(face, plane, false);
            if inside.len() >= 3 {
                inside_faces.push(inside);
            }
            if outside.len() >= 3 {
                outside_faces.push(outside);
            }
            for point in crossings_a.into_iter().chain(crossings_b) {
                push_unique(&mut intersections, point);
            }
        }

        if intersections.len() >= 3 {
            let cap = sorted_cap(intersections, plane.normal);
            inside_faces.push(cap.clone());
            outside_faces.push(cap);
        }

        (
            (!inside_faces.is_empty()).then_some(Self { faces: inside_faces }),
            (!outside_faces.is_empty()).then_some(Self { faces: outside_faces }),
        )
    }

    fn unique_points(&self) -> Vec<Vec3> {
        let mut points = Vec::new();
        for face in &self.faces {
            for &point in face {
                push_unique(&mut points, point);
            }
        }
        points
    }
}

fn clip_polygon(face: &[Vec3], plane: ClipPlane, keep_inside: bool) -> (Vec<Vec3>, Vec<Vec3>) {
    let mut output = Vec::new();
    let mut crossings = Vec::new();
    if face.is_empty() {
        return (output, crossings);
    }

    for index in 0..face.len() {
        let current = face[index];
        let next = face[(index + 1) % face.len()];
        let current_distance = plane.signed_distance(current);
        let next_distance = plane.signed_distance(next);
        let current_kept = if keep_inside {
            current_distance <= PLANE_EPSILON
        } else {
            current_distance >= -PLANE_EPSILON
        };
        let next_kept = if keep_inside {
            next_distance <= PLANE_EPSILON
        } else {
            next_distance >= -PLANE_EPSILON
        };

        if current_kept {
            push_unique(&mut output, current);
        }

        if current_kept != next_kept {
            let denominator = current_distance - next_distance;
            if denominator.abs() > PLANE_EPSILON {
                let fraction = (current_distance / denominator).clamp(0.0, 1.0);
                let intersection = current.lerp(next, fraction);
                push_unique(&mut output, intersection);
                push_unique(&mut crossings, intersection);
            }
        }
    }

    (output, crossings)
}

fn sorted_cap(mut points: Vec<Vec3>, normal: Vec3) -> Vec<Vec3> {
    let center = points.iter().copied().sum::<Vec3>() / points.len() as f32;
    let reference = if normal.x.abs() < 0.8 { Vec3::X } else { Vec3::Y };
    let tangent = normal.cross(reference).normalize_or_zero();
    let bitangent = normal.cross(tangent).normalize_or_zero();

    points.sort_by(|a, b| {
        let da = *a - center;
        let db = *b - center;
        let aa = da.dot(bitangent).atan2(da.dot(tangent));
        let ab = db.dot(bitangent).atan2(db.dot(tangent));
        aa.total_cmp(&ab)
    });
    points
}

fn push_unique(points: &mut Vec<Vec3>, point: Vec3) {
    if points
        .iter()
        .all(|existing| existing.distance_squared(point) > 1.0e-10)
    {
        points.push(point);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn subtracting_center_prism_leaves_multiple_convex_pieces() {
        let host = Transform::IDENTITY;
        let cut = RectangularCut {
            transform: Transform::from_xyz(0.0, 0.0, 1.0),
            half_size: Vec2::new(0.5, 0.75),
            clearance: 0.0,
        };
        let planes = cut_planes_in_host_space(Vec3::ONE, &host, cut).unwrap();
        let pieces = subtract_convex_volume(vec![ConvexPolyhedron::cuboid(Vec3::ONE)], &planes);

        assert!(pieces.len() >= 3);
        assert!(pieces.iter().all(|piece| piece.unique_points().len() >= 4));
    }
}
