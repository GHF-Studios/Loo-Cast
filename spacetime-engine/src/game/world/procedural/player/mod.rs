//! Prepare the local player for the procedural root world.

use avian3d::prelude::LinearVelocity;
use bevy::prelude::*;

use crate::{
    game::{
        control::LocalControlSubject,
        locomotion::{ControlledSubjectHull, ControlledSubjectLocomotion},
    },
    physics::character::CharacterDimensions,
    portal::PortalTraveler,
    spatial::{SpatialScale, UsfPosition},
    voxel::VoxelQueryPosition,
    worldgen::{TemporalScale, WorldgenEpoch, WorldgenStore},
};

use super::scale_stack::volume_for_scale_context;

pub(super) fn prepare_player(
    worldgen: Res<WorldgenStore>,
    player: Single<
        (
            &mut Transform,
            &mut PortalTraveler,
            &mut LinearVelocity,
            &mut ControlledSubjectLocomotion,
            Option<&ControlledSubjectHull>,
        ),
        With<LocalControlSubject>,
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

    let (mut transform, mut traveler, mut velocity, mut locomotion, hull) =
        player.into_inner();
    let half_height = hull
        .map(|hull| hull.size().y * 0.5)
        .unwrap_or(CharacterDimensions::HALF_HEIGHT);
    let position = Vec3::new(x, ground + half_height + 12.0, z);
    transform.translation = position;
    traveler.commit_position(position);
    velocity.0 = Vec3::ZERO;
    locomotion.request_automatic();
    locomotion.set_thrusters_enabled(false);
}
