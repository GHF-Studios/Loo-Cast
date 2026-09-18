//! Portal splitting for ordinary dynamic rigid bodies.
//!
//! One authoritative rigid body owns mass, inertia, gravity and external forces.
//! While it straddles a portal, a reserved dynamic peer exposes the mapped half
//! of its collider to Avian. The peer uses custom velocity integration and is
//! coupled to the authority inside Avian's substep loop: canonical continuous
//! forces are mapped into the peer before constraint solving, and peer-side
//! solver impulses are inverse-mapped back into the authority after each
//! substep. Both halves therefore participate as one effective rigid body.

use avian3d::{dynamics::solver::solver_body::SolverBody, prelude::*};
use bevy::prelude::*;

use crate::{
    game::portal::{
        Portal, PortalActive, PortalRigidSplitBody, PortalSplitTraveler, PortalTraveler,
        domain::ActivePortalSplit,
        topology::mapping::{map_transform, portal_mapping, portal_plane},
    },
    physics::{
        character::{
            CharacterMotor, CharacterPush, MAX_DYNAMIC_CONTACT_DELTA_SPEED,
            dynamic_contact_delta_velocity,
        },
        topology::{
            SpatialSplitBox, SpatialSplitPeer, SpatialSplitPeerActive, partition_box_by_plane,
        },
    },
};

use super::split::{
    box_fits_aperture_at, candidate_side, center_crossing_fraction, projected_crossing_center,
};

const PREOPEN_MARGIN: f32 = 0.04;
const CLEAR_MARGIN: f32 = 0.02;

/// Maps character impulses that strike the destination-side proxy back onto the
/// one authoritative rigid body before portal peers are synchronized for this
/// physics step.
///
/// The generic character system handles ordinary dynamic bodies directly and
/// deliberately skips [`SpatialSplitPeer`] entities. This adapter is the only
/// portal-aware part: it inverse-maps the world-space impulse and contact point
/// into the authority's current portal space, preserving both linear and torque
/// response on the canonical body.
pub(crate) fn apply_peer_character_pushes(
    mut pushes: MessageReader<CharacterPush>,
    portals: Query<(Entity, &Portal, &PortalActive, &Transform), With<Portal>>,
    peers: Query<&SpatialSplitPeer, With<SpatialSplitPeerActive>>,
    mut authorities: Query<
        (
            &PortalSplitTraveler,
            &PortalRigidSplitBody,
            Forces,
            &RigidBody,
        ),
        Without<SpatialSplitPeer>,
    >,
) {
    for push in pushes.read() {
        let Ok(peer) = peers.get(push.target) else {
            continue;
        };
        let Ok((split, rigid_split, mut forces, body)) = authorities.get_mut(peer.authority) else {
            continue;
        };
        if *body != RigidBody::Dynamic
            || !rigid_split.peer_solver_active
            || split.peer() != push.target
        {
            continue;
        }

        let Some(active) = split.active else {
            continue;
        };
        let Ok((_, _, source_active, source)) = portals.get(active.source) else {
            continue;
        };
        let Ok((_, _, destination_active, destination)) = portals.get(active.destination) else {
            continue;
        };
        if !source_active.0 || !destination_active.0 {
            continue;
        }

        let inverse_mapping = portal_mapping(destination, source);
        let impulse = inverse_mapping.transform_vector3(push.impulse);
        let point = inverse_mapping.transform_point3(push.point);
        forces.apply_linear_impulse_at_point(impulse, point);
    }
}

