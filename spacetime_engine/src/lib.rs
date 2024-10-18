pub mod camera;
pub mod camera_2d_bundle;
pub mod chunk;
pub mod chunk_actor;
pub mod chunk_loader;
pub mod commands;
pub mod core;
pub mod entity;
pub mod math;
pub mod operations;
pub mod player;
pub mod sprite_bundle;

use bevy::{app::PluginGroupBuilder, prelude::*};
use camera::CameraPlugin;
use camera_2d_bundle::Camera2dBundlePlugin;
use chunk::ChunkPlugin;
use chunk_actor::ChunkActorPlugin;
use chunk_loader::ChunkLoaderPlugin;
use commands::CommandsPlugin;
use core::CorePlugin;
use entity::EntityPlugin;
use math::MathPlugin;
use operations::OperationsPlugin;
use player::PlayerPlugin;
use sprite_bundle::SpriteBundlePlugin;

pub struct SpacetimeEnginePlugins;

impl PluginGroup for SpacetimeEnginePlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(CameraPlugin)
            .add(Camera2dBundlePlugin)
            .add(ChunkPlugin)
            .add(ChunkActorPlugin)
            .add(ChunkLoaderPlugin)
            .add(CommandsPlugin)	
            .add(CorePlugin)
            .add(EntityPlugin)
            .add(MathPlugin)
            .add(OperationsPlugin)
            .add(PlayerPlugin)
            .add(SpriteBundlePlugin)
    }
}