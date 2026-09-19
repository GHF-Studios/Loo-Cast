//! Immutable collision geometry that can be rebuilt after topology changes.

use avian3d::prelude::Collider;
use bevy::prelude::*;

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
