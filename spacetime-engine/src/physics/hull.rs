//! Canonical detailed physical box-hull description.
//!
//! A [`PhysicalBoxHull`] is authored in physical metres. It is the one body-shape
//! truth shared by collision realization, portal split geometry, support-radius
//! queries and stance changes. Backend [`Collider`]s and topology-local split
//! boxes are derived representations and must never become independent shape
//! authority.

use avian3d::prelude::Collider;
use bevy::prelude::*;

use crate::spatial::SpatialScale;

/// Authoritative detailed box hull in SI metres.
#[derive(Component, Reflect, Debug, Clone, Copy, PartialEq)]
#[reflect(Component)]
pub struct PhysicalBoxHull {
    half_extents_metres: Vec3,
}

impl PhysicalBoxHull {
    pub fn from_size_metres(size_metres: Vec3) -> Self {
        Self::from_half_extents_metres(size_metres.abs() * 0.5)
    }

    pub fn from_half_extents_metres(half_extents_metres: Vec3) -> Self {
        assert!(
            half_extents_metres.is_finite() && half_extents_metres.min_element() >= 0.0,
            "physical hull half-extents must be finite and non-negative"
        );
        Self {
            half_extents_metres,
        }
    }

    pub const fn half_extents_metres(self) -> Vec3 {
        self.half_extents_metres
    }

    pub fn size_metres(self) -> Vec3 {
        self.half_extents_metres * 2.0
    }

    pub fn half_extents_native(self, scale: SpatialScale) -> Vec3 {
        self.half_extents_metres * scale.metres_to_native_f32(1.0)
    }

    pub fn size_native(self, scale: SpatialScale) -> Vec3 {
        self.half_extents_native(scale) * 2.0
    }

    /// Conservative physical bounding radius of this semantic hull.
    pub fn bounding_radius_metres(self) -> f32 {
        self.half_extents_metres.length()
    }

    /// Scale-local conservative sphere derived from the physical hull.
    ///
    /// `clearance_metres` is physical policy. Only this backend boundary
    /// converts it into the destination Scale Slice's native units.
    pub fn bounding_sphere_collider(
        self,
        scale: SpatialScale,
        clearance_metres: f32,
    ) -> Collider {
        let radius_metres =
            self.bounding_radius_metres() + clearance_metres.max(0.0);
        Collider::sphere(
            scale
                .metres_to_native_f32(radius_metres)
                .max(f32::MIN_POSITIVE),
        )
    }

    pub fn collider(self, scale: SpatialScale) -> Collider {
        let size = self.size_native(scale);
        Collider::cuboid(size.x, size.y, size.z)
    }

    /// Physical support radius of the oriented box along a world-space axis.
    pub fn projection_radius_metres(self, rotation: Quat, world_axis: Vec3) -> f32 {
        let axis = world_axis.normalize_or_zero();
        if axis == Vec3::ZERO {
            return 0.0;
        }

        let local_axis = rotation.inverse() * axis;
        local_axis.x.abs() * self.half_extents_metres.x
            + local_axis.y.abs() * self.half_extents_metres.y
            + local_axis.z.abs() * self.half_extents_metres.z
    }
}

/// Marks that the current backend collider is the detailed collider derived
/// from [`PhysicalBoxHull`], rather than a coarse Scale-Slice proxy.
///
/// Portal body splitting consumes this fact so a proxy sphere is never silently
/// partitioned as though it were the detailed box.
#[derive(Component, Reflect, Debug, Default, Clone, Copy)]
#[reflect(Component)]
pub struct DetailedBodyCollision;
