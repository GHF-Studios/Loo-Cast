//! Loo Cast game composition built on Spacetime Engine domains.
//!
//! ## Integration
//!
//! LooCastGamePlugin composes the game facilities and orders input, requests, physical resolution,
//! consequences, and presentation. Cross-domain changes use the owning facility’s request or
//! command contract.
//!
//! ## Module map
//!
//! - `combat`: Weapons, projectiles and manifestation-space hit detection.
//! - `console_commands`: Loo Cast commands layered on the generic developer console.
//! - `control`: Semantic and local runtime control authority.
//! - `devtools`: Loo Cast adapters into generic developer focus, inspection and tooling
//!   facilities.
//! - `flight`: Flight policy, contact and safety state, and derived telemetry.
//! - `health`: Generic damageable-state, damage and death semantics.
//! - `inventory`: Generic local inventory-selection state.
//! - `item`: Generic item identity, metadata, actions and presentation.
//! - `locomotion`: Generic controlled-subject locomotion.
//! - `navigation`: Generic controlled-subject navigation and travel policy.
//! - `orbit`: Canonical orbital mechanics shared by simulation, prediction and presentation.
//! - `player`: Local-player gameplay and presentation.
//! - `playground`: Reusable in-game test playground.
//! - `spacecraft`: Reference spacecraft built from generic USF/control primitives.
//! - `surface`: Read-only physical-surface proximity telemetry.
//! - `world`: Loo Cast scenario bootstrap, authored fixtures, and scenario lifetime.
//!
//! The plugin or configure entrypoint here wires the child systems and resources.
//!

pub mod combat;
mod console_commands;
pub mod control;
mod devtools;
pub mod flight;
pub mod health;
pub mod inventory;
pub mod item;
pub mod locomotion;
pub mod navigation;
pub mod orbit;
pub mod player;
pub mod playground;
pub mod spacecraft;
pub mod surface;
mod world;

use bevy::{
    app::{RunFixedMainLoop, RunFixedMainLoopSystems},
    prelude::*,
};

pub use devtools::LooCastDeveloperAdaptersPlugin;
pub use world::GameScenario;

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

pub struct LooCastGamePlugin;

impl Plugin for LooCastGamePlugin {
    fn build(&self, app: &mut App) {
        console_commands::configure(app);
        configure_game_schedule(app);
        install_game_facilities(app);
    }
}

/// The game composes ordering across engine domains here; child plugins own
/// their internal systems and never have to reproduce this cross-domain chain.
fn configure_game_schedule(app: &mut App) {
    app.configure_sets(
        RunFixedMainLoop,
        (
            control::ControlSet::Sample,
            flight::FlightSet::Control,
            orbit::OrbitalMechanicsSet::Propagate,
            crate::physics::gravity::GravitySet::Sample,
            navigation::NavigationSet::Observe,
            control::ControlSet::Request,
            navigation::NavigationSet::Plan,
            locomotion::LocomotionSet::Resolve,
            locomotion::LocomotionSet::Realize,
            crate::physics::character::CharacterEnvironmentSet::ResolveReferenceFrame,
            control::ControlSet::CharacterIntent,
            navigation::NavigationSet::Publish,
        )
            .chain()
            .in_set(RunFixedMainLoopSystems::BeforeFixedMainLoop),
    );

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
        crate::portal::PortalUpdateSet::DerivedViews.in_set(PresentationSet::DerivedViews),
    )
    .configure_sets(
        Update,
        crate::portal::PortalUpdateSet::Presentation.in_set(GameSet::Presentation),
    )
    .configure_sets(
        Update,
        (
            crate::thermal::ThermalSet::SpatialInput,
            crate::thermal::ThermalSet::Evolution,
            crate::thermal::ThermalSet::SpatialOutput,
        )
            .chain()
            .in_set(SimulationSet::Phenomena),
    )
    .configure_sets(
        Update,
        crate::thermal::ThermalPresentationSet::Derived.in_set(GameSet::Presentation),
    );
}

fn install_game_facilities(app: &mut App) {
    app.add_plugins((
        inventory::InventoryPlugin,
        item::ItemPlugin,
        health::HealthPlugin,
        combat::CombatPlugin,
        crate::procedural_assets::ProceduralPresentationAssetsPlugin,
        crate::spatial::UsfSpatialPlugin,
        control::ControlPlugin,
    ))
    .add_plugins((
        locomotion::LocomotionPlugin,
        orbit::OrbitalMechanicsPlugin,
        navigation::NavigationPlugin,
        surface::SurfacePlugin,
        flight::FlightPlugin,
        player::PlayerPlugin,
        spacecraft::SpacecraftPlugin,
        crate::portal::PortalPlugin,
        crate::thermal::ThermalCorePlugin,
        crate::thermal::ThermalPresentationPlugin,
        world::GameScenarioPlugin,
        playground::PlaygroundPlugin,
    ));
}
