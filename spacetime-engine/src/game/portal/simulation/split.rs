//! Narrow but real spatial manifestation splitting across portal planes.
//!
//! The reusable pieces live below the portal layer:
//! - semantic entity/manifestation relationships in `ecs::manifestation`;
//! - box/plane partitioning and query exclusions in `physics::topology`.
//!
//! This module is only the portal adapter. It currently supports one active
//! split per opted-in authoritative manifestation and uses a reserved peer
//! manifestation for the opposite side.

use std::time::Duration;

use avian3d::{
    character_controller::move_and_slide::{
        MoveAndSlide, MoveAndSlideConfig, MoveAndSlideHitResponse,
    },
    prelude::*,
};
use bevy::prelude::*;

use crate::{
    ecs::{UsfManifestationAuthority, UsfManifestationOf, UsfManifestations},
    physics::{
        character::CharacterGroundState,
        topology::{KinematicQueryExclusions, SpatialSplitBox, SplitPlane, partition_box_by_plane},
    },
};

use crate::game::portal::{
    Portal, PortalActive, PortalSplitTraveler, PortalTraveler,
    domain::{ActivePortalSplit, PortalSide},
    topology::mapping::{map_transform, portal_mapping},
};

const PREOPEN_MARGIN: f32 = 0.04;
const CLEAR_MARGIN: f32 = 0.02;
/// Fit tolerance is positive: a hull exactly tangent to an aperture edge is a
/// valid traversal configuration. The collision stencil itself adds a small
/// physical clearance to keep numerical contacts from forming a lip.
const APERTURE_FIT_TOLERANCE: f32 = 0.01;
const CROSSING_EPSILON: f32 = 1.0e-5;
const DESTINATION_REMAINDER_SUBSTEPS: usize = 4;

#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum PortalSplitSet {
    Prepare,
    MaterializeBeforeMotor,
    Resolve,
    MaterializeAfterMotor,
}

/// Predictively opens portal-host collision before the character motor runs.
///
/// We open as soon as the rigid box overlaps the plane *or can reach it during
/// this fixed tick*. That is what prevents a floor portal from contributing a
/// one-frame ground contact before traversal gets a chance to happen.
pub(crate) fn prepare_portal_splits(
    time: Res<Time<Fixed>>,
    portals: Query<
        (Entity, &Portal, &PortalActive, &Transform),
        (With<Portal>, Without<UsfManifestationAuthority>),
    >,
    semantic_entities: Query<&UsfManifestations>,
    mut travelers: Query<
        (
            Entity,
            &Transform,
            &LinearVelocity,
            &SpatialSplitBox,
            &UsfManifestationOf,
            &mut PortalSplitTraveler,
            &mut KinematicQueryExclusions,
        ),
        (With<UsfManifestationAuthority>, Without<Portal>),
    >,
) {
    let dt = time.delta_secs().max(0.0);

    for (entity, body, velocity, split_box, manifestation, mut split, mut exclusions) in
        &mut travelers
    {
        let Some(peer) = peer_manifestation(entity, manifestation, &semantic_entities) else {
            split.active = None;
            exclusions.replace(std::iter::empty::<Entity>());
            continue;
        };

        split.tick_start = *body;

        if let Some(active) = split.active {
            if !active_pair_is_valid(active, &portals)
                || !box_reaches_portal_this_tick(
                    *split_box,
                    body,
                    velocity.0,
                    dt,
                    active.source,
                    &portals,
                )
            {
                split.active = None;
            }
        }

        if split.active.is_none() {
            split.active = find_split_candidate(*split_box, body, velocity.0, dt, &portals);
        }

        // Whole-entity exclusions are now reserved for the peer manifestation.
        // Portal hosts expose an actual clipped collider instead of disappearing
        // from the character controller's query world.
        exclusions.replace([peer]);
    }
}

