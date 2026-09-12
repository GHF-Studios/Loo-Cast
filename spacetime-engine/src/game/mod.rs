//! Small test game used to pressure-test Spacetime Engine semantics.

pub mod combat;
pub mod player;
pub mod playground;
pub mod portal;
pub mod thermal;

use bevy::prelude::*;

use combat::{Damage, Died, FireWeapon, Hit};

/// Stable top-level extension points.
#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameSet {
    Input,
    Action,
    Simulation,
    Consequence,
    Cleanup,
    Presentation,
}

/// Ordered structure inside [`GameSet::Input`].
#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InputSet {
    Interface,
    Cursor,
    Gameplay,
}

/// Internal structure of physical simulation.
#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SimulationSet {
    Motion,
    Topology,
    Collision,
    /// Environmental/systemic phenomena evaluated after physical collision.
    Phenomena,
}

/// Ordering inside presentation.
#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PresentationSet {
    PrimaryView,
    DerivedViews,
}

pub struct TestGamePlugin;

impl Plugin for TestGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<FireWeapon>()
            .add_message::<Hit>()
            .add_message::<Damage>()
            .add_message::<Died>()
            .configure_sets(
                Update,
                (
                    GameSet::Input,
                    GameSet::Action,
                    GameSet::Simulation,
                    GameSet::Consequence,
                    GameSet::Cleanup,
                    GameSet::Presentation,
                )
                    .chain(),
            )
            .configure_sets(
                Update,
                (InputSet::Interface, InputSet::Cursor, InputSet::Gameplay)
                    .chain()
                    .in_set(GameSet::Input),
            )
            .configure_sets(
                Update,
                (
                    SimulationSet::Motion,
                    SimulationSet::Topology,
                    SimulationSet::Collision,
                    SimulationSet::Phenomena,
                )
                    .chain()
                    .in_set(GameSet::Simulation),
            )
            .configure_sets(
                Update,
                (PresentationSet::PrimaryView, PresentationSet::DerivedViews)
                    .chain()
                    .in_set(GameSet::Presentation),
            )
            .add_plugins((
                combat::CombatPlugin,
                player::PlayerPlugin,
                portal::PortalPlugin,
                thermal::ThermalPlugin,
                playground::PlaygroundPlugin,
            ))
            .add_systems(Startup, setup_scene);
    }
}

#[derive(Resource)]
pub(crate) struct GameAssets {
    pub damageable_cube_mesh: Handle<Mesh>,
    pub damageable_cube_material: Handle<StandardMaterial>,
    pub projectile_mesh: Handle<Mesh>,
    pub projectile_material: Handle<StandardMaterial>,
}

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.insert_resource(GameAssets {
        damageable_cube_mesh: meshes.add(Cuboid::from_length(1.0)),
        damageable_cube_material: materials.add(Color::srgb(0.8, 0.2, 0.2)),
        projectile_mesh: meshes.add(Sphere::new(0.1)),
        projectile_material: materials.add(Color::WHITE),
    });
}
