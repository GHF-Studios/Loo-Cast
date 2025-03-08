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
use iyes_perf_ui::{entries::{PerfUiFramerateEntries, PerfUiSystemEntries}, prelude::{PerfUiEntryEntityCount, PerfUiRoot}};
use workflow::WorkflowPlugin;
use camera::CameraPlugin;
//use camera_2d_bundle::Camera2dBundlePlugin;
use chunk::ChunkPlugin;
//use chunk_actor::ChunkActorPlugin;
use chunk_loader::ChunkLoaderPlugin;
use oneshot_systems::MainOneshotSystems;
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

pub(in crate) struct SpacetimeEngineCorePlugin;
impl Plugin for SpacetimeEngineCorePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<MainOneshotSystems>()
            .add_systems(PreStartup, pre_startup_system)
            .add_systems(Startup, startup_system)
            //.add_systems(PostStartup, post_startup_system)
            ;
    }
}

fn pre_startup_system(
    mut commands: Commands,
    oneshot_systems: Res<MainOneshotSystems>,
) {
    let id = oneshot_systems.0["initialize_workflow_type_modules"];
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

    let id = oneshot_systems.0["test_workflow_framework"];
    commands.run_system(id);
}

fn startup_system() {
    
}

fn post_startup_system() {
    define_composite_workflow!(TestWorkflowFramework {
        let shader_name = "texture_generators/example_compute_uv";
        let shader_path = "assets/shaders/texture_generators/example_compute_uv.wgsl";

        run_workflow!(Gpu, SetupTextureGenerator, Input {
            shader_name,
            shader_path: shader_path.to_string(),
        });
        
        let generate_texture_output = run_workflow!(Gpu, GenerateTexture, Input {
            shader_name,
            texture_size: crate::config::statics::CONFIG.get::<f32>("chunk/size") as usize
        });
    
        run_workflow!(Chunk, SpawnChunk, Input {
            chunk_coord: (0, 0),
            chunk_owner: None,
            metric_texture: generate_texture_output.texture_handle,
        });
    
        Ok(())
    })
    
    crate::workflow::statics::COMPOSITE_WORKFLOW_RUNTIME.lock().unwrap().spawn_composite_workflow(Box::pin(test_workflow_framework()));
}

fn post_startup_system_expanded() {
    use thiserror::Error;

    #[derive(Debug, Error)]
    pub enum TestWorkflowFrameworkError {
        #[error("SetupTextureGeneratorError{0}")]
        SetupTextureGeneratorError(<crate::gpu::vorkflows::gpu::setup_texture_generator::Type as workflow::traits::WorkflowTypeIE>::Error),

        #[error("GenerateTextureError{0}")]
        GenerateTextureError(<crate::gpu::vorkflows::gpu::generate_texture::Type as workflow::traits::WorkflowTypeIOE>::Error),

        #[error("SpawnChunkError{0}")]
        SpawnChunkError(<crate::chunk::vorkflows::chunk::spawn_chunk::Type as workflow::traits::WorkflowTypeIE>::Error),
    }
    impl std::convert::From<gpu::vorkflows::gpu::generate_texture::Error> for TestWorkflowFrameworkError {
        fn from(e: gpu::vorkflows::gpu::generate_texture::Error) -> Self {
            Self::GenerateTextureError(e)
        }
    }
    impl std::convert::From<gpu::vorkflows::gpu::setup_texture_generator::Error> for TestWorkflowFrameworkError {
        fn from(e: gpu::vorkflows::gpu::setup_texture_generator::Error) -> Self {
            Self::SetupTextureGeneratorError(e)
        }
    }
    impl std::convert::From<chunk::vorkflows::chunk::spawn_chunk::Error> for TestWorkflowFrameworkError {
        fn from(e: chunk::vorkflows::chunk::spawn_chunk::Error) -> Self {
            Self::SpawnChunkError(e)
        }
    }

    pub async fn test_workflow_framework() -> Result<(), TestWorkflowFrameworkError> {
        let shader_name = "texture_generators/example_compute_uv";
        let shader_path = "assets/shaders/texture_generators/example_compute_uv.wgsl";

        {
            type T = crate::gpu::vorkflows::gpu::setup_texture_generator::Type;
            type I = <T as crate::workflow::traits::WorkflowTypeIE>::Input;
            crate::workflow::functions::run_workflow_ie::<T>(I {
                shader_name,
                shader_path: shader_path.to_string(),
            }).await.map_err(Into::<TestWorkflowFrameworkError>::into)
        }?;

        let generate_texture_output = {
            type T = crate::gpu::vorkflows::gpu::generate_texture::Type;
            type I = <T as crate::workflow::traits::WorkflowTypeIOE>::Input;
            crate::workflow::functions::run_workflow_ioe::<T>(I {
                shader_name,
                texture_size: crate::config::statics::CONFIG.get::<f32>("chunk/size") as usize,
                param_data: vec![]
            }).await.map_err(Into::<TestWorkflowFrameworkError>::into)
        }?;

        {
            type T = crate::chunk::vorkflows::chunk::spawn_chunk::Type;
            type I = <T as crate::workflow::traits::WorkflowTypeIE>::Input;
            crate::workflow::functions::run_workflow_ie::<T>(I {
                chunk_coord: (0, 0),
                chunk_owner: None,
                metric_texture: generate_texture_output.texture_handle,
            }).await.map_err(Into::<TestWorkflowFrameworkError>::into)
        }?;

        Ok(())
    }
    
    crate::workflow::statics::COMPOSITE_WORKFLOW_RUNTIME.lock().unwrap().spawn_composite_workflow(Box::pin(test_workflow_framework()));
}
