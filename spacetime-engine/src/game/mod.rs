//! Small test game used to pressure-test Spacetime Engine semantics.

pub mod combat;
pub mod player;
pub mod playground;
pub mod portal;

use avian3d::prelude::{Collider, RigidBody};
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
                (
                    InputSet::Interface,
                    InputSet::Cursor,
                    InputSet::Gameplay,
                )
                    .chain()
                    .in_set(GameSet::Input),
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

    commands.spawn((
        Name::new("Playground Floor"),
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
        RigidBody::Static,
        Collider::cuboid(50.0, 0.1, 50.0),
        Transform::from_xyz(0.0, -0.05, 0.0),
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
