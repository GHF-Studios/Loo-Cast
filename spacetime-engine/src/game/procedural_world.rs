//! Bootstrap for the actual top-down procedural game-world path.

use avian3d::prelude::LinearVelocity;
use bevy::prelude::*;

use crate::{
    game::{
        player::{Player, PlayerNoclip},
        portal::PortalTraveler,
    },
    physics::character::{CharacterDimensions, CharacterMotor},
    procedural_assets::ProceduralAssetLibrary,
    spatial::{SpatialScale, UsfSpatialFrame},
    voxel::{ProceduralVolume, VoxelBase, VoxelQueryPosition, VoxelStreaming, VoxelWorld},
    worldgen::{
        GEOLOGY_CLIMATE_HYDROLOGY, GeologyClimateHydrologyState, PhenomenonRegistry, TemporalScale,
        WorldgenEpoch, WorldgenEvaluationKey, WorldgenStore,
    },
};

use super::{map_selection::GameMap, scale_stack::ProceduralScaleStack};

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

fn spawn_procedural_world(
    mut commands: Commands,
    frame: Res<UsfSpatialFrame>,
    procedural_assets: Res<ProceduralAssetLibrary>,
    registry: Res<PhenomenonRegistry>,
    mut worldgen: ResMut<WorldgenStore>,
) {
    let epoch = WorldgenEpoch::present_day_bootstrap();
    let before = worldgen.len();
    let root = worldgen
        .bootstrap_root(
            *frame.origin(),
            TemporalScale::WORLDGEN_SNAPSHOT,
            epoch,
            &registry,
        )
        .expect("present-day root context must be canonically addressable");
    let leaf = worldgen
        .contextualize_to(root, *frame.origin(), SpatialScale::ZERO, &registry)
        .expect("present-day refinement must be canonically addressable")
        .expect("Scale +35 root must contextualize the playable Scale 0 branch");
    let generated = worldgen.len() - before;
    let volume = volume_for_branch(&worldgen, leaf);
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
        VoxelWorld::new_at(VoxelBase::Volume(volume), *frame.origin()),
        VoxelStreaming::new(24, procedural_assets.cracked_clay.material.clone()),
        ProceduralScaleStack::new(*frame.origin(), volume),
        Transform::IDENTITY,
    ));
}

fn volume_for_branch(worldgen: &WorldgenStore, leaf: WorldgenEvaluationKey) -> ProceduralVolume {
    let geology = worldgen
        .state::<GeologyClimateHydrologyState>(leaf, GEOLOGY_CLIMATE_HYDROLOGY)
        .expect("worldgen branch must carry geology/climate/hydrology state");

    ProceduralVolume::configured(
        geology.terrain_seed,
        -4.0,
        geology.local_relief_m,
        geology.terrain_frequency,
        geology.cave_potential,
        (geology.tectonic_activity * 0.65 + geology.rockiness * 0.35).clamp(0.0, 1.0),
    )
}

fn prepare_player(
    mut commands: Commands,
    frame: Res<UsfSpatialFrame>,
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
    let leaf = worldgen
        .key_for(
            *frame.origin(),
            SpatialScale::ZERO,
            TemporalScale::WORLDGEN_SNAPSHOT,
            epoch,
        )
        .expect("player worldgen branch must be canonically addressable");
    let volume = volume_for_branch(&worldgen, leaf);

    let (entity, mut transform, mut traveler, mut velocity, mut noclip) = player.into_inner();
    let x = 0.0;
    let z = 8.0;
    let world_origin = VoxelQueryPosition::new(*frame.origin());
    let terrain_query = world_origin
        .translated(Vec3::new(x, 0.0, z))
        .expect("procedural spawn query must translate canonically");
    // Spawn placement uses the exterior reference surface only; actual
    // world matter/collision is governed by the full 3D volume.
    let ground = volume.reference_surface_height_at(world_origin, terrain_query);
    let position = Vec3::new(x, ground + CharacterDimensions::HALF_HEIGHT + 0.20, z);

    transform.translation = position;
    traveler.commit_position(position);
    velocity.0 = Vec3::ZERO;
    noclip.active = false;
    commands.entity(entity).insert(CharacterMotor);
}
