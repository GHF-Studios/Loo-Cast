//! Pre-physics rigid split candidate selection and peer preparation.

use avian3d::prelude::*;
use bevy::prelude::*;

use crate::portal::simulation::split::active_portal_pair;

use crate::{
    ecs::{UsfAuthorityPartitionOf, UsfLogicalRealizationOf},
    physics::{
        DetailedBodyCollision, PhysicalBoxHull,
        character::CharacterMotor,
        topology::{SpatialSplitBox, SpatialSplitPeer},
    },
    portal::{Portal, PortalActive, PortalRigidSplitBody, PortalSplitTraveler},
    spatial::{SpatialScale, UsfScaleLayer},
};

use super::super::split::{
    activate_split_partition, active_pair_is_valid, box_reaches_portal_this_tick,
    find_split_candidate, retire_split_partition,
};
use super::peer::{
    AuthorityMotion, PeerComponents, PeerMaterialization, SolverBaseline, deactivate_peer,
    materialize_peer,
};

/// Keep partition membership current before touching either solver collider.
/// A peer is only materialized after this has selected a valid portal pair.
fn refresh_split_partition(
    commands: &mut Commands,
    portals: &Query<(Entity, &Portal, &PortalActive, &Transform), With<Portal>>,
    partitions: &Query<&UsfAuthorityPartitionOf>,
    primary: &UsfLogicalRealizationOf,
    split: &mut PortalSplitTraveler,
    peer_entity: Entity,
    split_box: SpatialSplitBox,
    body: &Transform,
    velocity: Vec3,
    dt: f32,
) {
    if let Some(active) = split.active {
        if !active_pair_is_valid(active, portals)
            || !box_reaches_portal_this_tick(split_box, body, velocity, dt, active.source, portals)
        {
            retire_split_partition(commands, split);
        }
    }

    split.tick_start = *body;
    if split.active.is_none()
        && let Some((source, destination)) =
            find_split_candidate(split_box, body, velocity, dt, portals)
    {
        split.active = activate_split_partition(
            commands,
            primary,
            partitions,
            peer_entity,
            source,
            destination,
        );
    }
}

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

        refresh_split_partition(
            &mut commands,
            &portals,
            &partitions,
            primary,
            &mut split,
            peer_entity,
            split_box,
            body,
            velocity.0,
            dt,
        );

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

        let Some((_, source, destination)) = active_portal_pair(active, &portals) else {
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
                solver_baseline: SolverBaseline::Capture,
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