/// Maps dynamic-body contact impulses from an active character split peer back
/// into the character authority after Avian has solved the physics step.
///
/// The kinematic character authority cannot receive solver impulses directly.
/// Its destination-side proxy can still participate in contact generation, so
/// we read the real normal impulses, inverse-map their reaction direction into
/// source space, and convert them into the same velocity response used for
/// contacts on the authority half.
pub(crate) fn receive_peer_dynamic_contact_pushes(
    collisions: Collisions,
    portals: Query<(Entity, &Portal, &PortalActive, &Transform), With<Portal>>,
    peers: Query<(Entity, &SpatialSplitPeer), (With<SpatialSplitPeerActive>, Without<Portal>)>,
    bodies: Query<&RigidBody>,
    mut authorities: Query<
        (&PortalSplitTraveler, &mut LinearVelocity),
        (
            With<CharacterMotor>,
            Without<SpatialSplitPeer>,
            Without<Portal>,
        ),
    >,
) {
    for (peer_entity, peer) in &peers {
        let Ok((split, mut velocity)) = authorities.get_mut(peer.authority) else {
            continue;
        };
        if split.peer() != peer_entity {
            continue;
        }

        let Some(active) = split.active else {
            continue;
        };
        let Ok((_, _, source_active, source)) = portals.get(active.source) else {
            continue;
        };
        let Ok((_, _, destination_active, destination)) = portals.get(active.destination) else {
            continue;
        };
        if !source_active.0 || !destination_active.0 {
            continue;
        }

        let inverse_mapping = portal_mapping(destination, source);
        let mut delta_velocity = Vec3::ZERO;

        for pair in collisions.collisions_with(peer_entity) {
            let peer_is_first = if pair.body1 == Some(peer_entity) || pair.collider1 == peer_entity
            {
                true
            } else if pair.body2 == Some(peer_entity) || pair.collider2 == peer_entity {
                false
            } else {
                continue;
            };

            let other_body = if peer_is_first {
                pair.body2
            } else {
                pair.body1
            };
            let Some(other_body) = other_body else {
                continue;
            };
            let Ok(other_kind) = bodies.get(other_body) else {
                continue;
            };
            if *other_kind != RigidBody::Dynamic {
                continue;
            }

            for manifold in &pair.manifolds {
                let impulse = manifold.total_normal_impulse().abs();
                if impulse <= 0.0 {
                    continue;
                }

                let destination_reaction = if peer_is_first {
                    -manifold.normal
                } else {
                    manifold.normal
                };
                let source_reaction = inverse_mapping
                    .transform_vector3(destination_reaction)
                    .normalize_or_zero();
                if source_reaction == Vec3::ZERO {
                    continue;
                }

                delta_velocity += dynamic_contact_delta_velocity(source_reaction, impulse);
            }
        }

        velocity.0 += delta_velocity.clamp_length_max(MAX_DYNAMIC_CONTACT_DELTA_SPEED);
    }
}

