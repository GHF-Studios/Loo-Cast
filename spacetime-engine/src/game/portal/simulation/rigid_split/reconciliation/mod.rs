//! Post-physics rigid split reconciliation.
//!
//! This system sequences authority crossing, split completion and disposable
//! peer rematerialization after Avian writes dynamic-body state back.

use avian3d::prelude::*;
use bevy::prelude::*;

use crate::{
    game::portal::{
        Portal, PortalActive, PortalRigidSplitBody, PortalSplitTraveler, PortalTraveler,
    },
    physics::{
        character::CharacterMotor,
        topology::{SpatialSplitBox, SpatialSplitPeer},
    },
};

use super::peer::{
    AuthorityMotion, PeerComponents, PeerMaterialization, deactivate_peer, materialize_peer,
};

mod completion;
mod crossing;

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

        crossing::resolve_authority_crossing(
            *split_box,
            &mut body,
            &mut velocity,
            &mut angular_velocity,
            &mut split,
            &portals,
        );

        if completion::split_should_finish(*split_box, &body, &split, &portals) {
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

            let materialized = pair.is_some_and(|(source, destination)| {
                materialize_peer(
                    &mut commands,
                    AuthorityMotion {
                        body: &body,
                        linear: velocity.0,
                        angular: angular_velocity.0,
                    },
                    PeerMaterialization {
                        entity: peer_entity,
                        split_box: *split_box,
                        source,
                        destination,
                        authority_collider: &mut authority_collider,
                        peer: PeerComponents {
                            transform: &mut peer_transform,
                            linear: &mut peer_velocity,
                            angular: &mut peer_angular,
                            collider: &mut peer_collider,
                        },
                        rigid_split: &mut rigid_split,
                        capture_solver_baseline: false,
                    },
                )
            });

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
