//! Minimal visible representation of the player.

use bevy::{camera::visibility::RenderLayers, prelude::*};

use crate::{
    portal::{DERIVED_VIEW_LAYER, PortalSplitVisual},
    physics::character::CharacterDimensions,
    spatial::{SpatialScale, UsfLocalScalePresentation},
};

/// Marks presentation geometry belonging to the player.
#[derive(Component)]
pub(super) struct PlayerModel;

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
        PortalSplitVisual,
        UsfLocalScalePresentation::new(SpatialScale::MAX),
        Mesh3d(meshes.add(Cuboid::new(
            CharacterDimensions::HULL_WIDTH,
            CharacterDimensions::HULL_HEIGHT,
            CharacterDimensions::HULL_WIDTH,
        ))),
        MeshMaterial3d(materials.add(Color::srgb(0.25, 0.45, 0.9))),
        Transform::IDENTITY,
        // First-person is the default camera mode. Keep the world model alive,
        // but place it on the derived-view-only layer until presentation sync
        // switches it back to the ordinary world layer in third person.
        RenderLayers::layer(DERIVED_VIEW_LAYER),
        Visibility::Inherited,
    )
}