pub(crate) fn prepare_rigid_splits(
    time: Res<Time<Fixed>>,
    mut commands: Commands,
    portals: Query<(Entity, &Portal, &PortalActive, &Transform), With<Portal>>,
    mut authorities: Query<
        (
            Entity,
            &Transform,
            &LinearVelocity,
            &AngularVelocity,
            &SpatialSplitBox,
            &mut PortalSplitTraveler,
            &mut PortalRigidSplitBody,
            &mut Collider,
        ),
        (
            Without<SpatialSplitPeer>,
            Without<Portal>,
            Without<CharacterMotor>,
        ),
    >,
    mut peers: Query<
        (
            &mut Transform,
            &mut LinearVelocity,
            &mut AngularVelocity,
            &mut Collider,
        ),
        (With<SpatialSplitPeer>, Without<Portal>),
    >,
) {
    let dt = time.delta_secs().max(0.0);

    for (
        _entity,
        body,
        velocity,
        angular_velocity,
        split_box,
        mut split,
        mut rigid_split,
        mut authority_collider,
    ) in &mut authorities
    {
        let peer_entity = split.peer();
        let Ok((mut peer_transform, mut peer_velocity, mut peer_angular, mut peer_collider)) =
            peers.get_mut(peer_entity)
        else {
            split.active = None;
            *authority_collider = split_box.full_collider();
            rigid_split.peer_solver_active = false;
            continue;
        };

        if let Some(active) = split.active {
            if !active_pair_is_valid(active, &portals)
                || !box_reaches_portal_this_tick(
                    *split_box,
                    &body,
                    velocity.0,
                    dt,
                    active.source,
                    &portals,
                )
            {
                split.active = None;
            }
        }

        split.tick_start = *body;
        if split.active.is_none() {
            split.active = find_split_candidate(*split_box, &body, velocity.0, dt, &portals);
        }

        let Some(active) = split.active else {
            deactivate_peer(
                &mut commands,
                peer_entity,
                *split_box,
                &mut authority_collider,
                &mut peer_velocity,
                &mut peer_angular,
                &mut rigid_split,
            );
            continue;
        };

        let Ok((_, _, source_active, source)) = portals.get(active.source) else {
            split.active = None;
            deactivate_peer(
                &mut commands,
                peer_entity,
                *split_box,
                &mut authority_collider,
                &mut peer_velocity,
                &mut peer_angular,
                &mut rigid_split,
            );
            continue;
        };
        let Ok((_, _, destination_active, destination)) = portals.get(active.destination) else {
            split.active = None;
            deactivate_peer(
                &mut commands,
                peer_entity,
                *split_box,
                &mut authority_collider,
                &mut peer_velocity,
                &mut peer_angular,
                &mut rigid_split,
            );
            continue;
        };
        if !source_active.0 || !destination_active.0 {
            split.active = None;
            deactivate_peer(
                &mut commands,
                peer_entity,
                *split_box,
                &mut authority_collider,
                &mut peer_velocity,
                &mut peer_angular,
                &mut rigid_split,
            );
            continue;
        }

        let Some(plane) = portal_plane(source) else {
            split.active = None;
            deactivate_peer(
                &mut commands,
                peer_entity,
                *split_box,
                &mut authority_collider,
                &mut peer_velocity,
                &mut peer_angular,
                &mut rigid_split,
            );
            continue;
        };
        let partition = partition_box_by_plane(*split_box, &body, plane);
        let mapping = portal_mapping(source, destination);

        *peer_transform = map_transform(&body, source, destination);
        peer_velocity.0 = mapping.transform_vector3(velocity.0);
        peer_angular.0 = mapping.transform_vector3(angular_velocity.0);
        rigid_split.peer_baseline_linear = peer_velocity.0;
        rigid_split.peer_baseline_angular = peer_angular.0;

        if !partition.straddles() {
            deactivate_peer(
                &mut commands,
                peer_entity,
                *split_box,
                &mut authority_collider,
                &mut peer_velocity,
                &mut peer_angular,
                &mut rigid_split,
            );
            continue;
        }

        let authority_positive = partition.center_distance >= 0.0;
        let (authority_half, peer_half) = if authority_positive {
            (partition.positive_collider(), partition.negative_collider())
        } else {
            (partition.negative_collider(), partition.positive_collider())
        };

        let (Some(authority_half), Some(peer_half)) = (authority_half, peer_half) else {
            deactivate_peer(
                &mut commands,
                peer_entity,
                *split_box,
                &mut authority_collider,
                &mut peer_velocity,
                &mut peer_angular,
                &mut rigid_split,
            );
            continue;
        };

        *authority_collider = authority_half;
        *peer_collider = peer_half;
        rigid_split.peer_solver_active = true;
        commands
            .entity(peer_entity)
            .insert((CollisionLayers::DEFAULT, SpatialSplitPeerActive));
    }
}

fn deactivate_peer(
    commands: &mut Commands,
    peer: Entity,
    split_box: SpatialSplitBox,
    authority_collider: &mut Collider,
    peer_velocity: &mut LinearVelocity,
    peer_angular: &mut AngularVelocity,
    rigid_split: &mut PortalRigidSplitBody,
) {
    *authority_collider = split_box.full_collider();
    peer_velocity.0 = Vec3::ZERO;
    peer_angular.0 = Vec3::ZERO;
    rigid_split.peer_solver_active = false;
    commands
        .entity(peer)
        .insert(CollisionLayers::NONE)
        .remove::<SpatialSplitPeerActive>();
}

/// Synchronizes each active split peer from the canonical solver body after
/// Avian has integrated continuous forces for the current substep but before
/// contact constraints are solved.
///
/// `CustomVelocityIntegration` keeps the peer from receiving gravity, damping,
/// or external forces independently. Copying the already-integrated canonical
/// velocity here gives it exactly the mapped motion of the one logical body.
pub(crate) fn sync_rigid_split_solver_peers(
    portals: Query<(Entity, &Portal, &PortalActive, &Transform), With<Portal>>,
    mut authorities: Query<
        (&PortalSplitTraveler, &mut PortalRigidSplitBody, &SolverBody),
        (
            Without<SpatialSplitPeer>,
            Without<Portal>,
            Without<CharacterMotor>,
        ),
    >,
    mut peers: Query<
        (Entity, &SpatialSplitPeer, &mut SolverBody),
        (With<SpatialSplitPeerActive>, Without<Portal>),
    >,
) {
    for (peer_entity, peer, mut peer_solver) in &mut peers {
        let Ok((split, mut rigid_split, authority_solver)) = authorities.get_mut(peer.authority)
        else {
            continue;
        };
        if split.peer() != peer_entity {
            continue;
        }
        if !rigid_split.peer_solver_active {
            continue;
        }
        let Some(active) = split.active else {
            continue;
        };
        let Ok((_, _, source_active, source)) = portals.get(active.source) else {
            continue;
        };
        let Ok((_, _, destination_active, destination)) = portals.get(active.destination) else {
            continue;
        };
        if !source_active.0 || !destination_active.0 {
            continue;
        }

        let mapping = portal_mapping(source, destination);
        peer_solver.linear_velocity = mapping.transform_vector3(authority_solver.linear_velocity);
        peer_solver.angular_velocity = mapping.transform_vector3(authority_solver.angular_velocity);
        rigid_split.peer_baseline_linear = peer_solver.linear_velocity;
        rigid_split.peer_baseline_angular = peer_solver.angular_velocity;
    }
}

