#![allow(clippy::type_complexity)]
#![allow(clippy::too_many_arguments)]

pub use bevy_consumable_event;
pub use core_api_macros;

pub use anymap;
pub use bevy;
pub use bevy_egui;
pub use bevy_inspector_egui;
pub use bevy_rapier2d;
pub use bytemuck;
pub use console_subscriber;
pub use crossbeam;
pub use crossbeam_channel;
pub use dashmap;
pub use dyn_clone;
pub use egui;
pub use egui_dock;
pub use futures;
pub use glam;
pub use image;
pub use iyes_perf_ui;
pub use lazy_static;
pub use log;
pub use mlua;
pub use noise;
pub use num_bigint;
pub use num_traits;
pub use once_cell;
pub use parking_lot;
pub use paste;
pub use pin_project_lite;
pub use queues;
pub use rand;
pub use serde;
pub use serde_json;
pub use thiserror;
pub use tokio;
pub use toml;
pub use tracing;
pub use tracing_appender;
pub use tracing_subscriber;
pub use transform_gizmo_bevy;
pub use uuid;

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
//pub mod decl_macros;
//pub mod singletons;
//pub mod traits;

// Modules
pub mod camera;
pub mod chunk;
pub mod chunk_actor;
pub mod chunk_loader;
pub mod config;
pub mod core;
pub mod debug;
pub mod entity;
pub mod follower;
pub mod gpu;
pub mod input;
pub mod logging;
pub mod player;
pub mod time;
pub mod ui;
pub mod utils;
pub mod window;
pub mod workflow;

use bevy::{app::PluginGroupBuilder, prelude::*};
use core_api_macros::{api_initializer, register_workflow_mods};

use camera::CameraPlugin;
use chunk::ChunkPlugin;
use chunk_actor::ChunkActorPlugin;
use chunk_loader::ChunkLoaderPlugin;
use config::ConfigPlugin;
use core::CorePlugin;
use debug::DebugPlugin;
use entity::EntityPlugin;
use follower::FollowerPlugin;
use gpu::GpuPlugin;
use input::InputPlugin;
use logging::LogPlugin;
use player::PlayerPlugin;
use time::TimePlugin;
use ui::UiPlugin;
use utils::UtilsPlugin;
use window::WindowPlugin;
use workflow::WorkflowPlugin;

pub struct CoreApiPluginGroup;
impl PluginGroup for CoreApiPluginGroup {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add_group(WorkflowPlugins)
            .add(CorePlugin)
            .add(CameraPlugin)
            .add(ChunkPlugin)
            .add(ChunkActorPlugin)
            .add(ChunkLoaderPlugin)
            .add(ConfigPlugin)
            .add(DebugPlugin)
            .add(EntityPlugin)
            .add(FollowerPlugin)
            .add(GpuPlugin)
            .add(InputPlugin)
            .add(LogPlugin)
            .add(PlayerPlugin)
            .add(TimePlugin)
            .add(UiPlugin)
            .add(UtilsPlugin)
            .add(WindowPlugin)
            .add(WorkflowPlugin)
    }
}

register_workflow_mods!(
    Camera {
        SpawnMainCameras {
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
        OnRemovedChunkLoader {
            SendRemovedChunkLoaderEvent: Ecs
        },
        LoadChunks {
            ValidateAndLoadAndWait: EcsWhile,
        },
        UnloadChunks {
            UnloadAndWait: EcsWhile,
        },
    },
    Core {
        FinishStartup {
            InsertResource: Ecs,
        },
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
        DespawnPlayer {
            ValidateAndDespawnAndWait: EcsWhile,
        }
    },
);

api_initializer!(
    crate::config::statics::CONFIG,
    crate::core::statics::TOKIO_RUNTIME,
    crate::core::statics::START_TIME,
    crate::entity::statics::ENTITY_RESERVATION_BUFFER,
    crate::logging::statics::LOG_ID_COUNTER,
    crate::logging::statics::SPAN_EVENT_BUFFER,
    crate::logging::statics::LOG_EVENT_BUFFER,
    crate::time::statics::ELAPSED_VIRTUAL_NANOS,
    crate::time::statics::PENDING_VIRTUAL_SLEEPS,
    crate::workflow::statics::WORKFLOW_TOKIO_RUNTIME,
    crate::workflow::statics::COMPOSITE_WORKFLOW_RUNTIME,
);