/// Rebuilds the two physical manifestation colliders from the same rigid box.
///
/// While the box intersects the portal plane, the authoritative manifestation
/// owns the half on its current side and the peer owns the complementary half
/// mapped through the portal. Outside the overlap interval the peer is disabled.
pub(crate) fn materialize_portal_splits(
    mut commands: Commands,
    portals: Query<
        (&Portal, &PortalActive, &Transform),
        (With<Portal>, Without<UsfManifestationAuthority>),
    >,
    semantic_entities: Query<&UsfManifestations>,
    mut authorities: Query<
        (
            Entity,
            &Transform,
            &LinearVelocity,
            &SpatialSplitBox,
            &UsfManifestationOf,
            &PortalSplitTraveler,
            &mut Collider,
        ),
        (With<UsfManifestationAuthority>, Without<Portal>),
    >,
    mut peers: Query<
        (&mut Transform, &mut LinearVelocity, &mut Collider),
        (Without<Portal>, Without<UsfManifestationAuthority>),
    >,
) {
    for (authority, body, velocity, split_box, manifestation, split, mut authority_collider) in
        &mut authorities
    {
        let Some(peer_entity) = peer_manifestation(authority, manifestation, &semantic_entities)
        else {
            continue;
        };
        let Ok((mut peer_transform, mut peer_velocity, mut peer_collider)) =
            peers.get_mut(peer_entity)
        else {
            continue;
        };

        let Some(active) = split.active else {
            *authority_collider = split_box.full_collider();
            commands.entity(peer_entity).insert(CollisionLayers::NONE);
            peer_velocity.0 = Vec3::ZERO;
            continue;
        };

        let Ok((_, source_active, source)) = portals.get(active.source) else {
            *authority_collider = split_box.full_collider();
            commands.entity(peer_entity).insert(CollisionLayers::NONE);
            continue;
        };
        let Ok((_, destination_active, destination)) = portals.get(active.destination) else {
            *authority_collider = split_box.full_collider();
            commands.entity(peer_entity).insert(CollisionLayers::NONE);
            continue;
        };
        if !source_active.0 || !destination_active.0 {
            *authority_collider = split_box.full_collider();
            commands.entity(peer_entity).insert(CollisionLayers::NONE);
            continue;
        }

        let Some(plane) = portal_plane(source) else {
            continue;
        };
        let partition = partition_box_by_plane(*split_box, body, plane);

        *peer_transform = map_transform(body, source, destination);
        peer_velocity.0 = portal_mapping(source, destination).transform_vector3(velocity.0);

        if !partition.straddles() {
            *authority_collider = split_box.full_collider();
            commands.entity(peer_entity).insert(CollisionLayers::NONE);
            continue;
        }

        let authority_positive = partition.center_distance >= 0.0;
        let (authority_half, peer_half) = if authority_positive {
            (partition.positive_collider(), partition.negative_collider())
        } else {
            (partition.negative_collider(), partition.positive_collider())
        };

        let (Some(authority_half), Some(peer_half)) = (authority_half, peer_half) else {
            *authority_collider = split_box.full_collider();
            commands.entity(peer_entity).insert(CollisionLayers::NONE);
            continue;
        };

        *authority_collider = authority_half;
        *peer_collider = peer_half;
        commands
            .entity(peer_entity)
            .insert(CollisionLayers::DEFAULT);
    }
}

