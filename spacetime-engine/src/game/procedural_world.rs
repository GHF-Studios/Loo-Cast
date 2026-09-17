//! Bootstrap for the actual procedural game-world path.

use avian3d::prelude::LinearVelocity;
use bevy::prelude::*;

use crate::{
    game::{player::{Player, PlayerNoclip}, portal::PortalTraveler},
    physics::character::{CharacterDimensions, CharacterMotor},
    spatial::UsfSpatialFrame,
    voxel::{ProceduralTerrain, VoxelBase, VoxelQueryPosition, VoxelStreaming, VoxelWorld},
};

use super::map_selection::GameMap;

const WORLD_SEED: u32 = 0x10_0CA57;

#[derive(Component)]
struct ProceduralWorldRoot;

pub struct ProceduralWorldPlugin;

impl Plugin for ProceduralWorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameMap::ProceduralWorld),
            (spawn_procedural_world, prepare_player),
        );
    }
}

fn spawn_procedural_world(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    frame: Res<UsfSpatialFrame>,
) {
    let material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.30, 0.27, 0.23),
        perceptual_roughness: 1.0,
        ..default()
    });

    commands.spawn((
        Name::new("Procedural World"),
        ProceduralWorldRoot,
        VoxelWorld::new_at(VoxelBase::terrain(WORLD_SEED), *frame.origin()),
        // Spatial extent now comes entirely from generic canonical demand
        // sources. This component only configures voxel realization throughput
        // and representation assets for satisfying that merged demand.
        VoxelStreaming::new(8, material),
        Transform::IDENTITY,
    ));
}

/// Places the player above the first streamed terrain window with ordinary
/// character physics enabled. Chunk loading is nearest-first, so the terrain
/// immediately below the player receives collision before distant chunks.
fn prepare_player(
    mut commands: Commands,
    frame: Res<UsfSpatialFrame>,
    player: Single<(
        Entity,
        &mut Transform,
        &mut PortalTraveler,
        &mut LinearVelocity,
        &mut PlayerNoclip,
    ), With<Player>>,
) {
    let (entity, mut transform, mut traveler, mut velocity, mut noclip) = player.into_inner();
    let x = 0.0;
    let z = 8.0;
    let world_origin = VoxelQueryPosition::new(*frame.origin());
    let terrain_query = world_origin
        .translated(Vec3::new(x, 0.0, z))
        .expect("procedural spawn query must translate canonically");
    let ground = ProceduralTerrain::new(WORLD_SEED).height_at(world_origin, terrain_query);
    let position = Vec3::new(x, ground + CharacterDimensions::HALF_HEIGHT + 0.20, z);

    transform.translation = position;
    traveler.commit_position(position);
    velocity.0 = Vec3::ZERO;
    noclip.active = false;
    commands.entity(entity).insert(CharacterMotor);
}
