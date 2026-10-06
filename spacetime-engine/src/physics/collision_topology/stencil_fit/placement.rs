//! Corrects a rectangle onto one cuboid face and its tangent bounds.

use super::*;

struct FaceAxes {
    axis: usize,
    normal: Vec3,
    right: Vec3,
    up: Vec3,
}

pub(super) fn fit_rectangle_to_cuboid_face(
    half_extents: Vec3,
    host: &Transform,
    requested: &Transform,
    half_size: Vec2,
    plane_tolerance: f32,
    max_translation: f32,
    edge_snap_distance: f32,
) -> Option<RectangularStencilFit> {
    let inverse_rotation = host.rotation.inverse();
    let mut center = inverse_rotation * (requested.translation - host.translation);
    let normal = (inverse_rotation * (requested.rotation * Vec3::Z)).normalize_or_zero();
    let requested_right = (inverse_rotation * (requested.rotation * Vec3::X)).normalize_or_zero();
    let requested_up = (inverse_rotation * (requested.rotation * Vec3::Y)).normalize_or_zero();
    let face = aligned_face(
        half_extents,
        center,
        normal,
        requested_right,
        requested_up,
        plane_tolerance,
    )?;
    set_component(
        &mut center,
        face.axis,
        component(half_extents, face.axis) * component(face.normal, face.axis),
    );
    let snapped_edges = fit_tangents(
        &mut center,
        half_extents,
        half_size,
        &face,
        plane_tolerance,
        edge_snap_distance,
    )?;

    let translation = host.translation + host.rotation * center;
    let displacement = translation.distance(requested.translation);
    if displacement > max_translation + plane_tolerance {
        return None;
    }

    let local_rotation =
        Quat::from_mat3(&Mat3::from_cols(face.right, face.up, face.normal)).normalize();
    let rotation = (host.rotation * local_rotation).normalize();

    Some(RectangularStencilFit {
        transform: Transform::from_translation(translation).with_rotation(rotation),
        displacement,
        snapped_edges,
    })
}

/// Accept only the requested face and quantize its roll to an exact quarter turn.
fn aligned_face(
    half_extents: Vec3,
    center: Vec3,
    normal: Vec3,
    requested_right: Vec3,
    requested_up: Vec3,
    tolerance: f32,
) -> Option<FaceAxes> {
    if normal == Vec3::ZERO || requested_right == Vec3::ZERO || requested_up == Vec3::ZERO {
        return None;
    }
    let (axis, alignment) = dominant_axis(normal);
    if alignment < FACE_ALIGNMENT_DOT {
        return None;
    }
    let coordinate = component(center, axis);
    if (coordinate.abs() - component(half_extents, axis)).abs() > tolerance {
        return None;
    }
    let sign = if coordinate >= 0.0 { 1.0 } else { -1.0 };
    if component(normal, axis) * sign < FACE_ALIGNMENT_DOT {
        return None;
    }
    let exact_normal = axis_vector(axis) * sign;
    let (right, up) = nearest_face_quarter_turn(exact_normal, axis, requested_right, requested_up)?;
    Some(FaceAxes {
        axis,
        normal: exact_normal,
        right,
        up,
    })
}

fn fit_tangents(
    center: &mut Vec3,
    half_extents: Vec3,
    half_size: Vec2,
    face: &FaceAxes,
    tolerance: f32,
    snap_distance: f32,
) -> Option<u8> {
    let mut snapped = 0;
    for axis in 0..3 {
        if axis == face.axis {
            continue;
        }
        let extent = component(half_extents, axis);
        let radius = half_size.x * component(face.right, axis).abs()
            + half_size.y * component(face.up, axis).abs();
        if radius > extent + tolerance {
            return None;
        }
        let minimum = -extent + radius;
        let maximum = extent - radius;
        if minimum > maximum + tolerance {
            return None;
        }
        let (coordinate, was_snapped) =
            snap_tangent(component(*center, axis), minimum, maximum, snap_distance);
        snapped += u8::from(was_snapped);
        set_component(center, axis, coordinate);
    }
    Some(snapped)
}

fn snap_tangent(requested: f32, minimum: f32, maximum: f32, snap_distance: f32) -> (f32, bool) {
    let mut fitted = requested.clamp(minimum, maximum);
    if requested < minimum || requested > maximum {
        return (fitted, true);
    }
    let to_minimum = (requested - minimum).abs();
    let to_maximum = (maximum - requested).abs();
    if to_minimum.min(to_maximum) > snap_distance {
        return (fitted, false);
    }
    fitted = if to_minimum <= to_maximum {
        minimum
    } else {
        maximum
    };
    (fitted, true)
}

fn nearest_face_quarter_turn(
    normal: Vec3,
    face_axis: usize,
    requested_right: Vec3,
    requested_up: Vec3,
) -> Option<(Vec3, Vec3)> {
    let tangent_axes = match face_axis {
        0 => [1, 2],
        1 => [0, 2],
        _ => [0, 1],
    };
    let mut best: Option<(f32, Vec3, Vec3)> = None;

    for tangent_axis in tangent_axes {
        for sign in [-1.0_f32, 1.0] {
            let right = axis_vector(tangent_axis) * sign;
            let up = normal.cross(right).normalize_or_zero();
            if up == Vec3::ZERO {
                continue;
            }

            let score = right.dot(requested_right) + up.dot(requested_up);
            let replace = match best {
                None => true,
                Some((best_score, _, _)) => score > best_score,
            };
            if replace {
                best = Some((score, right, up));
            }
        }
    }

    best.map(|(_, right, up)| (right, up))
}
