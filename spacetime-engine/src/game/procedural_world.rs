//! Bootstrap for the actual procedural game-world path.

use avian3d::prelude::LinearVelocity;
use bevy::prelude::*;

use crate::{
    game::{player::{Player, PlayerNoclip}, portal::PortalTraveler},
    physics::character::CharacterMotor,
    voxel::{VoxelBase, VoxelStreaming, VoxelWorld},
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
    mut materials: ResMut<Assets<StandardMaterial>>,
    player: Single<Entity, With<Player>>,
) {
    let material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.30, 0.27, 0.23),
        perceptual_roughness: 1.0,
        ..default()
    });

    commands.spawn((
        Name::new("Procedural World"),
        ProceduralWorldRoot,
        VoxelWorld::new(VoxelBase::terrain(0x10_0CA57)),
        // Two chunks horizontally cover the Voxel Hand's 64 m reach. Keep one
        // chunk vertically above/below the viewer so terrain remains available
        // while noclipping over modest elevation changes. Fresh work is spread
        // across frames rather than generating the full window at once.
        VoxelStreaming::new(
            player.into_inner(),
            IVec3::new(2, 1, 2),
            8,
            material,
        ),
        Transform::IDENTITY,
    ));
}

/// Voxel collision is deliberately not part of M3 yet. Start this map in the
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
