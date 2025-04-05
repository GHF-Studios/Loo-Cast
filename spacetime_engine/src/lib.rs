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
use camera::CameraPlugin;
use iyes_perf_ui::{
    entries::{PerfUiFramerateEntries, PerfUiSystemEntries},
    prelude::{PerfUiEntryEntityCount, PerfUiRoot},
};
use spacetime_engine_macros::{get_workflow_path, run_workflow};
use workflow::{resources::WorkflowTypeModuleRegistry, WorkflowPlugin};
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

pub(crate) struct SpacetimeEngineCorePlugin;
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
    mut workflow_type_module_registry: ResMut<WorkflowTypeModuleRegistry>,
) {
    // --- Spawn essential entities ---
    let id = oneshot_systems.0["spawn_main_camera"];
    commands.run_system(id);

    // TODO: FUN: Convert this to a workflow
    commands.spawn((
        PerfUiRoot::default(),
        PerfUiFramerateEntries::default(),
        PerfUiSystemEntries::default(),
        PerfUiEntryEntityCount::default(),
        // ...
    ));

    //let id = oneshot_systems.0["spawn_main_test_objects"];
    //commands.run_system(id);

    // --- Initialize workflow framework ---
    crate::chunk::workflows::chunk::register_workflow_type_module(&mut workflow_type_module_registry);
    crate::gpu::workflows::gpu::register_workflow_type_module(&mut workflow_type_module_registry);
}

// --- Fully expanded oneshot composite workflow `test_workflow_framework` ---
fn startup_system() {
    use thiserror::Error;

    #[derive(Debug, Error)]
    pub enum TestWorkflowFrameworkError {
        #[error("SetupTextureGeneratorError{0}")]
        SetupTextureGeneratorError(<crate::gpu::workflows::gpu::setup_texture_generator::TypeIE as workflow::traits::WorkflowTypeIE>::Error),

        #[error("GenerateTextureError{0}")]
        GenerateTextureError(<crate::gpu::workflows::gpu::generate_texture::TypeIOE as workflow::traits::WorkflowTypeIOE>::Error),

        #[error("SpawnChunkError{0}")]
        SpawnChunkError(<crate::chunk::workflows::chunk::spawn_chunk::TypeIE as workflow::traits::WorkflowTypeIE>::Error),
    }
    impl std::convert::From<gpu::workflows::gpu::generate_texture::Error>
        for TestWorkflowFrameworkError
    {
        fn from(e: gpu::workflows::gpu::generate_texture::Error) -> Self {
            Self::GenerateTextureError(e)
        }
    }
    impl std::convert::From<gpu::workflows::gpu::setup_texture_generator::Error>
        for TestWorkflowFrameworkError
    {
        fn from(e: gpu::workflows::gpu::setup_texture_generator::Error) -> Self {
            Self::SetupTextureGeneratorError(e)
        }
    }
    impl std::convert::From<chunk::workflows::chunk::spawn_chunk::Error>
        for TestWorkflowFrameworkError
    {
        fn from(e: chunk::workflows::chunk::spawn_chunk::Error) -> Self {
            Self::SpawnChunkError(e)
        }
    }

    pub async fn test_workflow_framework() -> Result<(), TestWorkflowFrameworkError> {
        let shader_name = "texture_generators/example_compute_uv";
        let shader_path = "assets/shaders/texture_generators/example_compute_uv.wgsl";

        {
            type T = crate::gpu::workflows::gpu::setup_texture_generator::TypeIE;
            type I = <T as crate::workflow::traits::WorkflowTypeIE>::Input;

            crate::gpu::workflows::gpu::setup_texture_generator::run(I {
                shader_name,
                shader_path: shader_path.to_string(),
            })
            .await
            .map_err(Into::<TestWorkflowFrameworkError>::into)
        }?;

        let generate_texture_output = {
            type T = crate::gpu::workflows::gpu::generate_texture::TypeIOE;
            type I = <T as crate::workflow::traits::WorkflowTypeIOE>::Input;

            crate::gpu::workflows::gpu::generate_texture::run(I {
                shader_name,
                texture_size: crate::config::statics::CONFIG.get::<f32>("chunk/size") as usize,
                param_data: vec![0.0],
            })
            .await
            .map_err(Into::<TestWorkflowFrameworkError>::into)
        }?;

        {
            type T = crate::chunk::workflows::chunk::spawn_chunk::TypeIE;
            type I = <T as crate::workflow::traits::WorkflowTypeIE>::Input;

            crate::chunk::workflows::chunk::spawn_chunk::run(I {
                chunk_coord: (0, 0),
                chunk_owner: None,
                metric_texture: generate_texture_output.texture_handle,
            })
            .await
            .map_err(Into::<TestWorkflowFrameworkError>::into)
        }?;

        Ok(())
    }

    crate::workflow::statics::COMPOSITE_WORKFLOW_RUNTIME
        .lock()
        .unwrap()
        .spawn_fallible(Box::pin(test_workflow_framework()));
}

