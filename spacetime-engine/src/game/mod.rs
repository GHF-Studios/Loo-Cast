//! Small test game used to pressure-test Spacetime Engine semantics.

pub mod combat;
mod devtools;
mod environment;
pub mod health;
pub mod inventory;
pub mod item;
pub mod map_selection;
pub mod player;
pub mod playground;
mod procedural_world;

use bevy::prelude::*;

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
        app.configure_sets(
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
            .configure_sets(
                Update,
                crate::portal::PortalUpdateSet::Topology.in_set(SimulationSet::Topology),
            )
            .configure_sets(
                Update,
                crate::portal::PortalUpdateSet::DerivedViews
                    .in_set(PresentationSet::DerivedViews),
            )
            .configure_sets(
                Update,
                crate::portal::PortalUpdateSet::Presentation.in_set(GameSet::Presentation),
            )
            .configure_sets(
                Update,
                (
                    crate::thermal::ThermalSet::SpatialInput,
                    crate::thermal::ThermalSet::Lumped,
                    crate::thermal::ThermalSet::SpatialOutput,
                )
                    .chain()
                    .in_set(SimulationSet::Phenomena),
            )
            .configure_sets(
                Update,
                crate::thermal::ThermalPresentationSet::Derived
                    .in_set(GameSet::Presentation),
            )
            .add_plugins((
                environment::EnvironmentLightingPlugin,
                inventory::InventoryPlugin,
                item::ItemPlugin,
                map_selection::MapSelectionPlugin,
                health::HealthPlugin,
                combat::CombatPlugin,
                crate::procedural_assets::ProceduralAssetsPlugin,
                crate::spatial::UsfSpatialPlugin,
                crate::worldgen::WorldGenerationPlugin,
                player::PlayerPlugin,
                crate::portal::PortalPlugin,
                crate::thermal::ThermalCorePlugin,
                crate::thermal::ThermalPresentationPlugin,
                procedural_world::ProceduralWorldPlugin,
                playground::PlaygroundPlugin,
            ));
    }
}
