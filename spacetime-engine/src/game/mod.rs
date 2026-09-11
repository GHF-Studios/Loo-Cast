//! Small test game used to pressure-test Spacetime Engine semantics.

pub mod combat;
pub mod player;
pub mod portal;
pub mod target;
pub mod ui;

use bevy::prelude::*;

use combat::{Damage, Died, FireWeapon, Hit};
use target::SpawnTarget;

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

/// Internal structure of physical simulation.
///
/// Topological remapping deliberately happens after motion but before
/// collision:
///
/// `Motion -> Topology -> Collision`
#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SimulationSet {
    Motion,
    Topology,
    Collision,
}

/// Ordering inside presentation.
///
/// The primary camera is resolved before views derived from it, such as portal
/// render cameras.
#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PresentationSet {
    PrimaryView,
    DerivedViews,
}

pub struct TestGamePlugin;

impl Plugin for TestGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<SpawnTarget>()
            .add_message::<FireWeapon>()
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
                (
                    SimulationSet::Motion,
                    SimulationSet::Topology,
                    SimulationSet::Collision,
                )
                    .chain()
                    .in_set(GameSet::Simulation),
            )
            .configure_sets(
                Update,
                (
                    PresentationSet::PrimaryView,
                    PresentationSet::DerivedViews,
                )
                    .chain()
                    .in_set(GameSet::Presentation),
            )
            .add_plugins((
                combat::CombatPlugin,
                player::PlayerPlugin,
                portal::PortalPlugin,
                target::TargetPlugin,
                ui::HudPlugin,
            ))
            .add_systems(Startup, setup_scene);
    }
}

#[derive(Resource)]
pub(crate) struct GameAssets {
    pub target_mesh: Handle<Mesh>,
    pub target_material: Handle<StandardMaterial>,
    pub projectile_mesh: Handle<Mesh>,
    pub projectile_material: Handle<StandardMaterial>,
}

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.insert_resource(GameAssets {
        target_mesh: meshes.add(Cuboid::from_length(2.0)),
        target_material: materials.add(Color::srgb(0.8, 0.2, 0.2)),
        projectile_mesh: meshes.add(Sphere::new(0.1)),
        projectile_material: materials.add(Color::WHITE),
    });

    commands.spawn((
        Mesh3d(
            meshes.add(
                Plane3d::default()
                    .mesh()
                    .size(50.0, 50.0),
            ),
        ),
        MeshMaterial3d(
            materials.add(Color::srgb(0.15, 0.15, 0.15)),
        ),
    ));

    commands.spawn((
        PointLight {
            intensity: 2_000_000.0,
            shadow_maps_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));
}
