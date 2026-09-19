//! Portal placement geometry and support resolution.
//!
//! Tools propose rigid portal transforms. This module decides whether a proposal
//! is physically meaningful and which clip-capable collider owns the surface.
//! Snapping, support fitting and overlap policy stay here so tools remain
//! decoupled from collision reconstruction and traversal.

use bevy::prelude::*;

use crate::{
    portal::Portal,
    physics::collision_topology::{CollisionClipSource, fit_rectangular_stencil},
};

const SUPPORT_PLANE_TOLERANCE: f32 = 0.025;
const MAX_POSITION_SNAP_DISTANCE: f32 = 1.50;
const EDGE_MAGNET_DISTANCE: f32 = 0.50;
const OVERLAP_PLANE_EPSILON: f32 = 0.01;
const OVERLAP_NORMAL_DOT: f32 = 0.999;
const OVERLAP_EDGE_EPSILON: f32 = 1.0e-4;

#[derive(Debug, Clone, Copy)]
pub(crate) struct PortalPlacement {
    pub transform: Transform,
    pub support: Entity,
}

/// Resolves one proposed portal transform against immutable clip-capable source
/// geometry. Nearby positions are corrected onto the nearest valid face region;
/// positions already close to a face edge magnetize to that edge (or corner).
/// Cuboid support also returns an exact face-axis quarter-turn orientation.
/// Multi-host coplanar support can be added here later.
pub(crate) fn resolve_portal_placement(
    transform: Transform,
    half_size: Vec2,
    hosts: &Query<(Entity, &CollisionClipSource, &Transform), Without<Portal>>,
) -> Option<PortalPlacement> {
    hosts
        .iter()
        .filter_map(|(support, source, host)| {
            fit_rectangular_stencil(
                *source,
                host,
                &transform,
                half_size,
                SUPPORT_PLANE_TOLERANCE,
                MAX_POSITION_SNAP_DISTANCE,
                EDGE_MAGNET_DISTANCE,
            )
            .map(|fit| (support, fit))
        })
        .min_by(|(_, a), (_, b)| {
            a.displacement
                .total_cmp(&b.displacement)
                .then_with(|| b.snapped_edges.cmp(&a.snapped_edges))
        })
        .map(|(support, fit)| PortalPlacement {
            transform: fit.transform,
            support,
        })
}

/// Returns whether two effectively coplanar portal apertures overlap.
/// Touching edges are allowed; positive-area overlap is rejected.
pub(crate) fn coplanar_apertures_overlap(
    first: &Transform,
    first_half_size: Vec2,
    second: &Transform,
    second_half_size: Vec2,
) -> bool {
    let first_normal = (first.rotation * Vec3::Z).normalize_or_zero();
    let second_normal = (second.rotation * Vec3::Z).normalize_or_zero();
    if first_normal == Vec3::ZERO || second_normal == Vec3::ZERO {
        return false;
    }

    if first_normal.dot(second_normal).abs() < OVERLAP_NORMAL_DOT {
        return false;
    }

    let center_delta = second.translation - first.translation;
    if center_delta.dot(first_normal).abs() > OVERLAP_PLANE_EPSILON {
        return false;
    }

    let first_right = first.rotation * Vec3::X;
    let first_up = first.rotation * Vec3::Y;
    let second_right = second.rotation * Vec3::X;
    let second_up = second.rotation * Vec3::Y;

    [first_right, first_up, second_right, second_up]
        .into_iter()
        .all(|axis| {
            let center_distance = center_delta.dot(axis).abs();
            let first_radius = first_half_size.x * first_right.dot(axis).abs()
                + first_half_size.y * first_up.dot(axis).abs();
            let second_radius = second_half_size.x * second_right.dot(axis).abs()
                + second_half_size.y * second_up.dot(axis).abs();

            center_distance < first_radius + second_radius - OVERLAP_EDGE_EPSILON
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn coplanar_portals_reject_overlap_but_allow_touching() {
        let first = Transform::default();
        let overlapping = Transform::from_xyz(1.0, 0.0, 0.0);
        let touching = Transform::from_xyz(2.5, 0.0, 0.0);
        let half_size = Vec2::new(1.25, 1.75);

        assert!(coplanar_apertures_overlap(
            &first,
            half_size,
            &overlapping,
            half_size,
        ));
        assert!(!coplanar_apertures_overlap(
            &first, half_size, &touching, half_size,
        ));
    }
}
