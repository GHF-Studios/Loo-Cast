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
/*
// Non-Expanded new WIP universal startup system, which will replace the current startup system
fn startup_system(
    mut commands: Commands,
    mut workflow_type_module_registry: ResMut<WorkflowTypeModuleRegistry>,
) {
    define_composite_workflow!(Startup {
        workflow!(id!(Camera::SpawnMainCamera));
        workflow!(id!(Debug::SpawnDebugUI));
        workflow!(id!(Debug::SpawnDebugObjects));
        workflow!(id!(IE, Gpu::SetupTextureGenerator), Input {
            shader_name: "texture_generators/example_compute_uv",
            shader_path: "assets/shaders/texture_generators/example_compute_uv.wgsl".to_string(),
        });
        let generate_texture_output = workflow!(id!(IOE, Gpu::GenerateTexture), Input {
            shader_name,
            texture_size: crate::config::statics::CONFIG.get::<f32>("chunk/size") as usize,
            param_data: vec![0.0]
        });
        workflow!(id!(IE, Chunk::SpawnChunk), Input {
            chunk_coord: (0, 0),
            chunk_owner: None,
            metric_texture: generate_texture_output.texture_handle,
        });

        Ok(())
    });

    crate::workflow::statics::COMPOSITE_WORKFLOW_RUNTIME.lock().unwrap().spawn_fallible(Box::pin(startup()));
}










fn startup_system_2(
    mut commands: Commands,
    mut workflow_type_module_registry: ResMut<WorkflowTypeModuleRegistry>,
) {
    define_composite_workflow!(Startup {
        workflow!(#[WorkflowSignature(None)]crate::camera::workflows::camera::spawn_main_camera::Type);
        workflow!(#[WorkflowSignature(None)]crate::debug::workflows::debug::spawn_debug_ui::Type);
        workflow!(#[WorkflowSignature(None)]crate::debug::workflows::debug::spawn_debug_objects::Type);

        workflow!(#[WorkflowSignature(IE)]crate::gpu::workflows::gpu::setup_texture_generator::TypeIE, Input {
            shader_name: "texture_generators/example_compute_uv",
            shader_path: "assets/shaders/texture_generators/example_compute_uv.wgsl".to_string(),
        });
        let generate_texture_output = workflow!(#[WorkflowSignature(IOE)]crate::gpu::workflows::gpu::generate_texture::TypeIOE, Input {
            shader_name,
            texture_size: crate::config::statics::CONFIG.get::<f32>("chunk/size") as usize,
            param_data: vec![0.0]
        });
        workflow!(#[WorkflowSignature(IE)]crate::chunk::workflows::chunk::spawn_chunk::TypeIE, Input {
            chunk_coord: (0, 0),
            chunk_owner: None,
            metric_texture: generate_texture_output.texture_handle,
        });

        Ok(())
    });

    crate::workflow::statics::COMPOSITE_WORKFLOW_RUNTIME.lock().unwrap().spawn_fallible(Box::pin(startup()));
}

*/









fn startup_system_3(
    mut commands: Commands,
    mut workflow_type_module_registry: ResMut<WorkflowTypeModuleRegistry>,
) {
    define_composite_workflow!(Startup {
        #[WorkflowSignature(None)] #[WorkflowType(crate::camera::workflows::camera::spawn_main_camera::Type)];
        #[WorkflowSignature(None)] #[WorkflowType(crate::debug::workflows::debug::spawn_debug_ui::Type)];
        #[WorkflowSignature(None)] #[WorkflowType(crate::debug::workflows::debug::spawn_debug_objects::Type)];

        let chunk_shader_name = "texture_generators/example_compute_uv";
        let chunk_shader_path = "assets/shaders/texture_generators/example_compute_uv.wgsl".to_string();

        #[WorkflowSignature(IE)] #[WorkflowType(crate::gpu::workflows::gpu::setup_texture_generator::TypeIE)] #[WorkflowInput {
            shader_name: chunk_shader_name,
            shader_path: chunk_shader_path,
        }];
        let generate_texture_output = #[WorkflowSignature(IOE)] #[WorkflowType(crate::gpu::workflows::gpu::generate_texture::TypeIOE)] #[WorkflowInput {
            shader_name: chunk_shader_name,
            texture_size: crate::config::statics::CONFIG.get::<f32>("chunk/size") as usize,
            param_data: vec![0.0]
        }];
        #[WorkflowSignature(IE)] #[WorkflowType(crate::chunk::workflows::chunk::spawn_chunk::TypeIE)] #[WorkflowInput {
            chunk_coord: (0, 0),
            chunk_owner: None,
            metric_texture: generate_texture_output.texture_handle,
        }];

        Ok(())
    });

    crate::workflow::statics::COMPOSITE_WORKFLOW_RUNTIME.lock().unwrap().spawn_fallible(Box::pin(startup()));
}











