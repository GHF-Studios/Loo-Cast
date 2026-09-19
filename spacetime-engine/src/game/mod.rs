//! Small test game used to pressure-test Spacetime Engine semantics.

pub mod combat;
mod devtools;
mod environment;
pub mod inventory;
pub mod item;
pub mod map_selection;
pub mod player;
pub mod playground;
pub mod portal;
mod procedural_world;
mod scale_stack;
pub mod thermal;

use bevy::prelude::*;

use combat::{Damage, Died, FireWeapon, Hit};

pub use devtools::TestGameDeveloperToolsPlugin;

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
                environment::EnvironmentLightingPlugin,
                inventory::InventoryPlugin,
                item::ItemPlugin,
                map_selection::MapSelectionPlugin,
                combat::CombatPlugin,
                crate::procedural_assets::ProceduralAssetsPlugin,
                crate::spatial::UsfSpatialPlugin,
                crate::worldgen::WorldGenerationPlugin,
                player::PlayerPlugin,
                portal::PortalPlugin,
                thermal::ThermalPlugin,
                procedural_world::ProceduralWorldPlugin,
                playground::PlaygroundPlugin,
            ));
    }
}