/// Folds destination-side contact/joint impulses back into the canonical solver
/// body after each Avian substep.
///
/// The baseline was captured immediately before constraint solving, so this
/// delta contains only solver effects from the peer side; canonical gravity and
/// external forces are not counted twice.
pub(crate) fn couple_rigid_split_solver_peers(
    portals: Query<(Entity, &Portal, &PortalActive, &Transform), With<Portal>>,
    mut peers: Query<
        (Entity, &SpatialSplitPeer, &mut SolverBody),
        (With<SpatialSplitPeerActive>, Without<Portal>),
    >,
    mut authorities: Query<
        (
            &PortalSplitTraveler,
            &mut PortalRigidSplitBody,
            &mut SolverBody,
        ),
        (
            Without<SpatialSplitPeer>,
            Without<Portal>,
            Without<CharacterMotor>,
        ),
    >,
) {
    for (peer_entity, peer, mut peer_solver) in &mut peers {
        let Ok((split, mut rigid_split, mut authority_solver)) =
            authorities.get_mut(peer.authority)
        else {
            continue;
        };
        if split.peer() != peer_entity || !rigid_split.peer_solver_active {
            continue;
        }
        let Some(active) = split.active else {
            continue;
        };
        let Ok((_, _, source_active, source)) = portals.get(active.source) else {
            continue;
        };
        let Ok((_, _, destination_active, destination)) = portals.get(active.destination) else {
            continue;
        };
        if !source_active.0 || !destination_active.0 {
            continue;
        }

        let peer_linear_delta = peer_solver.linear_velocity - rigid_split.peer_baseline_linear;
        let peer_angular_delta = peer_solver.angular_velocity - rigid_split.peer_baseline_angular;
        let inverse_mapping = portal_mapping(destination, source);

        authority_solver.linear_velocity += inverse_mapping.transform_vector3(peer_linear_delta);
        authority_solver.angular_velocity += inverse_mapping.transform_vector3(peer_angular_delta);

        // Immediately resynchronize the proxy from the now-combined canonical
        // velocity. When this runs before position integration, both solver
        // bodies advance from one rigid velocity rather than drifting for a
        // substep and being corrected later.
        let mapping = portal_mapping(source, destination);
        peer_solver.linear_velocity = mapping.transform_vector3(authority_solver.linear_velocity);
        peer_solver.angular_velocity = mapping.transform_vector3(authority_solver.angular_velocity);
        rigid_split.peer_baseline_linear = peer_solver.linear_velocity;
        rigid_split.peer_baseline_angular = peer_solver.angular_velocity;
    }
}