// --- Fully expanded and working oneshot composite workflow `startup` ---
fn fully_expanded_startup_system() {
    #[derive(Debug, thiserror::Error)]
    pub enum StartupError {
        #[error("SetupTextureGeneratorError{0}")]
        SetupTextureGeneratorError(<crate::gpu::workflows::gpu::setup_texture_generator::TypeIE as workflow::traits::WorkflowTypeIE>::Error),

        #[error("GenerateTextureError{0}")]
        GenerateTextureError(<crate::gpu::workflows::gpu::generate_texture::TypeIOE as workflow::traits::WorkflowTypeIOE>::Error),

        #[error("SpawnChunkError{0}")]
        SpawnChunkError(<crate::chunk::workflows::chunk::spawn_chunk::TypeIE as workflow::traits::WorkflowTypeIE>::Error),
    }
    impl std::convert::From<gpu::workflows::gpu::generate_texture::Error>
        for StartupError
    {
        fn from(e: gpu::workflows::gpu::generate_texture::Error) -> Self {
            Self::GenerateTextureError(e)
        }
    }
    impl std::convert::From<gpu::workflows::gpu::setup_texture_generator::Error>
        for StartupError
    {
        fn from(e: gpu::workflows::gpu::setup_texture_generator::Error) -> Self {
            Self::SetupTextureGeneratorError(e)
        }
    }
    impl std::convert::From<chunk::workflows::chunk::spawn_chunk::Error>
        for StartupError
    {
        fn from(e: chunk::workflows::chunk::spawn_chunk::Error) -> Self {
            Self::SpawnChunkError(e)
        }
    }

    pub async fn startup() -> Result<(), StartupError> {
        {
            crate::camera::workflows::camera::spawn_main_camera::run().await
        };
        {
            crate::debug::workflows::debug::spawn_debug_ui::run().await
        };
        {
            crate::debug::workflows::debug::spawn_debug_objects::run().await
        };

        let shader_name = "texture_generators/example_compute_uv";
        let shader_path = "assets/shaders/texture_generators/example_compute_uv.wgsl";
        {
            type T = crate::gpu::workflows::gpu::setup_texture_generator::TypeIE;
            type I = <T as crate::workflow::traits::WorkflowTypeIE>::Input;

            crate::gpu::workflows::gpu::setup_texture_generator::run(I {
                shader_name,
                shader_path: shader_path.to_string(),
            }).await
            .map_err(Into::<StartupError>::into)
        }?;
        let generate_texture_output = {
            type T = crate::gpu::workflows::gpu::generate_texture::TypeIOE;
            type I = <T as crate::workflow::traits::WorkflowTypeIOE>::Input;

            crate::gpu::workflows::gpu::generate_texture::run(I {
                shader_name,
                texture_size: crate::config::statics::CONFIG.get::<f32>("chunk/size") as usize,
                param_data: vec![0.0],
            }).await
            .map_err(Into::<StartupError>::into)
        }?;
        {
            type T = crate::chunk::workflows::chunk::spawn_chunk::TypeIE;
            type I = <T as crate::workflow::traits::WorkflowTypeIE>::Input;

            crate::chunk::workflows::chunk::spawn_chunk::run(I {
                chunk_coord: (0, 0),
                chunk_owner: None,
                metric_texture: generate_texture_output.texture_handle,
            }).await
            .map_err(Into::<StartupError>::into)
        }?;

        Ok(())
    }

    crate::workflow::statics::COMPOSITE_WORKFLOW_RUNTIME
        .lock()
        .unwrap()
        .spawn_fallible(Box::pin(startup()));
}


