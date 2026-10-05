//! Avian substep coupling for split rigid bodies.
//!
//! The authority is the only semantic rigid body. The peer receives mapped
//! canonical velocity before constraints; solver-only deltas are inverse-mapped
//! back afterward, then the peer is resynchronized from the combined authority.

use avian3d::dynamics::solver::solver_body::SolverBody;
use bevy::prelude::*;

use crate::portal::simulation::split::active_portal_pair;

use crate::{
    portal::{
        Portal, PortalActive, PortalRigidSplitBody, PortalSplitTraveler,
        topology::mapping::portal_mapping,
    },
    physics::{
        character::CharacterMotor,
        topology::{SpatialSplitPeer, SpatialSplitPeerActive},
    },
};

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
        if split.solver_peer() != peer_entity || !rigid_split.peer_solver_active {
            continue;
        }

        let Some(active) = split.active else {
            continue;
        };
        let Some((_, source, destination)) = active_portal_pair(active, &portals) else {
            continue;
        };

        let mapping = portal_mapping(source, destination);
        peer_solver.linear_velocity = mapping.transform_vector3(authority_solver.linear_velocity);
        peer_solver.angular_velocity = mapping.transform_vector3(authority_solver.angular_velocity);
        rigid_split.peer_baseline_linear = peer_solver.linear_velocity;
        rigid_split.peer_baseline_angular = peer_solver.angular_velocity;
    }
}

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
        if split.solver_peer() != peer_entity || !rigid_split.peer_solver_active {
            continue;
        }

        let Some(active) = split.active else {
            continue;
        };
        let Some((_, source, destination)) = active_portal_pair(active, &portals) else {
            continue;
        };

        let peer_linear_delta = peer_solver.linear_velocity - rigid_split.peer_baseline_linear;
        let peer_angular_delta = peer_solver.angular_velocity - rigid_split.peer_baseline_angular;
        let inverse_mapping = portal_mapping(destination, source);

        authority_solver.linear_velocity += inverse_mapping.transform_vector3(peer_linear_delta);
        authority_solver.angular_velocity += inverse_mapping.transform_vector3(peer_angular_delta);

        // Immediately resynchronize the proxy from the now-combined canonical
        // velocity so both solver bodies advance from one rigid velocity.
        let mapping = portal_mapping(source, destination);
        peer_solver.linear_velocity = mapping.transform_vector3(authority_solver.linear_velocity);
        peer_solver.angular_velocity = mapping.transform_vector3(authority_solver.angular_velocity);
        rigid_split.peer_baseline_linear = peer_solver.linear_velocity;
        rigid_split.peer_baseline_angular = peer_solver.angular_velocity;
    }
}