/// Resolves center-plane crossings after ordinary character movement.
///
/// The authoritative body is moved exactly to the crossing, rigidly mapped,
/// and then spends the remaining fraction of the same fixed tick moving in the
/// destination space. The peer remains alive until the trailing box extent
/// clears the new source plane.
pub(crate) fn resolve_portal_splits(
    time: Res<Time<Fixed>>,
    move_and_slide: MoveAndSlide,
    portals: Query<
        (Entity, &Portal, &PortalActive, &Transform),
        (With<Portal>, Without<UsfManifestationAuthority>),
    >,
    semantic_entities: Query<&UsfManifestations>,
    mut travelers: Query<
        (
            Entity,
            &SpatialSplitBox,
            &UsfManifestationOf,
            &mut Transform,
            &mut LinearVelocity,
            &mut CharacterGroundState,
            &mut PortalTraveler,
            &mut PortalSplitTraveler,
            &mut KinematicQueryExclusions,
        ),
        (With<UsfManifestationAuthority>, Without<Portal>),
    >,
) {
    let dt = time.delta_secs().max(0.0);
    if dt <= 0.0 {
        return;
    }

    for (
        entity,
        split_box,
        manifestation,
        mut body,
        mut velocity,
        mut ground,
        mut traveler,
        mut split,
        mut exclusions,
    ) in &mut travelers
    {
        let Some(peer) = peer_manifestation(entity, manifestation, &semantic_entities) else {
            split.active = None;
            exclusions.replace(std::iter::empty::<Entity>());
            traveler.commit_position(body.translation);
            continue;
        };

        let Some(active) = split.active else {
            traveler.commit_position(body.translation);
            continue;
        };

        let Ok((_, source_portal, source_active, source)) = portals.get(active.source) else {
            split.active = None;
            exclusions.replace([peer]);
            traveler.commit_position(body.translation);
            continue;
        };
        let Ok((_, _, destination_active, destination)) = portals.get(active.destination) else {
            split.active = None;
            exclusions.replace([peer]);
            traveler.commit_position(body.translation);
            continue;
        };

        if !source_active.0 || !destination_active.0 {
            split.active = None;
            exclusions.replace([peer]);
            traveler.commit_position(body.translation);
            continue;
        }

        let start = split.tick_start.translation;
        let end = body.translation;

        if let Some((fraction, side)) = center_crossing_fraction(source, start, end) {
            if source_portal.sidedness.allows(side)
                && box_fits_aperture_at(
                    *split_box,
                    body.rotation,
                    start.lerp(end, fraction),
                    source,
                    source_portal.half_size,
                )
            {
                let crossing = Transform {
                    translation: start.lerp(end, fraction),
                    rotation: split.tick_start.rotation.slerp(body.rotation, fraction),
                    scale: Vec3::ONE,
                };
                let mapping = portal_mapping(source, destination);
                let mapped_crossing = map_transform(&crossing, source, destination);
                let mapped_velocity = mapping.transform_vector3(velocity.0);

                let filter = SpatialQueryFilter::from_excluded_entities([entity, peer]);

                let remaining = dt * (1.0 - fraction).clamp(0.0, 1.0);
                let (resolved_body, resolved_velocity) = simulate_destination_remainder(
                    &move_and_slide,
                    *split_box,
                    mapped_crossing,
                    mapped_velocity,
                    destination,
                    remaining,
                    &filter,
                );

                *body = resolved_body;
                velocity.0 = resolved_velocity;

                ground.grounded = false;
                ground.ground_entity = None;
                ground.just_landed = false;
                ground.just_left_ground = true;

                // Authority now lives in the former destination space. Keeping
                // the split active means reversing direction while still
                // straddling naturally crosses back through the same pair.
                split.active = Some(ActivePortalSplit {
                    source: active.destination,
                    destination: active.source,
                });

                exclusions.replace([peer]);
            }
        }

        if let Some(active) = split.active {
            if let Ok((_, _, _, current_source)) = portals.get(active.source) {
                if let Some(plane) = portal_plane(current_source) {
                    let radius = split_box.projection_radius(body.rotation, plane.normal);
                    if plane.signed_distance(body.translation).abs() > radius + CLEAR_MARGIN {
                        split.active = None;
                        exclusions.replace([peer]);
                    }
                }
            }
        }

        traveler.commit_position(body.translation);
    }
}

