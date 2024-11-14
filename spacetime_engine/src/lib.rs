// Data types
mod structs;
mod wrappers;

// Functions

// Integrations

// Miscelaneous
mod constants;

pub mod camera;
pub mod camera_2d_bundle;
pub mod chunk;
pub mod chunk_actor;
pub mod chunk_loader;
pub mod command;
pub mod core;
pub mod entity;
pub mod math;
pub mod operation;
pub mod player;
pub mod sprite_bundle;

use bevy::{app::PluginGroupBuilder, prelude::*};
use camera_2d_bundle::structs::Camera2DBundle;
use chunk::components::Chunk;
use chunk_actor::components::ChunkActor;
use chunk_loader::components::ChunkLoader;
use command::structs::Command;
use constants::*;
use operation::structs::Operation;
use player::components::Player;
use core::constants::*;
use std::any::Any;
use core::structs::*;
use core::traits::*;
use std::any::TypeId;
use std::collections::*;
use std::fmt::{Debug, Display};
use std::hash::*;

use camera::CameraPlugin;
use camera_2d_bundle::Camera2dBundlePlugin;
use chunk::ChunkPlugin;
use chunk_actor::ChunkActorPlugin;
use chunk_loader::ChunkLoaderPlugin;
use command::CommandPlugin;
use core::CorePlugin;
use entity::EntityPlugin;
use math::MathPlugin;
use operation::OperationPlugin;
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
            .add(CommandPlugin)	
            .add(CorePlugin)
            .add(EntityPlugin)
            .add(MathPlugin)
            .add(OperationPlugin)
            .add(PlayerPlugin)
            .add(SpriteBundlePlugin)
    }
}