//fn post_startup_system() {
//    define_composite_workflow!(TestWorkflowFramework {
//        let shader_name = "texture_generators/example_compute_uv";
//        let shader_path = "assets/shaders/texture_generators/example_compute_uv.wgsl";
//
//        run_workflow!(get_workflow_path!(Gpu::SetupTextureGenerator, IE), Input {
//            shader_name,
//            shader_path: shader_path.to_string(),
//        });
//
//        let generate_texture_output = run_workflow!(get_workflow_path!(Gpu::GenerateTexture, IOE), Input {
//            shader_name,
//            texture_size: crate::config::statics::CONFIG.get::<f32>("chunk/size") as usize,
//            param_data: vec![0.0]
//        });
//
//        run_workflow!(get_workflow_path!(Chunk::SpawnChunk, IE), Input {
//            chunk_coord: (0, 0),
//            chunk_owner: None,
//            metric_texture: generate_texture_output.texture_handle,
//        });
//
//        Ok(())
//    })
//
//    crate::workflow::statics::COMPOSITE_WORKFLOW_RUNTIME.lock().unwrap().spawn_fallible(Box::pin(test_workflow_framework()));
//}

//fn post_startup_system_outter_expanded() {
//    use thiserror::Error;
//
//    #[derive(Debug, Error)]
//    pub enum TestWorkflowFrameworkError {
//        #[error("SetupTextureGeneratorError{0}")]
//        SetupTextureGeneratorError(<crate::gpu::workflows::gpu::setup_texture_generator::TypeIE as workflow::traits::WorkflowTypeIE>::Error),
//
//        #[error("GenerateTextureError{0}")]
//        GenerateTextureError(<crate::gpu::workflows::gpu::generate_texture::TypeIOE as workflow::traits::WorkflowTypeIOE>::Error),
//
//        #[error("SpawnChunkError{0}")]
//        SpawnChunkError(<crate::chunk::workflows::chunk::spawn_chunk::TypeIE as workflow::traits::WorkflowTypeIE>::Error),
//    }
//    impl std::convert::From<gpu::workflows::gpu::generate_texture::Error> for TestWorkflowFrameworkError {
//        fn from(e: gpu::workflows::gpu::generate_texture::Error) -> Self {
//            Self::GenerateTextureError(e)
//        }
//    }
//    impl std::convert::From<gpu::workflows::gpu::setup_texture_generator::Error> for TestWorkflowFrameworkError {
//        fn from(e: gpu::workflows::gpu::setup_texture_generator::Error) -> Self {
//            Self::SetupTextureGeneratorError(e)
//        }
//    }
//    impl std::convert::From<chunk::workflows::chunk::spawn_chunk::Error> for TestWorkflowFrameworkError {
//        fn from(e: chunk::workflows::chunk::spawn_chunk::Error) -> Self {
//            Self::SpawnChunkError(e)
//        }
//    }
//
//    pub async fn test_workflow_framework() -> Result<(), TestWorkflowFrameworkError> {
//        let shader_name = "texture_generators/example_compute_uv";
//        let shader_path = "assets/shaders/texture_generators/example_compute_uv.wgsl";
//
//        run_workflow!(get_workflow_path!(Gpu::SetupTextureGenerator, IE), Input {
//            shader_name,
//            shader_path: shader_path.to_string(),
//        });
//
//        let generate_texture_output = run_workflow!(get_workflow_path!(Gpu::GenerateTexture, IOE), Input {
//            shader_name,
//            texture_size: crate::config::statics::CONFIG.get::<f32>("chunk/size") as usize,
//            param_data: vec![0.0]
//        });
//
//        run_workflow!(get_workflow_path!(Chunk::SpawnChunk, IE), Input {
//            chunk_coord: (0, 0),
//            chunk_owner: None,
//            metric_texture: generate_texture_output.texture_handle,
//        });
//
//        Ok(())
//    }
//
//    crate::workflow::statics::COMPOSITE_WORKFLOW_RUNTIME.lock().unwrap().spawn_fallible(Box::pin(test_workflow_framework()));
//}