fn active_pair_is_valid(
    split: ActivePortalSplit,
    portals: &Query<
        (Entity, &Portal, &PortalActive, &Transform),
        (With<Portal>, Without<UsfManifestationAuthority>),
    >,
) -> bool {
    portals
        .get(split.source)
        .is_ok_and(|(_, _, active, _)| active.0)
        && portals
            .get(split.destination)
            .is_ok_and(|(_, _, active, _)| active.0)
}

fn find_split_candidate(
    split_box: SpatialSplitBox,
    body: &Transform,
    velocity: Vec3,
    dt: f32,
    portals: &Query<
        (Entity, &Portal, &PortalActive, &Transform),
        (With<Portal>, Without<UsfManifestationAuthority>),
    >,
) -> Option<ActivePortalSplit> {
    let mut best: Option<(f32, ActivePortalSplit)> = None;

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
        let closest = if distance * future_distance <= 0.0 {
            0.0
        } else {
            distance.abs().min(future_distance.abs())
        };

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
        let candidate = ActivePortalSplit {
            source: entity,
            destination: portal.destination,
        };
        let replace = match best {
            None => true,
            Some((best_score, _)) => score < best_score,
        };
        if replace {
            best = Some((score, candidate));
        }
    }

    best.map(|(_, candidate)| candidate)
}

fn box_reaches_portal_this_tick(
    split_box: SpatialSplitBox,
    body: &Transform,
    velocity: Vec3,
    dt: f32,
    source_entity: Entity,
    portals: &Query<
        (Entity, &Portal, &PortalActive, &Transform),
        (With<Portal>, Without<UsfManifestationAuthority>),
    >,
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
    let future = distance + velocity.dot(plane.normal) * dt;
    let radius = split_box.projection_radius(body.rotation, plane.normal);
    let closest = if distance * future <= 0.0 {
        0.0
    } else {
        distance.abs().min(future.abs())
    };

    closest <= radius + PREOPEN_MARGIN
        && box_fits_aperture_at(
            split_box,
            body.rotation,
            projected_crossing_center(body.translation, velocity, dt, plane),
            source,
            portal.half_size,
        )
}

fn portal_plane(transform: &Transform) -> Option<SplitPlane> {
    SplitPlane::new(transform.translation, transform.rotation * Vec3::Z)
}

fn candidate_side(distance: f32, normal_speed: f32, support_radius: f32) -> Option<PortalSide> {
    // If the hull already touches/straddles the portal, allow a stationary or
    // inward-moving body to open the host surface. This covers placing a floor
    // portal beneath a standing player. An outward-moving body must not
    // immediately reactivate a split that just finished collapsing.
    if distance > CROSSING_EPSILON {
        if distance <= support_radius + PREOPEN_MARGIN && normal_speed <= CROSSING_EPSILON {
            return Some(PortalSide::Front);
        }
        return None;
    }

    if distance < -CROSSING_EPSILON {
        if -distance <= support_radius + PREOPEN_MARGIN && normal_speed >= -CROSSING_EPSILON {
            return Some(PortalSide::Back);
        }
        return None;
    }

    if normal_speed < -CROSSING_EPSILON {
        Some(PortalSide::Front)
    } else if normal_speed > CROSSING_EPSILON {
        Some(PortalSide::Back)
    } else {
        None
    }
}

fn projected_crossing_center(center: Vec3, velocity: Vec3, dt: f32, plane: SplitPlane) -> Vec3 {
    let distance = plane.signed_distance(center);
    let normal_speed = velocity.dot(plane.normal);
    if normal_speed.abs() > CROSSING_EPSILON {
        let time = (-distance / normal_speed).clamp(0.0, dt);
        center + velocity * time
    } else {
        center - plane.normal * distance
    }
}

