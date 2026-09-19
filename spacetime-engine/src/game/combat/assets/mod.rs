//! Presentation assets used by ordinary combat projectiles.

use bevy::prelude::*;

#[derive(Resource)]
pub(super) struct CombatPresentationAssets {
    pub(super) projectile_mesh: Handle<Mesh>,
    pub(super) projectile_material: Handle<StandardMaterial>,
}

pub(super) fn setup_combat_assets(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.insert_resource(CombatPresentationAssets {
        projectile_mesh: meshes.add(Sphere::new(0.1)),
        projectile_material: materials.add(Color::WHITE),
    });
}
