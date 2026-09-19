//! Character ↔ split-rigid-body impulse bridging.

use avian3d::prelude::*;
use bevy::prelude::*;

use crate::{
    portal::{
        Portal, PortalActive, PortalRigidSplitBody, PortalSplitTraveler,
        topology::mapping::portal_mapping,
    },
    physics::{
        character::{
            CharacterMotor, CharacterPush, MAX_DYNAMIC_CONTACT_DELTA_SPEED,
            dynamic_contact_delta_velocity,
        },
        topology::{SpatialSplitPeer, SpatialSplitPeerActive},
    },
};

/// Maps character impulses that strike the destination-side proxy back onto the
/// one authoritative rigid body before portal peers are synchronized.
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
        let pair = match (
            portals.get(active.source),
            portals.get(active.destination),
        ) {
            (
                Ok((_, _, source_active, source)),
                Ok((_, _, destination_active, destination)),
            ) if source_active.0 && destination_active.0 => Some((source, destination)),
            _ => None,
        };
        let Some((source, destination)) = pair else {
            continue;
        };

        let inverse_mapping = portal_mapping(destination, source);
        let impulse = inverse_mapping.transform_vector3(push.impulse);
        let point = inverse_mapping.transform_point3(push.point);
        forces.apply_linear_impulse_at_point(impulse, point);
    }
}

/// Maps dynamic-body contact impulses from an active character split peer back
/// into the character authority after Avian has solved the physics step.
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
        let pair = match (
            portals.get(active.source),
            portals.get(active.destination),
        ) {
            (
                Ok((_, _, source_active, source)),
                Ok((_, _, destination_active, destination)),
            ) if source_active.0 && destination_active.0 => Some((source, destination)),
            _ => None,
        };
        let Some((source, destination)) = pair else {
            continue;
        };

        let inverse_mapping = portal_mapping(destination, source);
        let mut delta_velocity = Vec3::ZERO;

        for pair in collisions.collisions_with(peer_entity) {
            let peer_is_first = if pair.body1 == Some(peer_entity) || pair.collider1 == peer_entity {
                true
            } else if pair.body2 == Some(peer_entity) || pair.collider2 == peer_entity {
                false
            } else {
                continue;
            };

            let other_body = if peer_is_first { pair.body2 } else { pair.body1 };
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
