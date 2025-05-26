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

use crate::workflow::functions::handle_composite_workflow_return_later;
use bevy::{app::PluginGroupBuilder, prelude::*};
use camera::CameraPlugin;
use core_engine_macros::{
    composite_workflow, composite_workflow_return, register_workflow_mods,
};
use workflow::WorkflowPlugin;
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

pub struct SpacetimeEngineMainPlugins;
impl PluginGroup for SpacetimeEngineMainPlugins {
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

register_workflow_mods!(
    Camera {
        SpawnMainCamera {
            SpawnAndWait: EcsWhile,
        },
    },
    Chunk {
        SpawnChunks {
            ValidateAndSpawnAndWait: EcsWhile,
        },
        DespawnChunks {
            FindAndDespawnAndWait: EcsWhile,
        },
        TransferChunkOwnerships {
            FindAndTransferOwnership: Ecs,
        }
    },
    ChunkLoader {
        CategorizeChunks {
            Categorize: Ecs,
        },
        OnRemoveChunkLoader {
            ExtractUnloadChunkInputs: Ecs
        },
        LoadChunks {
            ValidateAndLoadAndWait: EcsWhile,
        },
        UnloadChunks {
            UnloadAndWait: EcsWhile,
        }
    },
    Debug {
        SpawnDebugObjects {
            SpawnAndWait: EcsWhile,
        },
    },
    Gpu {
        SetupTextureGenerator {
            SetupPhase1: Ecs,
            SetupPhase2: RenderWhile,
            SetupPhase3: Ecs,
        },
        GenerateTextures {
            PrepareBatch: Ecs,
            GetTextureViews: RenderWhile,
            DispatchBatch: Render,
            WaitForBatch: EcsWhile,
        },
    },
    Player {
        SpawnPlayer {
            ValidateAndSpawnAndWait: EcsWhile,
        },
    },
);

pub(crate) struct SpacetimeEngineCorePlugin;
impl Plugin for SpacetimeEngineCorePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, startup_system);
    }
}

fn startup_system() {
    let handle = composite_workflow!({
        workflow!(Camera::SpawnMainCamera);

        // NOTE!!!!!    Old code I left in because it show how complex stuff can get    !!!!!NOTE
        //let chunk_shader_name = "texture_generators/example_compute_uv";
        //let chunk_shader_path = "assets/shaders/texture_generators/example_compute_uv.wgsl".to_string();
        //workflow!(IE, Gpu::SetupTextureGenerator, Input {
        //    shader_name: chunk_shader_name,
        //    shader_path: chunk_shader_path,
        //});
        //let chunk_coords: Vec<(i32, i32)> = (-8..=8)
        //    .flat_map(|x| (-8..=8).map(move |y| (x, y)))
        //    .collect();
        //let texture_size = crate::config::statics::CONFIG.get::<f32>("chunk/size") as usize;
        //let param_data: Vec<Vec<f32>> = chunk_coords
        //    .iter()
        //    .map(|_| vec![0.0])
        //    .collect();
        //let texture_output = workflow!(IO, Gpu::GenerateTextures, Input {
        //    shader_name: chunk_shader_name,
        //    texture_sizes: vec![texture_size; chunk_coords.len()],
        //    param_data,
        //});
        //let spawn_inputs: Vec<_> = chunk_coords
        //    .into_iter()
        //    .zip(texture_output.texture_handles.into_iter())
        //    .map(|(chunk_coord, texture_handle)| crate::chunk::workflows::chunk::spawn_chunks::user_items::SpawnChunkInput {
        //        chunk_coord,
        //        chunk_owner: None,
        //        metric_texture: texture_handle,
        //    })
        //    .collect();
        //workflow!(IOE, Chunk::SpawnChunks, Input {
        //    inputs: spawn_inputs
        //});

        //workflow!(Debug::SpawnDebugObjects);
    });

    handle_composite_workflow_return_later(handle, |_ctx| {
        composite_workflow_return!();
    });
}

