//! Bootstrap for the top-down scale-layer procedural-world test.

use avian3d::prelude::LinearVelocity;
use bevy::prelude::*;

use crate::{
    game::{
        player::{Player, PlayerNoclip},
        portal::PortalTraveler,
    },
    physics::character::{CharacterDimensions, CharacterMotor},
    procedural_assets::ProceduralAssetLibrary,
    spatial::{SpatialScale, UsfPosition},
    voxel::VoxelQueryPosition,
    worldgen::{PhenomenonRegistry, TemporalScale, WorldgenEpoch, WorldgenStore},
};

use super::{
    map_selection::GameMap,
    scale_stack::{ProceduralScaleStack, volume_for_scale_context},
};

#[derive(Component)]
struct ProceduralWorldRoot;

pub struct ProceduralWorldPlugin;

impl Plugin for ProceduralWorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameMap::ProceduralWorld),
            (spawn_procedural_world, prepare_player).chain(),
        )
        .add_systems(Update, super::scale_stack::sync_scale_stack);
    }
}

fn semantic_test_target() -> UsfPosition {
    UsfPosition::default()
}

fn spawn_procedural_world(
    mut commands: Commands,
    procedural_assets: Res<ProceduralAssetLibrary>,
    registry: Res<PhenomenonRegistry>,
    mut worldgen: ResMut<WorldgenStore>,
) {
    let epoch = WorldgenEpoch::present_day_bootstrap();
    let target = semantic_test_target();
    let before = worldgen.len();
    let root = worldgen
        .bootstrap_root(target, TemporalScale::WORLDGEN_SNAPSHOT, epoch, &registry)
        .expect("present-day root context must be addressable");

    info!(
        generated_scopes = worldgen.len() - before,
        epoch_gyr = epoch.age_gyr(),
        "bootstrapped Scale +35 USF context; finer contexts remain demand-driven"
    );

    commands.spawn((
        Name::new("Procedural Scale-Layer World Stack"),
        ProceduralWorldRoot,
        ProceduralScaleStack::new(
            target,
            root,
            procedural_assets.cracked_clay.material.clone(),
        ),
        Transform::IDENTITY,
        Visibility::Inherited,
    ));
}

fn prepare_player(
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
    let target = semantic_test_target();
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
    let position = Vec3::new(x, ground + CharacterDimensions::HALF_HEIGHT + 0.20, z);

    let (entity, mut transform, mut traveler, mut velocity, mut noclip) = player.into_inner();
    transform.translation = position;
    traveler.commit_position(position);
    velocity.0 = Vec3::ZERO;
    noclip.active = false;
    commands.entity(entity).insert(CharacterMotor);
}