/// Runs after Avian has solved and written back dynamic rigid-body state. Solver
/// impulse reconciliation already happened inside the physics schedules; this
/// pass owns portal center crossing, topology collapse, and component-space peer
/// synchronization for the next physics tick.
pub(crate) fn reconcile_rigid_splits(
    mut commands: Commands,
    portals: Query<(Entity, &Portal, &PortalActive, &Transform), With<Portal>>,
    mut authorities: Query<
        (
            Entity,
            &SpatialSplitBox,
            &mut Transform,
            &mut LinearVelocity,
            &mut AngularVelocity,
            &mut Collider,
            &mut PortalTraveler,
            &mut PortalSplitTraveler,
            &mut PortalRigidSplitBody,
        ),
        (
            Without<SpatialSplitPeer>,
            Without<Portal>,
            Without<CharacterMotor>,
        ),
    >,
    mut peers: Query<
        (
            &mut Transform,
            &mut LinearVelocity,
            &mut AngularVelocity,
            &mut Collider,
        ),
        (With<SpatialSplitPeer>, Without<Portal>),
    >,
) {
    for (
        _entity,
        split_box,
        mut body,
        mut velocity,
        mut angular_velocity,
        mut authority_collider,
        mut traveler,
        mut split,
        mut rigid_split,
    ) in &mut authorities
    {
        let peer_entity = split.peer();
        let Ok((mut peer_transform, mut peer_velocity, mut peer_angular, mut peer_collider)) =
            peers.get_mut(peer_entity)
        else {
            split.active = None;
            *authority_collider = split_box.full_collider();
            rigid_split.peer_solver_active = false;
            traveler.commit_position(body.translation);
            continue;
        };

        if let Some(active) = split.active {
            if let (
                Ok((_, source_portal, source_active, source)),
                Ok((_, _, destination_active, destination)),
            ) = (portals.get(active.source), portals.get(active.destination))
            {
                if source_active.0 && destination_active.0 {
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
                            let mapping = portal_mapping(source, destination);
                            *body = map_transform(&body, source, destination);
                            velocity.0 = mapping.transform_vector3(velocity.0);
                            angular_velocity.0 = mapping.transform_vector3(angular_velocity.0);
                            split.active = Some(ActivePortalSplit {
                                source: active.destination,
                                destination: active.source,
                            });
                        }
                    }
                }
            }
        }

        let should_finish = split.active.is_some_and(|active| {
            if !active_pair_is_valid(active, &portals) {
                return true;
            }

            portals
                .get(active.source)
                .ok()
                .and_then(|(_, _, _, source)| {
                    let plane = portal_plane(source)?;
                    let radius = split_box.projection_radius(body.rotation, plane.normal);
                    Some(plane.signed_distance(body.translation).abs() > radius + CLEAR_MARGIN)
                })
                .unwrap_or(true)
        });

        if should_finish {
            split.active = None;
            deactivate_peer(
                &mut commands,
                peer_entity,
                *split_box,
                &mut authority_collider,
                &mut peer_velocity,
                &mut peer_angular,
                &mut rigid_split,
            );
        } else if let Some(active) = split.active {
            let mut materialized = false;

            if let (
                Ok((_, _, source_active, source)),
                Ok((_, _, destination_active, destination)),
            ) = (portals.get(active.source), portals.get(active.destination))
            {
                if source_active.0 && destination_active.0 {
                    let mapping = portal_mapping(source, destination);
                    *peer_transform = map_transform(&body, source, destination);
                    peer_velocity.0 = mapping.transform_vector3(velocity.0);
                    peer_angular.0 = mapping.transform_vector3(angular_velocity.0);

                    if let Some(plane) = portal_plane(source) {
                        let partition = partition_box_by_plane(*split_box, &body, plane);
                        if partition.straddles() {
                            let authority_positive = partition.center_distance >= 0.0;
                            let (authority_half, peer_half) = if authority_positive {
                                (partition.positive_collider(), partition.negative_collider())
                            } else {
                                (partition.negative_collider(), partition.positive_collider())
                            };

                            if let (Some(authority_half), Some(peer_half)) =
                                (authority_half, peer_half)
                            {
                                *authority_collider = authority_half;
                                *peer_collider = peer_half;
                                rigid_split.peer_solver_active = true;
                                commands
                                    .entity(peer_entity)
                                    .insert((CollisionLayers::DEFAULT, SpatialSplitPeerActive));
                                materialized = true;
                            }
                        }
                    }
                }
            }

            if !materialized {
                deactivate_peer(
                    &mut commands,
                    peer_entity,
                    *split_box,
                    &mut authority_collider,
                    &mut peer_velocity,
                    &mut peer_angular,
                    &mut rigid_split,
                );
            }
        }

        traveler.commit_position(body.translation);
    }
}

fn active_pair_is_valid(
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

fn find_split_candidate(
    split_box: SpatialSplitBox,
    body: &Transform,
    velocity: Vec3,
    dt: f32,
    portals: &Query<(Entity, &Portal, &PortalActive, &Transform), With<Portal>>,
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
