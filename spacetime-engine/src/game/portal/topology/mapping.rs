//! Rigid source-to-destination portal mapping.

use std::f32::consts::PI;

use bevy::prelude::*;

use crate::physics::topology::SplitPlane;

/// World-space plane of a portal. Portal forward is local +Z.
pub(crate) fn portal_plane(transform: &Transform) -> Option<SplitPlane> {
    SplitPlane::new(transform.translation, transform.rotation * Vec3::Z)
}

/// World-space mapping through a portal pair.
///
/// 1. source world -> source-local;
/// 2. half-turn around local Y;
/// 3. destination-local -> destination world.
///
/// Because portal transforms are constrained to unit scale, this is a rigid
/// mapping and supports arbitrary translation + 3D rotation.
pub(crate) fn portal_mapping(source: &Transform, destination: &Transform) -> Mat4 {
    destination.to_matrix() * Mat4::from_rotation_y(PI) * source.to_matrix().inverse()
}

pub(crate) fn map_transform(
    transform: &Transform,
    source: &Transform,
    destination: &Transform,
) -> Transform {
    Transform::from_matrix(portal_mapping(source, destination) * transform.to_matrix())
}
