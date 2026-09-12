//! Minimal visible representation of the player.

use bevy::prelude::*;

use crate::physics::character::CharacterDimensions;

/// Marks presentation geometry belonging to the player.
#[derive(Component)]
pub struct PlayerModel;

/// Creates the deliberately boring reference model.
///
/// The reference mesh matches the standing collision hull so presentation does
/// not visually extend through geometry before the physical body reaches it.
pub fn create_model(
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) -> impl Bundle {
    (
        Name::new("Player Model"),
        PlayerModel,
        Mesh3d(meshes.add(Cuboid::new(
            CharacterDimensions::HULL_WIDTH,
            CharacterDimensions::HULL_HEIGHT,
            CharacterDimensions::HULL_WIDTH,
        ))),
        MeshMaterial3d(materials.add(Color::srgb(0.25, 0.45, 0.9))),
        Transform::IDENTITY,
        Visibility::Hidden,
    )
}