//fn post_startup_system_full_expanded() {
//    use thiserror::Error;
//
//    #[derive(Debug, Error)]
//    pub enum TestWorkflowFrameworkError {
//        #[error("SetupTextureGeneratorError{0}")]
//        SetupTextureGeneratorError(<crate::gpu::workflows::gpu::setup_texture_generator::TypeIE as workflow::traits::WorkflowTypeIE>::Error),
//
//        #[error("GenerateTextureError{0}")]
//        GenerateTextureError(<crate::gpu::workflows::gpu::generate_texture::TypeIOE as workflow::traits::WorkflowTypeIOE>::Error),
//
//        #[error("SpawnChunkError{0}")]
//        SpawnChunkError(<crate::chunk::workflows::chunk::spawn_chunk::TypeIE as workflow::traits::WorkflowTypeIE>::Error),
//    }
//    impl std::convert::From<gpu::workflows::gpu::generate_texture::Error> for TestWorkflowFrameworkError {
//        fn from(e: gpu::workflows::gpu::generate_texture::Error) -> Self {
//            Self::GenerateTextureError(e)
//        }
//    }
//    impl std::convert::From<gpu::workflows::gpu::setup_texture_generator::Error> for TestWorkflowFrameworkError {
//        fn from(e: gpu::workflows::gpu::setup_texture_generator::Error) -> Self {
//            Self::SetupTextureGeneratorError(e)
//        }
//    }
//    impl std::convert::From<chunk::workflows::chunk::spawn_chunk::Error> for TestWorkflowFrameworkError {
//        fn from(e: chunk::workflows::chunk::spawn_chunk::Error) -> Self {
//            Self::SpawnChunkError(e)
//        }
//    }
//
//    pub async fn test_workflow_framework() -> Result<(), TestWorkflowFrameworkError> {
//        let shader_name = "texture_generators/example_compute_uv";
//        let shader_path = "assets/shaders/texture_generators/example_compute_uv.wgsl";
//
//        {
//            type T = crate::gpu::workflows::gpu::setup_texture_generator::TypeIE;
//            type I = <T as crate::workflow::traits::WorkflowTypeIE>::Input;
//
//            crate::gpu::workflows::gpu::setup_texture_generator::run(I {
//                shader_name,
//                shader_path: shader_path.to_string(),
//            }).await.map_err(Into::<TestWorkflowFrameworkError>::into)
//        }?;
//
//        let generate_texture_output = {
//            type T = crate::gpu::workflows::gpu::generate_texture::TypeIOE;
//            type I = <T as crate::workflow::traits::WorkflowTypeIOE>::Input;
//
//            crate::gpu::workflows::gpu::generate_texture::run(I {
//                shader_name,
//                texture_size: crate::config::statics::CONFIG.get::<f32>("chunk/size") as usize,
//                param_data: vec![0.0]
//            }).await.map_err(Into::<TestWorkflowFrameworkError>::into)
//        }?;
//
//        {
//            type T = crate::chunk::workflows::chunk::spawn_chunk::TypeIE;
//            type I = <T as crate::workflow::traits::WorkflowTypeIE>::Input;
//
//            crate::chunk::workflows::chunk::spawn_chunk::run(I {
//                chunk_coord: (0, 0),
//                chunk_owner: None,
//                metric_texture: generate_texture_output.texture_handle,
//            }).await.map_err(Into::<TestWorkflowFrameworkError>::into)
//        }?;
//
//        Ok(())
//    }
//
//    crate::workflow::statics::COMPOSITE_WORKFLOW_RUNTIME.lock().unwrap().spawn_fallible(Box::pin(test_workflow_framework()));
//}
