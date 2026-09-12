//! Immutable collision geometry that can be rebuilt after topology changes.

use avian3d::prelude::Collider;
use bevy::prelude::*;

const UNIT_SCALE_EPSILON: f32 = 1.0e-4;

/// Immutable source geometry for a collider that may be modified at runtime.
///
/// More source shapes can be added without changing stencil producers. Cuboids
/// cover authored portal surfaces today and establish the extension point for
/// convex/trimesh sources later.
#[derive(Component, Debug, Clone, Copy)]
pub enum CollisionClipSource {
    Cuboid { half_extents: Vec3 },
}

impl CollisionClipSource {
    pub fn cuboid(size: Vec3) -> Self {
        Self::Cuboid {
            half_extents: size * 0.5,
        }
    }

    pub fn original_collider(self) -> Collider {
        match self {
            Self::Cuboid { half_extents } => Collider::cuboid(
                half_extents.x * 2.0,
                half_extents.y * 2.0,
                half_extents.z * 2.0,
            ),
        }
    }
}

/// Returns whether `stencil` lies flush on one face of `source` and its full
/// rectangular aperture fits inside that face.
///
/// This is physics-owned placement geometry: portals are merely the first
/// caller, and future topology tools can reuse the same support test.
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

    let absolute = normal.abs();
    let (face_axis, alignment) = if absolute.x >= absolute.y && absolute.x >= absolute.z {
        (0, absolute.x)
    } else if absolute.y >= absolute.z {
        (1, absolute.y)
    } else {
        (2, absolute.z)
    };
    if alignment < 0.999 {
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

fn component(vector: Vec3, axis: usize) -> f32 {
    match axis {
        0 => vector.x,
        1 => vector.y,
        _ => vector.z,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn floor_aligned_rectangle_can_touch_cuboid_face_edge() {
        let source = CollisionClipSource::cuboid(Vec3::new(5.2, 5.4, 0.4));
        let host = Transform::from_xyz(0.0, 2.7, 0.0);
        let stencil = Transform::from_xyz(0.0, 1.75, -0.2)
            .with_rotation(Quat::from_rotation_y(std::f32::consts::PI));

        assert!(supports_rectangular_stencil(
            source,
            &host,
            &stencil,
            Vec2::new(1.25, 1.75),
            1.0e-3,
        ));
    }
}
