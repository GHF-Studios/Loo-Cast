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

use crate::workflow::functions::handle_composite_workflow_return;
use bevy::{app::PluginGroupBuilder, prelude::*};
use camera::CameraPlugin;
use iyes_perf_ui::{
    entries::{PerfUiFramerateEntries, PerfUiSystemEntries},
    prelude::{PerfUiEntryEntityCount, PerfUiRoot},
};
use spacetime_engine_macros::{
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
            Spawn: Ecs,
        },
    },
    Chunk {
        SpawnChunks {
            ValidateAndSpawn: Ecs,
        },
        DespawnChunks {
            FindAndDespawn: Ecs,
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
            ValidateAndLoad: Ecs,
        },
        UnloadChunks {
            Unload: Ecs,
        }
    },
    Debug {
        SpawnDebugObjects {
            Spawn: Ecs,
        },
    },
    Gpu {
        SetupTextureGenerator {
            SetupPhase1: Ecs,
            SetupPhase2: RenderWhile,
            SetupPhase3: Ecs,
        },
        GenerateTexture {
            PrepareRequest: Ecs,
            GetTextureView: RenderWhile,
            DispatchCompute: Render,
            WaitForCompute: EcsWhile,
        },
    },
    Player {
        SpawnPlayer {
            ValidateAndSpawn: Ecs,
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
    let handle = composite_workflow!(JustDoIt {
        workflow!(Camera::SpawnMainCamera);

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
        workflow!(IE, Chunk::SpawnChunks, Input {
            inputs: vec![crate::chunk::workflows::chunk::spawn_chunks::user_items::SpawnChunkInput {
                chunk_coord: (0, 0),
                chunk_owner: None,
                metric_texture: generate_texture_output.texture_handle,
            }]
        });

        workflow!(Debug::SpawnDebugObjects);
    });

    handle_composite_workflow_return(handle, || {
        composite_workflow_return!();
    });
}
