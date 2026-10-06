//! Strict uncorrected support predicate for a rectangular aperture.

use super::*;

pub(super) fn cuboid_supports_rectangle(
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
