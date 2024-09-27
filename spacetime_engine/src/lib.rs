pub mod core;
pub mod chunk;
pub mod chunk_actor;
pub mod chunk_loader;
pub mod entity;
pub mod math;
pub mod operations;
pub mod player;

use core::CorePlugin;
use chunk::ChunkPlugin;
use chunk_actor::ChunkActorPlugin;
use chunk_loader::ChunkLoaderPlugin;
use entity::EntityPlugin;
use math::MathPlugin;
use operations::OperationsPlugin;
use player::PlayerPlugin;
use bevy::{app::PluginGroupBuilder, prelude::*};

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
    }
}