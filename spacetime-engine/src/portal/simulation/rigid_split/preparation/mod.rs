//! Pre-physics rigid split candidate selection and peer preparation.

use avian3d::prelude::*;
use bevy::prelude::*;

use crate::{
    ecs::{UsfAuthorityPartitionOf, UsfLogicalRealizationOf},
    portal::{Portal, PortalActive, PortalRigidSplitBody, PortalSplitTraveler},
    physics::{
        DetailedBodyCollision, PhysicalBoxHull,
        character::CharacterMotor,
        topology::{SpatialSplitBox, SpatialSplitPeer},
    },
    spatial::{SpatialScale, UsfScaleLayer},
};

use super::peer::{AuthorityMotion, PeerComponents, PeerMaterialization, deactivate_peer, materialize_peer};
use super::super::split::{
    activate_split_partition, active_pair_is_valid, box_reaches_portal_this_tick,
    find_split_candidate, retire_split_partition,
};

pub(crate) fn prepare_rigid_splits(
    time: Res<Time<Fixed>>,
    mut commands: Commands,
    portals: Query<(Entity, &Portal, &PortalActive, &Transform), With<Portal>>,
    partitions: Query<&UsfAuthorityPartitionOf>,
    mut authorities: Query<
        (
            Entity,
            &UsfLogicalRealizationOf,
            &Transform,
            &LinearVelocity,
            &AngularVelocity,
            &PhysicalBoxHull,
            Option<&UsfScaleLayer>,
            &mut PortalSplitTraveler,
            &mut PortalRigidSplitBody,
            &mut Collider,
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
    let dt = time.delta_secs().max(0.0);

    for (
        _entity,
        primary,
        body,
        velocity,
        angular_velocity,
        hull,
        layer,
        mut split,
        mut rigid_split,
        mut authority_collider,
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
            continue;
        };

        if let Some(active) = split.active {
            if !active_pair_is_valid(active, &portals)
                || !box_reaches_portal_this_tick(
                    split_box,
                    body,
                    velocity.0,
                    dt,
                    active.source,
                    &portals,
                )
            {
                retire_split_partition(&mut commands, &mut split);
            }
        }

        split.tick_start = *body;
        if split.active.is_none()
            && let Some((source, destination)) =
                find_split_candidate(split_box, body, velocity.0, dt, &portals)
        {
            split.active = activate_split_partition(
                &mut commands,
                primary,
                &partitions,
                peer_entity,
                source,
                destination,
            );
        }

        let Some(active) = split.active else {
            deactivate_peer(
                &mut commands,
                peer_entity,
                split_box,
                &mut authority_collider,
                &mut peer_velocity,
                &mut peer_angular,
                &mut rigid_split,
            );
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
            continue;
        };

        if !materialize_peer(
            &mut commands,
            AuthorityMotion {
                body,
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
                capture_solver_baseline: true,
            },
        ) {
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
}