// OLD CODE OLD CODE OLD CODE OLD CODE OLD CODE OLD CODE OLD CODE OLD CODE OLD CODE OLD CODE OLD CODE OLD CODE OLD CODE OLD CODE OLD CODE OLD CODE OLD CODE OLD CODE OLD CODE OLD CODE
// --- Fully expanded and working oneshot composite workflow `test_workflow_framework` ---
//fn startup_system() {
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
//    impl std::convert::From<gpu::workflows::gpu::generate_texture::Error>
//        for TestWorkflowFrameworkError
//    {
//        fn from(e: gpu::workflows::gpu::generate_texture::Error) -> Self {
//            Self::GenerateTextureError(e)
//        }
//    }
//    impl std::convert::From<gpu::workflows::gpu::setup_texture_generator::Error>
//        for TestWorkflowFrameworkError
//    {
//        fn from(e: gpu::workflows::gpu::setup_texture_generator::Error) -> Self {
//            Self::SetupTextureGeneratorError(e)
//        }
//    }
//    impl std::convert::From<chunk::workflows::chunk::spawn_chunk::Error>
//        for TestWorkflowFrameworkError
//    {
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
//            })
//            .await
//            .map_err(Into::<TestWorkflowFrameworkError>::into)
//        }?;
//
//        let generate_texture_output = {
//            type T = crate::gpu::workflows::gpu::generate_texture::TypeIOE;
//            type I = <T as crate::workflow::traits::WorkflowTypeIOE>::Input;
//
//            crate::gpu::workflows::gpu::generate_texture::run(I {
//                shader_name,
//                texture_size: crate::config::statics::CONFIG.get::<f32>("chunk/size") as usize,
//                param_data: vec![0.0],
//            })
//            .await
//            .map_err(Into::<TestWorkflowFrameworkError>::into)
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
//            })
//            .await
//            .map_err(Into::<TestWorkflowFrameworkError>::into)
//        }?;
//
//        Ok(())
//    }
//
//    crate::workflow::statics::COMPOSITE_WORKFLOW_RUNTIME
//        .lock()
//        .unwrap()
//        .spawn_fallible(Box::pin(test_workflow_framework()));
//}

//fn startup_system_unexpanded() {
//    define_composite_workflow!(TestWorkflowFramework {
//        let shader_name = "texture_generators/example_compute_uv";
//        let shader_path = "assets/shaders/texture_generators/example_compute_uv.wgsl";
//
//        run_workflow!(workflow_path!(Gpu::SetupTextureGenerator, IE), Input {
//            shader_name,
//            shader_path: shader_path.to_string(),
//        });
//
//        let generate_texture_output = run_workflow!(workflow_path!(Gpu::GenerateTexture, IOE), Input {
//            shader_name,
//            texture_size: crate::config::statics::CONFIG.get::<f32>("chunk/size") as usize,
//            param_data: vec![0.0]
//        });
//
//        run_workflow!(workflow_path!(Chunk::SpawnChunk, IE), Input {
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
//        run_workflow!(workflow_path!(Gpu::SetupTextureGenerator, IE), Input {
//            shader_name,
//            shader_path: shader_path.to_string(),
//        });
//
//        let generate_texture_output = run_workflow!(workflow_path!(Gpu::GenerateTexture, IOE), Input {
//            shader_name,
//            texture_size: crate::config::statics::CONFIG.get::<f32>("chunk/size") as usize,
//            param_data: vec![0.0]
//        });
//
//        run_workflow!(workflow_path!(Chunk::SpawnChunk, IE), Input {
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

