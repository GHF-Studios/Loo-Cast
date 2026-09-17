//! Bootstrap for the actual top-down procedural game-world path.

use avian3d::prelude::LinearVelocity;
use bevy::prelude::*;

use crate::{
    game::{player::{Player, PlayerNoclip}, portal::PortalTraveler},
    physics::character::{CharacterDimensions, CharacterMotor},
    procedural_assets::ProceduralAssetLibrary,
    spatial::{SpatialScale, UsfSpatialFrame},
    voxel::{ProceduralTerrain, VoxelBase, VoxelQueryPosition, VoxelStreaming, VoxelWorld},
    worldgen::{
        GEOLOGY_CLIMATE_HYDROLOGY, GeologyClimateHydrologyState, PhenomenonRegistry,
        TemporalScale, WorldgenEpoch, WorldgenEvaluationKey, WorldgenStore,
    },
};

use super::map_selection::GameMap;

#[derive(Component)]
struct ProceduralWorldRoot;

pub struct ProceduralWorldPlugin;

impl Plugin for ProceduralWorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameMap::ProceduralWorld),
            (spawn_procedural_world, prepare_player).chain(),
        );
    }
}

fn spawn_procedural_world(
    mut commands: Commands,
    frame: Res<UsfSpatialFrame>,
    procedural_assets: Res<ProceduralAssetLibrary>,
    registry: Res<PhenomenonRegistry>,
    mut worldgen: ResMut<WorldgenStore>,
) {
    let epoch = WorldgenEpoch::present_day_bootstrap();
    let before = worldgen.len();
    let leaf = worldgen
        .ensure_branch(
            *frame.origin(),
            SpatialScale::ZERO,
            TemporalScale::WORLDGEN_SNAPSHOT,
            epoch,
            &registry,
        )
        .expect("present-day worldgen branch must be canonically addressable");
    let generated = worldgen.len() - before;
    let terrain = terrain_for_branch(&worldgen, leaf);
    let lineage = worldgen.lineage(leaf);

    info!(
        generated_scopes = generated,
        lineage_depth = lineage.len(),
        epoch_gyr = epoch.age_gyr(),
        "generated sparse USF world branch from Scale +35 to Scale 0"
    );
    for node in lineage.iter().rev() {
        let phenomena = node
            .phenomena()
            .map(|snapshot| format!("{}: {}", snapshot.id().name(), snapshot.summary()))
            .collect::<Vec<_>>()
            .join(" | ");
        debug!(
            scale = %node.context().spatial_scale(),
            temporal_scale = node.context().temporal_scale().label(),
            %phenomena,
            "USF worldgen lineage"
        );
    }

    commands.spawn((
        Name::new("Procedural World"),
        ProceduralWorldRoot,
        // Voxel terrain is now parameterized by Scale-0 semantic geology state.
        // It remains a temporary realization algorithm, not world authority.
        VoxelWorld::new_at(VoxelBase::Terrain(terrain), *frame.origin()),
        VoxelStreaming::new(8, procedural_assets.cracked_clay.material.clone()),
        Transform::IDENTITY,
    ));
}

fn terrain_for_branch(worldgen: &WorldgenStore, leaf: WorldgenEvaluationKey) -> ProceduralTerrain {
    let geology = worldgen
        .state::<GeologyClimateHydrologyState>(leaf, GEOLOGY_CLIMATE_HYDROLOGY)
        .expect("Scale-0 branch must carry geology/climate/hydrology state");

    ProceduralTerrain::configured(
        geology.terrain_seed,
        -4.0,
        geology.local_relief_m,
        geology.terrain_frequency,
    )
}

/// Places the player above the first streamed terrain window with ordinary
/// character physics enabled. The terrain parameters come from the exact same
/// Scale-0 semantic branch used to create the voxel realization.
fn prepare_player(
    mut commands: Commands,
    frame: Res<UsfSpatialFrame>,
    worldgen: Res<WorldgenStore>,
    player: Single<(
        Entity,
        &mut Transform,
        &mut PortalTraveler,
        &mut LinearVelocity,
        &mut PlayerNoclip,
    ), With<Player>>,
) {
    let epoch = WorldgenEpoch::present_day_bootstrap();
    let leaf = worldgen
        .key_for(
            *frame.origin(),
            SpatialScale::ZERO,
            TemporalScale::WORLDGEN_SNAPSHOT,
            epoch,
        )
        .expect("player worldgen branch must be canonically addressable");
    let terrain = terrain_for_branch(&worldgen, leaf);

    let (entity, mut transform, mut traveler, mut velocity, mut noclip) = player.into_inner();
    let x = 0.0;
    let z = 8.0;
    let world_origin = VoxelQueryPosition::new(*frame.origin());
    let terrain_query = world_origin
        .translated(Vec3::new(x, 0.0, z))
        .expect("procedural spawn query must translate canonically");
    let ground = terrain.height_at(world_origin, terrain_query);
    let position = Vec3::new(x, ground + CharacterDimensions::HALF_HEIGHT + 0.20, z);

    transform.translation = position;
    traveler.commit_position(position);
    velocity.0 = Vec3::ZERO;
    noclip.active = false;
    commands.entity(entity).insert(CharacterMotor);
}
