pub mod core;
pub mod chunk;
pub mod chunk_actor;
pub mod chunk_loader;
pub mod entity;
pub mod math;
pub mod operations;
pub mod player;
pub mod sprite_bundle;

use bevy::{app::PluginGroupBuilder, prelude::*};
use core::CorePlugin;
use chunk::ChunkPlugin;
use chunk_actor::ChunkActorPlugin;
use chunk_loader::ChunkLoaderPlugin;
use entity::EntityPlugin;
use math::MathPlugin;
use operations::OperationsPlugin;
use player::PlayerPlugin;
use sprite_bundle::SpriteBundlePlugin;

pub struct SpacetimeEnginePlugins;

impl PluginGroup for SpacetimeEnginePlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(CorePlugin)
            .add(ChunkPlugin)
            .add(ChunkActorPlugin)
            .add(ChunkLoaderPlugin)
            .add(EntityPlugin)
            .add(MathPlugin)
            .add(OperationsPlugin)
            .add(PlayerPlugin)
            .add(SpriteBundlePlugin)
    }
}