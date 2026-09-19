//! Rectangular subtractive-stencil fitting against immutable collision sources.

use bevy::prelude::*;

use super::source::CollisionClipSource;

const UNIT_SCALE_EPSILON: f32 = 1.0e-4;
const FACE_ALIGNMENT_DOT: f32 = 0.999;

/// A nearby valid placement for a rectangular subtractive stencil.
#[derive(Debug, Clone, Copy)]
pub struct RectangularStencilFit {
    pub transform: Transform,
    /// World-space correction from the requested center, in metres.
    pub displacement: f32,
    /// Number of tangent axes magnetized/clamped to a support boundary.
    /// Two means the aperture is corner-snapped.
    pub snapped_edges: u8,
}

/// Finds the nearest valid placement of a rectangular stencil on `source`.
///
/// Cuboid faces provide natural exact axes, so the returned rectangle is also
/// roll-quantized to a quarter turn on the selected face. This removes tiny
/// orientation errors while keeping the support/fit policy generic rather than
/// portal-specific.
pub fn fit_rectangular_stencil(
    source: CollisionClipSource,
    host: &Transform,
    requested: &Transform,
    half_size: Vec2,
    plane_tolerance: f32,
    max_translation: f32,
    edge_snap_distance: f32,
) -> Option<RectangularStencilFit> {
    if (host.scale - Vec3::ONE).length_squared() > UNIT_SCALE_EPSILON * UNIT_SCALE_EPSILON {
        return None;
    }

    match source {
        CollisionClipSource::Cuboid { half_extents } => fit_rectangle_to_cuboid_face(
            half_extents,
            host,
            requested,
            half_size,
            plane_tolerance.max(0.0),
            max_translation.max(0.0),
            edge_snap_distance.max(0.0),
        ),
    }
}

/// Returns whether `stencil` lies flush on one face of `source` and its full
/// rectangular aperture fits inside that face.
///
/// This strict predicate is useful when callers do not want correction. Tools
/// that support snapping should use [`fit_rectangular_stencil`] instead.
pub fn supports_rectangular_stencil(
    source: CollisionClipSource,
    host: &Transform,
    stencil: &Transform,
    half_size: Vec2,
    plane_tolerance: f32,
) -> bool {
    if (host.scale - Vec3::ONE).length_squared() > UNIT_SCALE_EPSILON * UNIT_SCALE_EPSILON {
        return false;
    }

    match source {
        CollisionClipSource::Cuboid { half_extents } => cuboid_supports_rectangle(
            half_extents,
            host,
            stencil,
            half_size,
            plane_tolerance.max(0.0),
        ),
    }
}

fn fit_rectangle_to_cuboid_face(
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
    if normal == Vec3::ZERO || requested_right == Vec3::ZERO || requested_up == Vec3::ZERO {
        return None;
    }

    let (face_axis, alignment) = dominant_axis(normal);
    if alignment < FACE_ALIGNMENT_DOT {
        return None;
    }

    let face_extent = component(half_extents, face_axis);
    let coordinate = component(center, face_axis);
    if (coordinate.abs() - face_extent).abs() > plane_tolerance {
        return None;
    }

    let face_sign = if coordinate >= 0.0 { 1.0 } else { -1.0 };
    if component(normal, face_axis) * face_sign < FACE_ALIGNMENT_DOT {
        return None;
    }

    let exact_normal = axis_vector(face_axis) * face_sign;
    let (right, up) =
        nearest_face_quarter_turn(exact_normal, face_axis, requested_right, requested_up)?;
    set_component(&mut center, face_axis, face_sign * face_extent);

    let mut snapped_edges = 0_u8;
    for tangent_axis in 0..3 {
        if tangent_axis == face_axis {
            continue;
        }

        let extent = component(half_extents, tangent_axis);
        let radius = half_size.x * component(right, tangent_axis).abs()
            + half_size.y * component(up, tangent_axis).abs();
        if radius > extent + plane_tolerance {
            return None;
        }

        let minimum = -extent + radius;
        let maximum = extent - radius;
        if minimum > maximum + plane_tolerance {
            return None;
        }

        let requested_coordinate = component(center, tangent_axis);
        let mut fitted_coordinate = requested_coordinate.clamp(minimum, maximum);
        let was_outside = requested_coordinate < minimum || requested_coordinate > maximum;

        if !was_outside {
            let to_minimum = (requested_coordinate - minimum).abs();
            let to_maximum = (maximum - requested_coordinate).abs();
            if to_minimum.min(to_maximum) <= edge_snap_distance {
                fitted_coordinate = if to_minimum <= to_maximum {
                    minimum
                } else {
                    maximum
                };
                snapped_edges += 1;
            }
        } else {
            snapped_edges += 1;
        }

        set_component(&mut center, tangent_axis, fitted_coordinate);
    }

    let translation = host.translation + host.rotation * center;
    let displacement = translation.distance(requested.translation);
    if displacement > max_translation + plane_tolerance {
        return None;
    }

    let local_rotation = Quat::from_mat3(&Mat3::from_cols(right, up, exact_normal)).normalize();
    let rotation = (host.rotation * local_rotation).normalize();

    Some(RectangularStencilFit {
        transform: Transform::from_translation(translation).with_rotation(rotation),
        displacement,
        snapped_edges,
    })
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

fn cuboid_supports_rectangle(
    half_extents: Vec3,
    host: &Transform,
    stencil: &Transform,
    half_size: Vec2,
    plane_tolerance: f32,
) -> bool {
    let inverse_rotation = host.rotation.inverse();
    let center = inverse_rotation * (stencil.translation - host.translation);
    let normal = (inverse_rotation * (stencil.rotation * Vec3::Z)).normalize_or_zero();
    let right = (inverse_rotation * (stencil.rotation * Vec3::X)).normalize_or_zero();
    let up = (inverse_rotation * (stencil.rotation * Vec3::Y)).normalize_or_zero();
    if normal == Vec3::ZERO || right == Vec3::ZERO || up == Vec3::ZERO {
        return false;
    }

    let (face_axis, alignment) = dominant_axis(normal);
    if alignment < FACE_ALIGNMENT_DOT {
        return false;
    }

    let coordinate = component(center, face_axis);
    let face_extent = component(half_extents, face_axis);
    if (coordinate.abs() - face_extent).abs() > plane_tolerance {
        return false;
    }

    for tangent_axis in 0..3 {
        if tangent_axis == face_axis {
            continue;
        }
        let projected_radius = half_size.x * component(right, tangent_axis).abs()
            + half_size.y * component(up, tangent_axis).abs();
        if component(center, tangent_axis).abs() + projected_radius
            > component(half_extents, tangent_axis) + plane_tolerance
        {
            return false;
        }
    }

    true
}

fn dominant_axis(vector: Vec3) -> (usize, f32) {
    let absolute = vector.abs();
    if absolute.x >= absolute.y && absolute.x >= absolute.z {
        (0, absolute.x)
    } else if absolute.y >= absolute.z {
        (1, absolute.y)
    } else {
        (2, absolute.z)
    }
}

fn axis_vector(axis: usize) -> Vec3 {
    match axis {
        0 => Vec3::X,
        1 => Vec3::Y,
        _ => Vec3::Z,
    }
}

fn component(vector: Vec3, axis: usize) -> f32 {
    match axis {
        0 => vector.x,
        1 => vector.y,
        _ => vector.z,
    }
}

fn set_component(vector: &mut Vec3, axis: usize, value: f32) {
    match axis {
        0 => vector.x = value,
        1 => vector.y = value,
        _ => vector.z = value,
    }
}

#[cfg(test)]
mod tests;