fn box_fits_aperture_at(
    split_box: SpatialSplitBox,
    body_rotation: Quat,
    center: Vec3,
    portal: &Transform,
    half_size: Vec2,
) -> bool {
    let right = portal.rotation * Vec3::X;
    let up = portal.rotation * Vec3::Y;
    let local = portal.to_matrix().inverse().transform_point3(center);
    let radius_x = split_box.projection_radius(body_rotation, right);
    let radius_y = split_box.projection_radius(body_rotation, up);

    local.x.abs() + radius_x <= half_size.x + APERTURE_FIT_TOLERANCE
        && local.y.abs() + radius_y <= half_size.y + APERTURE_FIT_TOLERANCE
}

fn center_crossing_fraction(
    portal: &Transform,
    start: Vec3,
    end: Vec3,
) -> Option<(f32, PortalSide)> {
    let plane = portal_plane(portal)?;
    let a = plane.signed_distance(start);
    let b = plane.signed_distance(end);

    let side = if a > CROSSING_EPSILON && b <= CROSSING_EPSILON {
        PortalSide::Front
    } else if a < -CROSSING_EPSILON && b >= -CROSSING_EPSILON {
        PortalSide::Back
    } else {
        return None;
    };

    let denominator = a - b;
    if denominator.abs() <= CROSSING_EPSILON {
        return None;
    }

    let fraction = a / denominator;
    (0.0..=1.0).contains(&fraction).then_some((fraction, side))
}

fn simulate_destination_remainder(
    move_and_slide: &MoveAndSlide,
    split_box: SpatialSplitBox,
    mut body: Transform,
    mut velocity: Vec3,
    destination: &Transform,
    remaining: f32,
    filter: &SpatialQueryFilter,
) -> (Transform, Vec3) {
    if remaining <= 0.0 {
        return (body, velocity);
    }

    let step = remaining / DESTINATION_REMAINDER_SUBSTEPS as f32;
    for _ in 0..DESTINATION_REMAINDER_SUBSTEPS {
        let collider = destination_half_collider(split_box, &body, destination, velocity)
            .unwrap_or_else(|| split_box.full_collider());

        let output = move_and_slide.move_and_slide(
            &collider,
            body.translation,
            body.rotation,
            velocity,
            Duration::from_secs_f32(step),
            &MoveAndSlideConfig::default(),
            filter,
            |_| MoveAndSlideHitResponse::Accept,
        );

        body.translation = output.position;
        velocity = output.projected_velocity;
    }

    (body, velocity)
}

fn destination_half_collider(
    split_box: SpatialSplitBox,
    body: &Transform,
    destination: &Transform,
    velocity: Vec3,
) -> Option<Collider> {
    let plane = portal_plane(destination)?;
    let partition = partition_box_by_plane(split_box, body, plane);
    if velocity.dot(plane.normal) >= 0.0 {
        partition.positive_collider()
    } else {
        partition.negative_collider()
    }
}

fn peer_manifestation(
    authority: Entity,
    manifestation: &UsfManifestationOf,
    semantic_entities: &Query<&UsfManifestations>,
) -> Option<Entity> {
    semantic_entities
        .get(manifestation.0)
        .ok()?
        .iter()
        .find(|entity| *entity != authority)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn center_crossing_reports_exact_fraction() {
        let portal = Transform::IDENTITY;
        let (fraction, side) =
            center_crossing_fraction(&portal, Vec3::new(0.0, 0.0, 1.0), Vec3::new(0.0, 0.0, -3.0))
                .unwrap();

        assert!((fraction - 0.25).abs() < 1.0e-6);
        assert_eq!(side, PortalSide::Front);
    }

    #[test]
    fn character_box_fits_floor_portal() {
        let split_box = SpatialSplitBox::from_size(Vec3::new(0.8128, 1.9, 0.8128));
        let floor_portal =
            Transform::IDENTITY.with_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2));

        assert!(box_fits_aperture_at(
            split_box,
            Quat::IDENTITY,
            Vec3::ZERO,
            &floor_portal,
            Vec2::new(1.25, 1.75),
        ));
    }
}
