// Data types
pub mod components;
pub mod enums;
pub mod errors;
pub mod structs;

// Functions
pub mod commands;
pub mod hooks;
pub mod systems;

// Integrations

// Miscellaneous
pub mod constants;
pub mod decl_macros;
pub mod singletons;
pub mod traits;

// Modules
pub mod camera;
//pub mod camera_2d_bundle;
//pub mod chunk;
//pub mod chunk_actor;
//pub mod chunk_loader;
//pub mod core;
//pub mod entity;
//pub mod math;
//pub mod player;
//pub mod sprite_bundle;

use bevy::{app::PluginGroupBuilder, prelude::*};
use camera::CameraPlugin;
//use camera_2d_bundle::Camera2dBundlePlugin;
//use chunk::ChunkPlugin;
//use chunk_actor::ChunkActorPlugin;
//use chunk_loader::ChunkLoaderPlugin;
//use core::CorePlugin;
//use entity::EntityPlugin;
//use math::MathPlugin;
//use player::PlayerPlugin;
//use sprite_bundle::SpriteBundlePlugin;

pub struct SpacetimeEnginePlugins;
impl PluginGroup for SpacetimeEnginePlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(SpacetimeEngineCorePlugin)
            .add(CameraPlugin)
            //.add(Camera2dBundlePlugin)
            //.add(ChunkPlugin)
            //.add(ChunkActorPlugin)
            //.add(ChunkLoaderPlugin)
            //.add(CorePlugin)
            //.add(EntityPlugin)
            //.add(MathPlugin)
            //.add(PlayerPlugin)
            //.add(SpriteBundlePlugin)
    }
}

pub(in crate) struct SpacetimeEngineCorePlugin;
impl Plugin for SpacetimeEngineCorePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(PreStartup, systems::pre_startup)
            .add_systems(Startup, systems::startup)
            .add_systems(PostUpdate, systems::post_update);
    }
}