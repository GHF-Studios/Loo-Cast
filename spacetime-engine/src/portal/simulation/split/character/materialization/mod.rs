//! Character authority/peer collider materialization.

use avian3d::prelude::*;
use bevy::prelude::*;

use crate::{
    ecs::UsfLogicalProjection,
    portal::{
        Portal, PortalActive, PortalSplitTraveler,
        topology::mapping::{map_transform, portal_mapping, portal_plane},
    },
    physics::{
        DetailedBodyCollision, PhysicalBoxHull,
        topology::{
            KinematicQueryExclusions, SpatialSplitBox, SpatialSplitPeer, SpatialSplitPeerActive,
            partition_box_by_plane,
        },
    },
    spatial::UsfScaleLayer,
};

/// Rebuilds the two physical manifestation colliders from the same rigid box.
///
/// While the box intersects the portal plane, the primary logical projection
/// owns the half on its current side and the peer owns the complementary half
/// mapped through the portal. Outside the overlap interval the peer is disabled.
pub(crate) fn materialize_portal_splits(
    mut commands: Commands,
    portals: Query<(&Portal, &PortalActive, &Transform), With<Portal>>,
    mut authorities: Query<
        (
            Entity,
            &Transform,
            &LinearVelocity,
            &PhysicalBoxHull,
            &UsfScaleLayer,
            &PortalSplitTraveler,
            &mut Collider,
        ),
        (
            With<UsfLogicalProjection>,
            With<KinematicQueryExclusions>,
            With<DetailedBodyCollision>,
            Without<SpatialSplitPeer>,
            Without<Portal>,
        ),
    >,
    mut peers: Query<
        (&mut Transform, &mut LinearVelocity, &mut Collider),
        (
            With<UsfLogicalProjection>,
            With<SpatialSplitPeer>,
            Without<Portal>,
        ),
    >,
) {
    for (_authority, body, velocity, hull, layer, split, mut authority_collider) in &mut authorities {
        let split_box = SpatialSplitBox::from_physical(*hull, layer.scale());
        let peer_entity = split.solver_peer();
        let Ok((mut peer_transform, mut peer_velocity, mut peer_collider)) =
            peers.get_mut(peer_entity)
        else {
            continue;
        };

        let Some(active) = split.active else {
            restore_full_authority(
                &mut commands,
                peer_entity,
                split_box,
                &mut authority_collider,
            );
            peer_velocity.0 = Vec3::ZERO;
            continue;
        };

        let pair = match (
            portals.get(active.source),
            portals.get(active.destination),
        ) {
            (
                Ok((_, source_active, source)),
                Ok((_, destination_active, destination)),
            ) if source_active.0 && destination_active.0 => Some((source, destination)),
            _ => None,
        };

        let Some((source, destination)) = pair else {
            restore_full_authority(
                &mut commands,
                peer_entity,
                split_box,
                &mut authority_collider,
            );
            continue;
        };

        let Some(plane) = portal_plane(source) else {
            continue;
        };
        let partition = partition_box_by_plane(split_box, body, plane);

        *peer_transform = map_transform(body, source, destination);
        peer_velocity.0 = portal_mapping(source, destination).transform_vector3(velocity.0);

        if !partition.straddles() {
            restore_full_authority(
                &mut commands,
                peer_entity,
                split_box,
                &mut authority_collider,
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
            restore_full_authority(
                &mut commands,
                peer_entity,
                split_box,
                &mut authority_collider,
            );
            continue;
        };

        *authority_collider = authority_half;
        *peer_collider = peer_half;
        commands
            .entity(peer_entity)
            .insert((CollisionLayers::DEFAULT, SpatialSplitPeerActive));
    }
}

fn restore_full_authority(
    commands: &mut Commands,
    peer: Entity,
    split_box: SpatialSplitBox,
    authority_collider: &mut Collider,
) {
    *authority_collider = split_box.full_collider();
    commands
        .entity(peer)
        .insert(CollisionLayers::NONE)
        .remove::<SpatialSplitPeerActive>();
}
