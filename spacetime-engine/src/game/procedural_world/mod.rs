//! Bootstrap for the top-down hierarchical scale-layer procedural-world test.

use avian3d::prelude::LinearVelocity;
use bevy::prelude::*;

use crate::{
    config::EngineConfig,
    game::{
        player::{Player, PlayerNoclip},
        portal::PortalTraveler,
    },
    physics::character::{CharacterDimensions, CharacterMotor},
    procedural_assets::ProceduralAssetLibrary,
    spatial::{SpatialScale, UsfPosition, UsfScaleLayer},
    voxel::{VoxelBase, VoxelPresentationMaterial, VoxelQueryPosition, VoxelStreaming, VoxelWorld},
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
    config: Res<EngineConfig>,
    mut commands: Commands,
    procedural_assets: Res<ProceduralAssetLibrary>,
    registry: Res<PhenomenonRegistry>,
    mut worldgen: ResMut<WorldgenStore>,
) {
    let epoch = WorldgenEpoch::present_day_bootstrap();
    let target = semantic_test_target();
    let root = worldgen
        .bootstrap_root(target, TemporalScale::WORLDGEN_SNAPSHOT, epoch, &registry)
        .expect("present-day root context must be canonically addressable");
    let volume = volume_for_scale_context(&worldgen, root);

    let stack_entity = commands
        .spawn((
            Name::new("Procedural Hierarchical Scale Stack"),
            ProceduralWorldRoot,
            Transform::IDENTITY,
            Visibility::Inherited,
        ))
        .id();

    let root_world = commands
        .spawn((
            Name::new("USF Scale +35 Root Voxel World"),
            ChildOf(stack_entity),
            UsfScaleLayer::new(SpatialScale::MAX),
            VoxelWorld::new_at(
                VoxelBase::Volume(volume),
                UsfPosition::zero(SpatialScale::MAX),
            ),
            VoxelStreaming::new(config.voxel.streaming.default_load_budget_per_frame),
            VoxelPresentationMaterial::new(procedural_assets.debug_grid.clone()),
            Transform::IDENTITY,
            Visibility::Inherited,
        ))
        .id();

    commands
        .entity(stack_entity)
        .insert(ProceduralScaleStack::new(
            target,
            root,
            procedural_assets.debug_grid.clone(),
            root_world,
        ));

    info!(
        epoch_gyr = epoch.age_gyr(),
        "bootstrapped visible Scale +35 voxel root; finer scales remain contextual refinements"
    );
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

    let position = Vec3::new(x, ground + CharacterDimensions::HALF_HEIGHT + 12.0, z);

    let (entity, mut transform, mut traveler, mut velocity, mut noclip) = player.into_inner();
    transform.translation = position;
    traveler.commit_position(position);
    velocity.0 = Vec3::ZERO;
    noclip.active = false;
    commands.entity(entity).insert(CharacterMotor);
}
