//! Post-physics rigid split reconciliation.
//!
//! This system sequences authority crossing, split completion and disposable
//! peer rematerialization after Avian writes dynamic-body state back.

use avian3d::prelude::*;
use bevy::prelude::*;

use crate::portal::simulation::split::active_portal_pair;

use crate::{
    physics::{
        DetailedBodyCollision, PhysicalBoxHull,
        character::CharacterMotor,
        topology::{SpatialSplitBox, SpatialSplitPeer},
    },
    portal::{Portal, PortalActive, PortalRigidSplitBody, PortalSplitTraveler, PortalTraveler},
    spatial::{SpatialScale, UsfScaleLayer},
};

use super::super::split::retire_split_partition;
use super::peer::{
    AuthorityMotion, PeerComponents, PeerMaterialization, SolverBaseline, deactivate_peer,
    materialize_peer,
};

mod completion;
mod crossing;

pub(crate) fn reconcile_rigid_splits(
    mut commands: Commands,
    portals: Query<(Entity, &Portal, &PortalActive, &Transform), With<Portal>>,
    mut authorities: Query<
        (
            Entity,
            &PhysicalBoxHull,
            Option<&UsfScaleLayer>,
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
            With<DetailedBodyCollision>,
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
        hull,
        layer,
        mut body,
        mut velocity,
        mut angular_velocity,
        mut authority_collider,
        mut traveler,
        mut split,
        mut rigid_split,
    ) in &mut authorities
    {
        let scale = layer.map_or(SpatialScale::ZERO, |layer| layer.scale());
        let split_box = SpatialSplitBox::from_physical(*hull, scale);
        let peer_entity = split.solver_peer();
        let Ok((mut peer_transform, mut peer_velocity, mut peer_angular, mut peer_collider)) =
            peers.get_mut(peer_entity)
        else {
            retire_split_partition(&mut commands, &mut split);
            *authority_collider = split_box.full_collider();
            rigid_split.peer_solver_active = false;
            traveler.commit_position(body.translation);
            continue;
        };

        crossing::resolve_authority_crossing(
            split_box,
            &mut body,
            &mut velocity,
            &mut angular_velocity,
            &mut split,
            &portals,
        );

        if completion::split_should_finish(split_box, &body, &split, &portals) {
            retire_split_partition(&mut commands, &mut split);
            deactivate_peer(
                &mut commands,
                peer_entity,
                split_box,
                &mut authority_collider,
                &mut peer_velocity,
                &mut peer_angular,
                &mut rigid_split,
            );
        } else if let Some(active) = split.active {
            let materialized =
                active_portal_pair(active, &portals).is_some_and(|(_, source, destination)| {
                    materialize_peer(
                        &mut commands,
                        AuthorityMotion {
                            body: &body,
                            linear: velocity.0,
                            angular: angular_velocity.0,
                        },
                        PeerMaterialization {
                            entity: peer_entity,
                            split_box: split_box,
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
                            solver_baseline: SolverBaseline::Preserve,
                        },
                    )
                });

            if !materialized {
                deactivate_peer(
                    &mut commands,
                    peer_entity,
                    split_box,
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
