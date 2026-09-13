//! Segment/plane/aperture crossing tests.

use bevy::prelude::*;

use crate::game::portal::domain::{PortalSide, PortalSidedness};

/// Returns the side crossed by a world-space motion segment.
///
/// The entire test happens in portal-local coordinates, so portal orientation
/// in world space is irrelevant.
pub(crate) fn crossed_aperture(
    portal_transform: &Transform,
    half_size: Vec2,
    sidedness: PortalSidedness,
    previous_world: Vec3,
    current_world: Vec3,
) -> Option<PortalSide> {
    aperture_crossing(portal_transform, half_size, sidedness, previous_world, current_world)
        .map(|(_, side)| side)
}

pub(crate) fn crossed_aperture_fraction(
    portal_transform: &Transform,
    half_size: Vec2,
    sidedness: PortalSidedness,
    previous_world: Vec3,
    current_world: Vec3,
) -> Option<f32> {
    aperture_crossing(portal_transform, half_size, sidedness, previous_world, current_world)
        .map(|(fraction, _)| fraction)
}

fn aperture_crossing(
    portal_transform: &Transform,
    half_size: Vec2,
    sidedness: PortalSidedness,
    previous_world: Vec3,
    current_world: Vec3,
) -> Option<(f32, PortalSide)> {
    let world_to_portal = portal_transform.to_matrix().inverse();

    let previous = world_to_portal.transform_point3(previous_world);

    let current = world_to_portal.transform_point3(current_world);

    let side = if previous.z > 0.0 && current.z <= 0.0 {
        PortalSide::Front
    } else if previous.z < 0.0 && current.z >= 0.0 {
        PortalSide::Back
    } else {
        return None;
    };

    if !sidedness.allows(side) {
        return None;
    }

    let denominator = previous.z - current.z;

    if denominator.abs() <= f32::EPSILON {
        return None;
    }

    let fraction = previous.z / denominator;

    if !(0.0..=1.0).contains(&fraction) {
        return None;
    }

    let crossing = previous.lerp(current, fraction);

    if crossing.x.abs() > half_size.x || crossing.y.abs() > half_size.y {
        return None;
    }

    Some((fraction, side))
}
