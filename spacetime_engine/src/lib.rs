pub mod camera;
pub mod chunk;
pub mod entity;
pub mod follower;
pub mod math;
pub mod player;

use camera::CameraPlugin;
use chunk::ChunkPlugin;
use entity::EntityPlugin;
use follower::FollowerPlugin;
use math::MathPlugin;
use player::PlayerPlugin;
use bevy::{app::PluginGroupBuilder, prelude::*};

pub struct SpacetimeEnginePlugins;

impl PluginGroup for SpacetimeEnginePlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(CameraPlugin)
            .add(ChunkPlugin)
            .add(EntityPlugin)
            .add(FollowerPlugin)
            .add(MathPlugin)
            .add(PlayerPlugin)
    }
}