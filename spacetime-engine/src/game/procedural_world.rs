//! Bootstrap for the actual procedural game-world path.

use avian3d::prelude::LinearVelocity;
use bevy::{camera::visibility::NoFrustumCulling, prelude::*};

use crate::{
    game::{player::{Player, PlayerNoclip}, portal::PortalTraveler},
    physics::character::CharacterMotor,
    voxel::{
        VoxelBase, VoxelChunkCoord, VoxelChunkOf, VoxelWorld, empty_voxel_mesh,
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
            (spawn_procedural_world, prepare_player),
        );
    }
}

fn spawn_procedural_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let world_entity = commands
        .spawn((
            Name::new("Procedural World"),
            ProceduralWorldRoot,
            Transform::IDENTITY,
        ))
        .id();

    let material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.30, 0.27, 0.23),
        perceptual_roughness: 1.0,
        ..default()
    });
    let mut world = VoxelWorld::new(VoxelBase::terrain(0x10_0CA57));

    // M2 intentionally materializes a small window only. The procedural base
    // describes the rest of the implicit world without allocating chunks.
    for z in -1..=1 {
        for y in -1..=0 {
            for x in -1..=1 {
                let coord = VoxelChunkCoord::new(IVec3::new(x, y, z));
                let chunk = world.materialize_chunk(coord);
                let chunk_entity = commands
                    .spawn((
                        Name::new(format!("Procedural Chunk ({x}, {y}, {z})")),
                        VoxelChunkOf::new(world_entity, coord),
                        chunk,
                        Mesh3d(meshes.add(empty_voxel_mesh())),
                        MeshMaterial3d(material.clone()),
                        NoFrustumCulling,
                        Transform::IDENTITY,
                    ))
                    .id();

                commands.entity(world_entity).add_child(chunk_entity);
                assert!(world.insert_chunk(coord, chunk_entity).is_none());
            }
        }
    }

    commands.entity(world_entity).insert(world);
}

/// Voxel collision is deliberately not part of M2 yet. Start this map in the
/// existing developer noclip mode so the player can inspect/edit the procedural
/// field instead of falling through its currently render-only surface.
fn prepare_player(
    mut commands: Commands,
    player: Single<(
        Entity,
        &mut Transform,
        &mut PortalTraveler,
        &mut LinearVelocity,
        &mut PlayerNoclip,
    ), With<Player>>,
) {
    let (entity, mut transform, mut traveler, mut velocity, mut noclip) = player.into_inner();
    let position = Vec3::new(0.0, 8.0, 8.0);

    transform.translation = position;
    traveler.commit_position(position);
    velocity.0 = Vec3::ZERO;
    noclip.active = true;
    commands.entity(entity).remove::<CharacterMotor>();
}
