//! Shared presentation assets for concrete playground cube items.

use bevy::prelude::*;

#[derive(Resource)]
pub(super) struct PlaygroundItemPresentationAssets {
    pub(super) cube_material: Handle<StandardMaterial>,
}

pub(super) fn setup_item_assets(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.insert_resource(PlaygroundItemPresentationAssets {
        cube_material: materials.add(Color::srgb(0.8, 0.2, 0.2)),
    });
}
