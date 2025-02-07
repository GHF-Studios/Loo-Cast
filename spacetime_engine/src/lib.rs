#![allow(clippy::type_complexity)]
#![allow(clippy::too_many_arguments)]

// Data types
//pub mod components;
//pub mod enums;
//pub mod errors;
//pub mod structs;

// Functions
//pub mod commands;
//pub mod hooks;
pub mod oneshot_systems;
//pub mod systems;

// Integrations

// Miscellaneous
//pub mod constants;
//pub mod decl_macros;
//pub mod singletons;
//pub mod traits;

// Modules
pub mod action;
pub mod camera;
//pub mod camera_2d_bundle;
pub mod chunk;
pub mod chunk_actor;
pub mod chunk_loader;
pub mod config;
//pub mod core;
pub mod debug;
pub mod follower;
pub mod gpu;
//pub mod entity;
//pub mod math;
pub mod player;
//pub mod sprite_bundle;

use bevy::{app::PluginGroupBuilder, prelude::*};
use iyes_perf_ui::{entries::{PerfUiFramerateEntries, PerfUiSystemEntries}, prelude::{PerfUiEntryEntityCount, PerfUiRoot}};
use action::ActionPlugin;
use camera::CameraPlugin;
//use camera_2d_bundle::Camera2dBundlePlugin;
use chunk::ChunkPlugin;
//use chunk_actor::ChunkActorPlugin;
use chunk_loader::ChunkLoaderPlugin;
use oneshot_systems::MainOneshotSystems;
//use core::CorePlugin;
use debug::DebugPlugin;
use follower::FollowerPlugin;
//use entity::EntityPlugin;
//use math::MathPlugin;
use player::PlayerPlugin;
//use sprite_bundle::SpriteBundlePlugin;

pub struct SpacetimeEnginePlugins;
impl PluginGroup for SpacetimeEnginePlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(SpacetimeEngineCorePlugin)
            .add(ActionPlugin)
            .add(CameraPlugin)
            //.add(Camera2dBundlePlugin)
            .add(ChunkPlugin)
            //.add(ChunkActorPlugin)
            .add(ChunkLoaderPlugin)
            //.add(CorePlugin)
            .add(DebugPlugin)
            .add(FollowerPlugin)
            //.add(EntityPlugin)
            //.add(MathPlugin)
            .add(PlayerPlugin)
            //.add(SpriteBundlePlugin)
    }
}

pub(in crate) struct SpacetimeEngineCorePlugin;
impl Plugin for SpacetimeEngineCorePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<MainOneshotSystems>()
            .add_systems(PreStartup, pre_startup_system)
            .add_systems(Startup, startup_system);
    }
}

fn pre_startup_system(
    mut commands: Commands,
    oneshot_systems: Res<MainOneshotSystems>,
) {
    let id = oneshot_systems.0["initialize_action_type_modules"];
    commands.run_system(id);

    let id = oneshot_systems.0["spawn_main_camera"];
    commands.run_system(id);
    
    commands.spawn((
        PerfUiRoot::default(),
        PerfUiFramerateEntries::default(),
        PerfUiSystemEntries::default(),
        PerfUiEntryEntityCount::default(),
        // ...
    ));

    //let id = oneshot_systems.0["spawn_main_test_objects"];
    //commands.run_system(id);

    let id = oneshot_systems.0["test_action_framework"];
    commands.run_system(id);
}

fn startup_system() {
    
}
