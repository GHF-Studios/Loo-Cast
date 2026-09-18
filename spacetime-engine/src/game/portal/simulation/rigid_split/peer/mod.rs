//! Disposable rigid split-peer materialization.

use avian3d::prelude::*;
use bevy::prelude::*;

use crate::{
    game::portal::{
        PortalRigidSplitBody,
        topology::mapping::{map_transform, portal_mapping, portal_plane},
    },
    physics::topology::{
        SpatialSplitBox, SpatialSplitPeerActive, partition_box_by_plane,
    },
};

pub(super) struct AuthorityMotion<'a> {
    pub body: &'a Transform,
    pub linear: Vec3,
    pub angular: Vec3,
}

pub(super) struct PeerComponents<'a> {
    pub transform: &'a mut Transform,
    pub linear: &'a mut LinearVelocity,
    pub angular: &'a mut AngularVelocity,
    pub collider: &'a mut Collider,
}

pub(super) struct PeerMaterialization<'a> {
    pub entity: Entity,
    pub split_box: SpatialSplitBox,
    pub source: &'a Transform,
    pub destination: &'a Transform,
    pub authority_collider: &'a mut Collider,
    pub peer: PeerComponents<'a>,
    pub rigid_split: &'a mut PortalRigidSplitBody,
    pub capture_solver_baseline: bool,
}

pub(super) fn materialize_peer(
    commands: &mut Commands,
    authority: AuthorityMotion<'_>,
    materialization: PeerMaterialization<'_>,
) -> bool {
    let PeerMaterialization {
        entity,
        split_box,
        source,
        destination,
        authority_collider,
        peer,
        rigid_split,
        capture_solver_baseline,
    } = materialization;

    let mapping = portal_mapping(source, destination);
    *peer.transform = map_transform(authority.body, source, destination);
    peer.linear.0 = mapping.transform_vector3(authority.linear);
    peer.angular.0 = mapping.transform_vector3(authority.angular);

    if capture_solver_baseline {
        rigid_split.peer_baseline_linear = peer.linear.0;
        rigid_split.peer_baseline_angular = peer.angular.0;
    }

    let Some(plane) = portal_plane(source) else {
        return false;
    };
    let partition = partition_box_by_plane(split_box, authority.body, plane);
    if !partition.straddles() {
        return false;
    }

    let authority_positive = partition.center_distance >= 0.0;
    let (authority_half, peer_half) = if authority_positive {
        (partition.positive_collider(), partition.negative_collider())
    } else {
        (partition.negative_collider(), partition.positive_collider())
    };

    let (Some(authority_half), Some(peer_half)) = (authority_half, peer_half) else {
        return false;
    };

    *authority_collider = authority_half;
    *peer.collider = peer_half;
    rigid_split.peer_solver_active = true;
    commands
        .entity(entity)
        .insert((CollisionLayers::DEFAULT, SpatialSplitPeerActive));

    true
}

pub(super) fn deactivate_peer(
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
