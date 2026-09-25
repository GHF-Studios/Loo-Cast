//! Active-pair validity and predictive split candidate selection.

use bevy::prelude::*;

use crate::{
    portal::{
        Portal, PortalActive,
        domain::{ActivePortalSplit, PortalSide},
        topology::mapping::portal_plane,
    },
    physics::topology::SpatialSplitBox,
};

use super::aperture::{
    box_fits_aperture_at, projected_crossing_center, CROSSING_EPSILON,
};

const PREOPEN_MARGIN: f32 = 0.04;

pub(in crate::portal::simulation) fn active_pair_is_valid(
    split: ActivePortalSplit,
    portals: &Query<(Entity, &Portal, &PortalActive, &Transform), With<Portal>>,
) -> bool {
    portals
        .get(split.source)
        .is_ok_and(|(_, _, active, _)| active.0)
        && portals
            .get(split.destination)
            .is_ok_and(|(_, _, active, _)| active.0)
}

pub(in crate::portal::simulation) fn find_split_candidate(
    split_box: SpatialSplitBox,
    body: &Transform,
    velocity: Vec3,
    dt: f32,
    portals: &Query<(Entity, &Portal, &PortalActive, &Transform), With<Portal>>,
) -> Option<(Entity, Entity)> {
    let mut best: Option<(f32, Entity, Entity)> = None;

    for (entity, portal, active, source) in portals {
        if !active.0 {
            continue;
        }
        let Ok((_, _, destination_active, _)) = portals.get(portal.destination) else {
            continue;
        };
        if !destination_active.0 {
            continue;
        }

        let Some(plane) = portal_plane(source) else {
            continue;
        };
        let distance = plane.signed_distance(body.translation);
        let normal_speed = velocity.dot(plane.normal);
        let radius = split_box.projection_radius(body.rotation, plane.normal);
        let Some(side) = candidate_side(distance, normal_speed, radius) else {
            continue;
        };
        if !portal.sidedness.allows(side) {
            continue;
        }

        let future_distance = distance + normal_speed * dt;
        let closest = closest_plane_distance(distance, future_distance);
        if closest > radius + PREOPEN_MARGIN {
            continue;
        }

        let crossing_center = projected_crossing_center(body.translation, velocity, dt, plane);
        if !box_fits_aperture_at(
            split_box,
            body.rotation,
            crossing_center,
            source,
            portal.half_size,
        ) {
            continue;
        }

        let score = (distance.abs() - radius).max(0.0);
        let replace = match best {
            None => true,
            Some((best_score, _, _)) => score < best_score,
        };
        if replace {
            best = Some((score, entity, portal.destination));
        }
    }

    best.map(|(_, source, destination)| (source, destination))
}

pub(in crate::portal::simulation) fn box_reaches_portal_this_tick(
    split_box: SpatialSplitBox,
    body: &Transform,
    velocity: Vec3,
    dt: f32,
    source_entity: Entity,
    portals: &Query<(Entity, &Portal, &PortalActive, &Transform), With<Portal>>,
) -> bool {
    let Ok((_, portal, active, source)) = portals.get(source_entity) else {
        return false;
    };
    if !active.0 {
        return false;
    }
    let Some(plane) = portal_plane(source) else {
        return false;
    };

    let distance = plane.signed_distance(body.translation);
    let future_distance = distance + velocity.dot(plane.normal) * dt;
    let radius = split_box.projection_radius(body.rotation, plane.normal);

    closest_plane_distance(distance, future_distance) <= radius + PREOPEN_MARGIN
        && box_fits_aperture_at(
            split_box,
            body.rotation,
            projected_crossing_center(body.translation, velocity, dt, plane),
            source,
            portal.half_size,
        )
}

fn candidate_side(
    distance: f32,
    normal_speed: f32,
    support_radius: f32,
) -> Option<PortalSide> {
    let epsilon = CROSSING_EPSILON;

    // If the hull already touches/straddles the portal, allow a stationary or
    // inward-moving body to open the host surface. This covers placing a floor
    // portal beneath a standing player. An outward-moving body must not
    // immediately reactivate a split that just finished collapsing.
    if distance > epsilon {
        if distance <= support_radius + PREOPEN_MARGIN && normal_speed <= epsilon {
            return Some(PortalSide::Front);
        }
        return None;
    }

    if distance < -epsilon {
        if -distance <= support_radius + PREOPEN_MARGIN && normal_speed >= -epsilon {
            return Some(PortalSide::Back);
        }
        return None;
    }

    if normal_speed < -epsilon {
        Some(PortalSide::Front)
    } else if normal_speed > epsilon {
        Some(PortalSide::Back)
    } else {
        None
    }
}

fn closest_plane_distance(start: f32, end: f32) -> f32 {
    if start * end <= 0.0 {
        0.0
    } else {
        start.abs().min(end.abs())
    }
}
