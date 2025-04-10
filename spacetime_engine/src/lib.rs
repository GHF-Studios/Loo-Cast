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
//pub mod systems;

// Integrations

// Miscellaneous
//pub mod constants;
//pub mod decl_macros;
//pub mod singletons;
pub mod statics;
//pub mod traits;

// Modules
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
pub mod workflow;

use bevy::{app::PluginGroupBuilder, prelude::*};
use camera::CameraPlugin;
use iyes_perf_ui::{
    entries::{PerfUiFramerateEntries, PerfUiSystemEntries},
    prelude::{PerfUiEntryEntityCount, PerfUiRoot},
};
use spacetime_engine_macros::define_composite_workflow;
use workflow::{resources::WorkflowTypeModuleRegistry, WorkflowPlugin};
//use camera_2d_bundle::Camera2dBundlePlugin;
use chunk::ChunkPlugin;
//use chunk_actor::ChunkActorPlugin;
use chunk_loader::ChunkLoaderPlugin;
//use core::CorePlugin;
use debug::DebugPlugin;
use follower::FollowerPlugin;
use gpu::GpuPlugin;
//use entity::EntityPlugin;
//use math::MathPlugin;
use player::PlayerPlugin;
//use sprite_bundle::SpriteBundlePlugin;

pub struct SpacetimeEnginePlugins;
impl PluginGroup for SpacetimeEnginePlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(SpacetimeEngineCorePlugin)
            .add(WorkflowPlugin)
            .add(CameraPlugin)
            //.add(Camera2dBundlePlugin)
            .add(ChunkPlugin)
            //.add(ChunkActorPlugin)
            .add(ChunkLoaderPlugin)
            //.add(CorePlugin)
            .add(DebugPlugin)
            .add(FollowerPlugin)
            .add(GpuPlugin)
            //.add(EntityPlugin)
            //.add(MathPlugin)
            .add(PlayerPlugin)
        //.add(SpriteBundlePlugin)
    }
}

pub(crate) struct SpacetimeEngineCorePlugin;
impl Plugin for SpacetimeEngineCorePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(PreStartup, pre_startup_system)
            .add_systems(Startup, startup_system);
    }
}

fn pre_startup_system(
    mut workflow_type_module_registry: ResMut<WorkflowTypeModuleRegistry>
) {
    // --- Startup workflow framework ---
    crate::camera::workflows::camera::register_workflow_type_module(&mut workflow_type_module_registry);
    crate::chunk::workflows::chunk::register_workflow_type_module(&mut workflow_type_module_registry);
    crate::debug::workflows::debug::register_workflow_type_module(&mut workflow_type_module_registry);
    crate::gpu::workflows::gpu::register_workflow_type_module(&mut workflow_type_module_registry);
    crate::player::workflows::player::register_workflow_type_module(&mut workflow_type_module_registry);
}

fn startup_system() {
    define_composite_workflow!(Startup {
        //workflow!(Camera::SpawnMainCamera);
        //workflow!(Debug::SpawnDebugUI);
        //workflow!(Debug::SpawnDebugObjects);

        let chunk_shader_name = "texture_generators/example_compute_uv";
        let chunk_shader_path = "assets/shaders/texture_generators/example_compute_uv.wgsl".to_string();

        workflow!(IE, Gpu::SetupTextureGenerator, Input {
            shader_name: chunk_shader_name,
            shader_path: chunk_shader_path,
        });
        let generate_texture_output = workflow!(IOE, Gpu::GenerateTexture, Input {
            shader_name: chunk_shader_name,
            texture_size: crate::config::statics::CONFIG.get::<f32>("chunk/size") as usize,
            param_data: vec![0.0]
        });
        workflow!(IE, Chunk::SpawnChunk, Input {
            chunk_coord: (0, 0),
            chunk_owner: None,
            metric_texture: generate_texture_output.texture_handle,
        });

        Ok(())
    });

    crate::workflow::statics::COMPOSITE_WORKFLOW_RUNTIME.lock().unwrap().spawn_fallible(Box::pin(startup()));
}
