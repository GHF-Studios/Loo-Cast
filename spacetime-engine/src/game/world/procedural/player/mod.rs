//! Prepare the local player for the procedural root world.

use avian3d::prelude::LinearVelocity;
use bevy::prelude::*;

use crate::{
    game::player::{Player, PlayerNoclip},
    physics::character::{CharacterDimensions, CharacterMotor},
    portal::PortalTraveler,
    spatial::{SpatialScale, UsfPosition},
    voxel::VoxelQueryPosition,
    worldgen::{TemporalScale, WorldgenEpoch, WorldgenStore},
};

use super::scale_stack::volume_for_scale_context;

pub(super) fn prepare_player(
    mut commands: Commands,
    worldgen: Res<WorldgenStore>,
    player: Single<
        (
            Entity,
            &mut Transform,
            &mut PortalTraveler,
            &mut LinearVelocity,
            &mut PlayerNoclip,
        ),
        With<Player>,
    >,
) {
    let epoch = WorldgenEpoch::present_day_bootstrap();
    let target = UsfPosition::default();
    let root = worldgen
        .key_for(
            target,
            SpatialScale::MAX,
            TemporalScale::WORLDGEN_SNAPSHOT,
            epoch,
        )
        .expect("root player context must be addressable");
    let volume = volume_for_scale_context(&worldgen, root);

    let world_origin = VoxelQueryPosition::new(UsfPosition::zero(SpatialScale::MAX));
    let x = 0.0;
    let z = 8.0;
    let query = world_origin
        .translated(Vec3::new(x, 0.0, z))
        .expect("root-layer spawn query must stay local");
    let ground = volume.reference_surface_height_at(world_origin, query);

    let position = Vec3::new(x, ground + CharacterDimensions::HALF_HEIGHT + 12.0, z);

    let (entity, mut transform, mut traveler, mut velocity, mut noclip) = player.into_inner();
    transform.translation = position;
    traveler.commit_position(position);
    velocity.0 = Vec3::ZERO;
    noclip.active = false;
    commands.entity(entity).insert(CharacterMotor);
}
