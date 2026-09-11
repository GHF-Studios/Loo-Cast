//! Minimal visible representation of the player.

use bevy::prelude::*;

/// Marks presentation geometry belonging to the player.
#[derive(Component)]
pub struct PlayerModel;

/// Creates the deliberately boring reference model.
///
/// It is an actual 1×1×1 cube so portal projection problems are visually
/// obvious instead of being confused with non-uniform model dimensions.
pub fn create_model(
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) -> impl Bundle {
    (
        Name::new("Player Model"),
        PlayerModel,
        Mesh3d(meshes.add(Cuboid::from_length(1.0))),
        MeshMaterial3d(
            materials.add(Color::srgb(0.25, 0.45, 0.9)),
        ),

        // The gameplay transform represents roughly eye height.
        Transform::from_xyz(0.0, -0.75, 0.0),
    )
}
