#![allow(clippy::type_complexity)]
#![allow(clippy::too_many_arguments)]

pub use bevy_consumable_event;
pub use core_mod_macros;

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
pub mod usf;
pub mod utils;
pub mod window;
pub mod workflow;

use bevy::{app::PluginGroupBuilder, prelude::*};
use core_mod_macros::{api_initializer, register_workflow_mods};

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
        CategorizeChunksScaleQuectoMeter000001 {
            Categorize: Ecs,
        },
        OnRemoveChunkLoaderScaleQuectoMeter000001 {
            ExtractUnloadChunkInputs: Ecs
        },
        OnRemovedChunkLoaderScaleQuectoMeter000001 {
            SendRemovedChunkLoaderEvent: Ecs
        },
        LoadChunksScaleQuectoMeter000001 {
            ValidateAndLoadAndWait: EcsWhile,
        },
        UnloadChunksScaleQuectoMeter000001 {
            UnloadAndWait: EcsWhile,
        },

        CategorizeChunksScaleQuectoMeter00001 {
            Categorize: Ecs,
        },
        OnRemoveChunkLoaderScaleQuectoMeter00001 {
            ExtractUnloadChunkInputs: Ecs
        },
        OnRemovedChunkLoaderScaleQuectoMeter00001 {
            SendRemovedChunkLoaderEvent: Ecs
        },
        LoadChunksScaleQuectoMeter00001 {
            ValidateAndLoadAndWait: EcsWhile,
        },
        UnloadChunksScaleQuectoMeter00001 {
            UnloadAndWait: EcsWhile,
        },

        CategorizeChunksScaleQuectoMeter0001 {
            Categorize: Ecs,
        },
        OnRemoveChunkLoaderScaleQuectoMeter0001 {
            ExtractUnloadChunkInputs: Ecs
        },
        OnRemovedChunkLoaderScaleQuectoMeter0001 {
            SendRemovedChunkLoaderEvent: Ecs
        },
        LoadChunksScaleQuectoMeter0001 {
            ValidateAndLoadAndWait: EcsWhile,
        },
        UnloadChunksScaleQuectoMeter0001 {
            UnloadAndWait: EcsWhile,
        },

        CategorizeChunksScaleQuectoMeter001 {
            Categorize: Ecs,
        },
        OnRemoveChunkLoaderScaleQuectoMeter001 {
            ExtractUnloadChunkInputs: Ecs
        },
        OnRemovedChunkLoaderScaleQuectoMeter001 {
            SendRemovedChunkLoaderEvent: Ecs
        },
        LoadChunksScaleQuectoMeter001 {
            ValidateAndLoadAndWait: EcsWhile,
        },
        UnloadChunksScaleQuectoMeter001 {
            UnloadAndWait: EcsWhile,
        },

        CategorizeChunksScaleQuectoMeter01 {
            Categorize: Ecs,
        },
        OnRemoveChunkLoaderScaleQuectoMeter01 {
            ExtractUnloadChunkInputs: Ecs
        },
        OnRemovedChunkLoaderScaleQuectoMeter01 {
            SendRemovedChunkLoaderEvent: Ecs
        },
        LoadChunksScaleQuectoMeter01 {
            ValidateAndLoadAndWait: EcsWhile,
        },
        UnloadChunksScaleQuectoMeter01 {
            UnloadAndWait: EcsWhile,
        },

        CategorizeChunksScaleQuectoMeter1 {
            Categorize: Ecs,
        },
        OnRemoveChunkLoaderScaleQuectoMeter1 {
            ExtractUnloadChunkInputs: Ecs
        },
        OnRemovedChunkLoaderScaleQuectoMeter1 {
            SendRemovedChunkLoaderEvent: Ecs
        },
        LoadChunksScaleQuectoMeter1 {
            ValidateAndLoadAndWait: EcsWhile,
        },
        UnloadChunksScaleQuectoMeter1 {
            UnloadAndWait: EcsWhile,
        },

        CategorizeChunksScaleQuectoMeter10 {
            Categorize: Ecs,
        },
        OnRemoveChunkLoaderScaleQuectoMeter10 {
            ExtractUnloadChunkInputs: Ecs
        },
        OnRemovedChunkLoaderScaleQuectoMeter10 {
            SendRemovedChunkLoaderEvent: Ecs
        },
        LoadChunksScaleQuectoMeter10 {
            ValidateAndLoadAndWait: EcsWhile,
        },
        UnloadChunksScaleQuectoMeter10 {
            UnloadAndWait: EcsWhile,
        },

        CategorizeChunksScaleQuectoMeter100 {
            Categorize: Ecs,
        },
        OnRemoveChunkLoaderScaleQuectoMeter100 {
            ExtractUnloadChunkInputs: Ecs
        },
        OnRemovedChunkLoaderScaleQuectoMeter100 {
            SendRemovedChunkLoaderEvent: Ecs
        },
        LoadChunksScaleQuectoMeter100 {
            ValidateAndLoadAndWait: EcsWhile,
        },
        UnloadChunksScaleQuectoMeter100 {
            UnloadAndWait: EcsWhile,
        },

        CategorizeChunksScaleRontoMeter1 {
            Categorize: Ecs,
        },
        OnRemoveChunkLoaderScaleRontoMeter1 {
            ExtractUnloadChunkInputs: Ecs
        },
        OnRemovedChunkLoaderScaleRontoMeter1 {
            SendRemovedChunkLoaderEvent: Ecs
        },
        LoadChunksScaleRontoMeter1 {
            ValidateAndLoadAndWait: EcsWhile,
        },
        UnloadChunksScaleRontoMeter1 {
            UnloadAndWait: EcsWhile,
        },

        CategorizeChunksScaleRontoMeter10 {
            Categorize: Ecs,
        },
        OnRemoveChunkLoaderScaleRontoMeter10 {
            ExtractUnloadChunkInputs: Ecs
        },
        OnRemovedChunkLoaderScaleRontoMeter10 {
            SendRemovedChunkLoaderEvent: Ecs
        },
        LoadChunksScaleRontoMeter10 {
            ValidateAndLoadAndWait: EcsWhile,
        },
        UnloadChunksScaleRontoMeter10 {
            UnloadAndWait: EcsWhile,
        },

        CategorizeChunksScaleRontoMeter100 {
            Categorize: Ecs,
        },
        OnRemoveChunkLoaderScaleRontoMeter100 {
            ExtractUnloadChunkInputs: Ecs
        },
        OnRemovedChunkLoaderScaleRontoMeter100 {
            SendRemovedChunkLoaderEvent: Ecs
        },
        LoadChunksScaleRontoMeter100 {
            ValidateAndLoadAndWait: EcsWhile,
        },
        UnloadChunksScaleRontoMeter100 {
            UnloadAndWait: EcsWhile,
        },

        CategorizeChunksScaleYoctoMeter1 {
            Categorize: Ecs,
        },
        OnRemoveChunkLoaderScaleYoctoMeter1 {
            ExtractUnloadChunkInputs: Ecs
        },
        OnRemovedChunkLoaderScaleYoctoMeter1 {
            SendRemovedChunkLoaderEvent: Ecs
        },
        LoadChunksScaleYoctoMeter1 {
            ValidateAndLoadAndWait: EcsWhile,
        },
        UnloadChunksScaleYoctoMeter1 {
            UnloadAndWait: EcsWhile,
        },

        CategorizeChunksScaleYoctoMeter10 {
            Categorize: Ecs,
        },
        OnRemoveChunkLoaderScaleYoctoMeter10 {
            ExtractUnloadChunkInputs: Ecs
        },
        OnRemovedChunkLoaderScaleYoctoMeter10 {
            SendRemovedChunkLoaderEvent: Ecs
        },
        LoadChunksScaleYoctoMeter10 {
            ValidateAndLoadAndWait: EcsWhile,
        },
        UnloadChunksScaleYoctoMeter10 {
            UnloadAndWait: EcsWhile,
        },

        CategorizeChunksScaleYoctoMeter100 {
            Categorize: Ecs,
        },
        OnRemoveChunkLoaderScaleYoctoMeter100 {
            ExtractUnloadChunkInputs: Ecs
        },
        OnRemovedChunkLoaderScaleYoctoMeter100 {
            SendRemovedChunkLoaderEvent: Ecs
        },
        LoadChunksScaleYoctoMeter100 {
            ValidateAndLoadAndWait: EcsWhile,
        },
        UnloadChunksScaleYoctoMeter100 {
            UnloadAndWait: EcsWhile,
        },

        CategorizeChunksScaleZeptoMeter1 {
            Categorize: Ecs,
        },
        OnRemoveChunkLoaderScaleZeptoMeter1 {
            ExtractUnloadChunkInputs: Ecs
        },
        OnRemovedChunkLoaderScaleZeptoMeter1 {
            SendRemovedChunkLoaderEvent: Ecs
        },
        LoadChunksScaleZeptoMeter1 {
            ValidateAndLoadAndWait: EcsWhile,
        },
        UnloadChunksScaleZeptoMeter1 {
            UnloadAndWait: EcsWhile,
        },

        CategorizeChunksScaleZeptoMeter10 {
            Categorize: Ecs,
        },
        OnRemoveChunkLoaderScaleZeptoMeter10 {
            ExtractUnloadChunkInputs: Ecs
        },
        OnRemovedChunkLoaderScaleZeptoMeter10 {
            SendRemovedChunkLoaderEvent: Ecs
        },
        LoadChunksScaleZeptoMeter10 {
            ValidateAndLoadAndWait: EcsWhile,
        },
        UnloadChunksScaleZeptoMeter10 {
            UnloadAndWait: EcsWhile,
        },

        CategorizeChunksScaleZeptoMeter100 {
            Categorize: Ecs,
        },
        OnRemoveChunkLoaderScaleZeptoMeter100 {
            ExtractUnloadChunkInputs: Ecs
        },
        OnRemovedChunkLoaderScaleZeptoMeter100 {
            SendRemovedChunkLoaderEvent: Ecs
        },
        LoadChunksScaleZeptoMeter100 {
            ValidateAndLoadAndWait: EcsWhile,
        },
        UnloadChunksScaleZeptoMeter100 {
            UnloadAndWait: EcsWhile,
        },

        CategorizeChunksScaleAttoMeter1 {
            Categorize: Ecs,
        },
        OnRemoveChunkLoaderScaleAttoMeter1 {
            ExtractUnloadChunkInputs: Ecs
        },
        OnRemovedChunkLoaderScaleAttoMeter1 {
            SendRemovedChunkLoaderEvent: Ecs
        },
        LoadChunksScaleAttoMeter1 {
            ValidateAndLoadAndWait: EcsWhile,
        },
        UnloadChunksScaleAttoMeter1 {
            UnloadAndWait: EcsWhile,
        },

        CategorizeChunksScaleAttoMeter10 {
            Categorize: Ecs,
        },
        OnRemoveChunkLoaderScaleAttoMeter10 {
            ExtractUnloadChunkInputs: Ecs
        },
        OnRemovedChunkLoaderScaleAttoMeter10 {
            SendRemovedChunkLoaderEvent: Ecs
        },
        LoadChunksScaleAttoMeter10 {
            ValidateAndLoadAndWait: EcsWhile,
        },
        UnloadChunksScaleAttoMeter10 {
            UnloadAndWait: EcsWhile,
        },

        CategorizeChunksScaleAttoMeter100 {
            Categorize: Ecs,
        },
        OnRemoveChunkLoaderScaleAttoMeter100 {
            ExtractUnloadChunkInputs: Ecs
        },
        OnRemovedChunkLoaderScaleAttoMeter100 {
            SendRemovedChunkLoaderEvent: Ecs
        },
        LoadChunksScaleAttoMeter100 {
            ValidateAndLoadAndWait: EcsWhile,
        },
        UnloadChunksScaleAttoMeter100 {
            UnloadAndWait: EcsWhile,
        },

        CategorizeChunksScaleFemtoMeter1 {
            Categorize: Ecs,
        },
        OnRemoveChunkLoaderScaleFemtoMeter1 {
            ExtractUnloadChunkInputs: Ecs
        },
        OnRemovedChunkLoaderScaleFemtoMeter1 {
            SendRemovedChunkLoaderEvent: Ecs
        },
        LoadChunksScaleFemtoMeter1 {
            ValidateAndLoadAndWait: EcsWhile,
        },
        UnloadChunksScaleFemtoMeter1 {
            UnloadAndWait: EcsWhile,
        },

        CategorizeChunksScaleFemtoMeter10 {
            Categorize: Ecs,
        },
        OnRemoveChunkLoaderScaleFemtoMeter10 {
            ExtractUnloadChunkInputs: Ecs
        },
        OnRemovedChunkLoaderScaleFemtoMeter10 {
            SendRemovedChunkLoaderEvent: Ecs
        },
        LoadChunksScaleFemtoMeter10 {
            ValidateAndLoadAndWait: EcsWhile,
        },
        UnloadChunksScaleFemtoMeter10 {
            UnloadAndWait: EcsWhile,
        },

        CategorizeChunksScaleFemtoMeter100 {
            Categorize: Ecs,
        },
        OnRemoveChunkLoaderScaleFemtoMeter100 {
            ExtractUnloadChunkInputs: Ecs
        },
        OnRemovedChunkLoaderScaleFemtoMeter100 {
            SendRemovedChunkLoaderEvent: Ecs
        },
        LoadChunksScaleFemtoMeter100 {
            ValidateAndLoadAndWait: EcsWhile,
        },
        UnloadChunksScaleFemtoMeter100 {
            UnloadAndWait: EcsWhile,
        },

        CategorizeChunksScalePicoMeter1 {
            Categorize: Ecs,
        },
        OnRemoveChunkLoaderScalePicoMeter1 {
            ExtractUnloadChunkInputs: Ecs
        },
        OnRemovedChunkLoaderScalePicoMeter1 {
            SendRemovedChunkLoaderEvent: Ecs
        },
        LoadChunksScalePicoMeter1 {
            ValidateAndLoadAndWait: EcsWhile,
        },
        UnloadChunksScalePicoMeter1 {
            UnloadAndWait: EcsWhile,
        },

        CategorizeChunksScalePicoMeter10 {
            Categorize: Ecs,
        },
        OnRemoveChunkLoaderScalePicoMeter10 {
            ExtractUnloadChunkInputs: Ecs
        },
        OnRemovedChunkLoaderScalePicoMeter10 {
            SendRemovedChunkLoaderEvent: Ecs
        },
        LoadChunksScalePicoMeter10 {
            ValidateAndLoadAndWait: EcsWhile,
        },
        UnloadChunksScalePicoMeter10 {
            UnloadAndWait: EcsWhile,
        },

        CategorizeChunksScalePicoMeter100 {
            Categorize: Ecs,
        },
        OnRemoveChunkLoaderScalePicoMeter100 {
            ExtractUnloadChunkInputs: Ecs
        },
        OnRemovedChunkLoaderScalePicoMeter100 {
            SendRemovedChunkLoaderEvent: Ecs
        },
        LoadChunksScalePicoMeter100 {
            ValidateAndLoadAndWait: EcsWhile,
        },
        UnloadChunksScalePicoMeter100 {
            UnloadAndWait: EcsWhile,
        },

        CategorizeChunksScaleNanoMeter1 {
            Categorize: Ecs,
        },
        OnRemoveChunkLoaderScaleNanoMeter1 {
            ExtractUnloadChunkInputs: Ecs
        },
        OnRemovedChunkLoaderScaleNanoMeter1 {
            SendRemovedChunkLoaderEvent: Ecs
        },
        LoadChunksScaleNanoMeter1 {
            ValidateAndLoadAndWait: EcsWhile,
        },
        UnloadChunksScaleNanoMeter1 {
            UnloadAndWait: EcsWhile,
        },

        CategorizeChunksScaleNanoMeter10 {
            Categorize: Ecs,
        },
        OnRemoveChunkLoaderScaleNanoMeter10 {
            ExtractUnloadChunkInputs: Ecs
        },
        OnRemovedChunkLoaderScaleNanoMeter10 {
            SendRemovedChunkLoaderEvent: Ecs
        },
        LoadChunksScaleNanoMeter10 {
            ValidateAndLoadAndWait: EcsWhile,
        },
        UnloadChunksScaleNanoMeter10 {
            UnloadAndWait: EcsWhile,
        },

        CategorizeChunksScaleNanoMeter100 {
            Categorize: Ecs,
        },
        OnRemoveChunkLoaderScaleNanoMeter100 {
            ExtractUnloadChunkInputs: Ecs
        },
        OnRemovedChunkLoaderScaleNanoMeter100 {
            SendRemovedChunkLoaderEvent: Ecs
        },
        LoadChunksScaleNanoMeter100 {
            ValidateAndLoadAndWait: EcsWhile,
        },
        UnloadChunksScaleNanoMeter100 {
            UnloadAndWait: EcsWhile,
        },

        CategorizeChunksScaleMicroMeter1 {
            Categorize: Ecs,
        },
        OnRemoveChunkLoaderScaleMicroMeter1 {
            ExtractUnloadChunkInputs: Ecs
        },
        OnRemovedChunkLoaderScaleMicroMeter1 {
            SendRemovedChunkLoaderEvent: Ecs
        },
        LoadChunksScaleMicroMeter1 {
            ValidateAndLoadAndWait: EcsWhile,
        },
        UnloadChunksScaleMicroMeter1 {
            UnloadAndWait: EcsWhile,
        },

        CategorizeChunksScaleMicroMeter10 {
            Categorize: Ecs,
        },
        OnRemoveChunkLoaderScaleMicroMeter10 {
            ExtractUnloadChunkInputs: Ecs
        },
        OnRemovedChunkLoaderScaleMicroMeter10 {
            SendRemovedChunkLoaderEvent: Ecs
        },
        LoadChunksScaleMicroMeter10 {
            ValidateAndLoadAndWait: EcsWhile,
        },
        UnloadChunksScaleMicroMeter10 {
            UnloadAndWait: EcsWhile,
        },

        CategorizeChunksScaleMicroMeter100 {
            Categorize: Ecs,
        },
        OnRemoveChunkLoaderScaleMicroMeter100 {
            ExtractUnloadChunkInputs: Ecs
        },
        OnRemovedChunkLoaderScaleMicroMeter100 {
            SendRemovedChunkLoaderEvent: Ecs
        },
        LoadChunksScaleMicroMeter100 {
            ValidateAndLoadAndWait: EcsWhile,
        },
        UnloadChunksScaleMicroMeter100 {
            UnloadAndWait: EcsWhile,
        },

        CategorizeChunksScaleMilliMeter1 {
            Categorize: Ecs,
        },
        OnRemoveChunkLoaderScaleMilliMeter1 {
            ExtractUnloadChunkInputs: Ecs
        },
        OnRemovedChunkLoaderScaleMilliMeter1 {
            SendRemovedChunkLoaderEvent: Ecs
        },
        LoadChunksScaleMilliMeter1 {
            ValidateAndLoadAndWait: EcsWhile,
        },
        UnloadChunksScaleMilliMeter1 {
            UnloadAndWait: EcsWhile,
        },

        CategorizeChunksScaleMilliMeter10 {
            Categorize: Ecs,
        },
        OnRemoveChunkLoaderScaleMilliMeter10 {
            ExtractUnloadChunkInputs: Ecs
        },
        OnRemovedChunkLoaderScaleMilliMeter10 {
            SendRemovedChunkLoaderEvent: Ecs
        },
        LoadChunksScaleMilliMeter10 {
            ValidateAndLoadAndWait: EcsWhile,
        },
        UnloadChunksScaleMilliMeter10 {
            UnloadAndWait: EcsWhile,
        },

        CategorizeChunksScaleMilliMeter100 {
            Categorize: Ecs,
        },
        OnRemoveChunkLoaderScaleMilliMeter100 {
            ExtractUnloadChunkInputs: Ecs
        },
        OnRemovedChunkLoaderScaleMilliMeter100 {
            SendRemovedChunkLoaderEvent: Ecs
        },
        LoadChunksScaleMilliMeter100 {
            ValidateAndLoadAndWait: EcsWhile,
        },
        UnloadChunksScaleMilliMeter100 {
            UnloadAndWait: EcsWhile,
        },

        CategorizeChunksScaleMeter1 {
            Categorize: Ecs,
        },
        OnRemoveChunkLoaderScaleMeter1 {
            ExtractUnloadChunkInputs: Ecs
        },
        OnRemovedChunkLoaderScaleMeter1 {
            SendRemovedChunkLoaderEvent: Ecs
        },
        LoadChunksScaleMeter1 {
            ValidateAndLoadAndWait: EcsWhile,
        },
        UnloadChunksScaleMeter1 {
            UnloadAndWait: EcsWhile,
        },

        CategorizeChunksScaleMeter10 {
            Categorize: Ecs,
        },
        OnRemoveChunkLoaderScaleMeter10 {
            ExtractUnloadChunkInputs: Ecs
        },
        OnRemovedChunkLoaderScaleMeter10 {
            SendRemovedChunkLoaderEvent: Ecs
        },
        LoadChunksScaleMeter10 {
            ValidateAndLoadAndWait: EcsWhile,
        },
        UnloadChunksScaleMeter10 {
            UnloadAndWait: EcsWhile,
        },

        CategorizeChunksScaleMeter100 {
            Categorize: Ecs,
        },
        OnRemoveChunkLoaderScaleMeter100 {
            ExtractUnloadChunkInputs: Ecs
        },
        OnRemovedChunkLoaderScaleMeter100 {
            SendRemovedChunkLoaderEvent: Ecs
        },
        LoadChunksScaleMeter100 {
            ValidateAndLoadAndWait: EcsWhile,
        },
        UnloadChunksScaleMeter100 {
            UnloadAndWait: EcsWhile,
        },

        CategorizeChunksScaleKiloMeter1 {
            Categorize: Ecs,
        },
        OnRemoveChunkLoaderScaleKiloMeter1 {
            ExtractUnloadChunkInputs: Ecs
        },
        OnRemovedChunkLoaderScaleKiloMeter1 {
            SendRemovedChunkLoaderEvent: Ecs
        },
        LoadChunksScaleKiloMeter1 {
            ValidateAndLoadAndWait: EcsWhile,
        },
        UnloadChunksScaleKiloMeter1 {
            UnloadAndWait: EcsWhile,
        },

        CategorizeChunksScaleKiloMeter10 {
            Categorize: Ecs,
        },
        OnRemoveChunkLoaderScaleKiloMeter10 {
            ExtractUnloadChunkInputs: Ecs
        },
        OnRemovedChunkLoaderScaleKiloMeter10 {
            SendRemovedChunkLoaderEvent: Ecs
        },
        LoadChunksScaleKiloMeter10 {
            ValidateAndLoadAndWait: EcsWhile,
        },
        UnloadChunksScaleKiloMeter10 {
            UnloadAndWait: EcsWhile,
        },

        CategorizeChunksScaleKiloMeter100 {
            Categorize: Ecs,
        },
        OnRemoveChunkLoaderScaleKiloMeter100 {
            ExtractUnloadChunkInputs: Ecs
        },
        OnRemovedChunkLoaderScaleKiloMeter100 {
            SendRemovedChunkLoaderEvent: Ecs
        },
        LoadChunksScaleKiloMeter100 {
            ValidateAndLoadAndWait: EcsWhile,
        },
        UnloadChunksScaleKiloMeter100 {
            UnloadAndWait: EcsWhile,
        },

        CategorizeChunksScaleMegaMeter1 {
            Categorize: Ecs,
        },
        OnRemoveChunkLoaderScaleMegaMeter1 {
            ExtractUnloadChunkInputs: Ecs
        },
        OnRemovedChunkLoaderScaleMegaMeter1 {
            SendRemovedChunkLoaderEvent: Ecs
        },
        LoadChunksScaleMegaMeter1 {
            ValidateAndLoadAndWait: EcsWhile,
        },
        UnloadChunksScaleMegaMeter1 {
            UnloadAndWait: EcsWhile,
        },

        CategorizeChunksScaleMegaMeter10 {
            Categorize: Ecs,
        },
        OnRemoveChunkLoaderScaleMegaMeter10 {
            ExtractUnloadChunkInputs: Ecs
        },
        OnRemovedChunkLoaderScaleMegaMeter10 {
            SendRemovedChunkLoaderEvent: Ecs
        },
        LoadChunksScaleMegaMeter10 {
            ValidateAndLoadAndWait: EcsWhile,
        },
        UnloadChunksScaleMegaMeter10 {
            UnloadAndWait: EcsWhile,
        },

        CategorizeChunksScaleMegaMeter100 {
            Categorize: Ecs,
        },
        OnRemoveChunkLoaderScaleMegaMeter100 {
            ExtractUnloadChunkInputs: Ecs
        },
        OnRemovedChunkLoaderScaleMegaMeter100 {
            SendRemovedChunkLoaderEvent: Ecs
        },
        LoadChunksScaleMegaMeter100 {
            ValidateAndLoadAndWait: EcsWhile,
        },
        UnloadChunksScaleMegaMeter100 {
            UnloadAndWait: EcsWhile,
        },

        CategorizeChunksScaleGigaMeter1 {
            Categorize: Ecs,
        },
        OnRemoveChunkLoaderScaleGigaMeter1 {
            ExtractUnloadChunkInputs: Ecs
        },
        OnRemovedChunkLoaderScaleGigaMeter1 {
            SendRemovedChunkLoaderEvent: Ecs
        },
        LoadChunksScaleGigaMeter1 {
            ValidateAndLoadAndWait: EcsWhile,
        },
        UnloadChunksScaleGigaMeter1 {
            UnloadAndWait: EcsWhile,
        },

        CategorizeChunksScaleGigaMeter10 {
            Categorize: Ecs,
        },
        OnRemoveChunkLoaderScaleGigaMeter10 {
            ExtractUnloadChunkInputs: Ecs
        },
        OnRemovedChunkLoaderScaleGigaMeter10 {
            SendRemovedChunkLoaderEvent: Ecs
        },
        LoadChunksScaleGigaMeter10 {
            ValidateAndLoadAndWait: EcsWhile,
        },
        UnloadChunksScaleGigaMeter10 {
            UnloadAndWait: EcsWhile,
        },

        CategorizeChunksScaleGigaMeter100 {
            Categorize: Ecs,
        },
        OnRemoveChunkLoaderScaleGigaMeter100 {
            ExtractUnloadChunkInputs: Ecs
        },
        OnRemovedChunkLoaderScaleGigaMeter100 {
            SendRemovedChunkLoaderEvent: Ecs
        },
        LoadChunksScaleGigaMeter100 {
            ValidateAndLoadAndWait: EcsWhile,
        },
        UnloadChunksScaleGigaMeter100 {
            UnloadAndWait: EcsWhile,
        },

        CategorizeChunksScaleTeraMeter1 {
            Categorize: Ecs,
        },
        OnRemoveChunkLoaderScaleTeraMeter1 {
            ExtractUnloadChunkInputs: Ecs
        },
        OnRemovedChunkLoaderScaleTeraMeter1 {
            SendRemovedChunkLoaderEvent: Ecs
        },
        LoadChunksScaleTeraMeter1 {
            ValidateAndLoadAndWait: EcsWhile,
        },
        UnloadChunksScaleTeraMeter1 {
            UnloadAndWait: EcsWhile,
        },

        CategorizeChunksScaleTeraMeter10 {
            Categorize: Ecs,
        },
        OnRemoveChunkLoaderScaleTeraMeter10 {
            ExtractUnloadChunkInputs: Ecs
        },
        OnRemovedChunkLoaderScaleTeraMeter10 {
            SendRemovedChunkLoaderEvent: Ecs
        },
        LoadChunksScaleTeraMeter10 {
            ValidateAndLoadAndWait: EcsWhile,
        },
        UnloadChunksScaleTeraMeter10 {
            UnloadAndWait: EcsWhile,
        },

        CategorizeChunksScaleTeraMeter100 {
            Categorize: Ecs,
        },
        OnRemoveChunkLoaderScaleTeraMeter100 {
            ExtractUnloadChunkInputs: Ecs
        },
        OnRemovedChunkLoaderScaleTeraMeter100 {
            SendRemovedChunkLoaderEvent: Ecs
        },
        LoadChunksScaleTeraMeter100 {
            ValidateAndLoadAndWait: EcsWhile,
        },
        UnloadChunksScaleTeraMeter100 {
            UnloadAndWait: EcsWhile,
        },

        CategorizeChunksScalePetaMeter1 {
            Categorize: Ecs,
        },
        OnRemoveChunkLoaderScalePetaMeter1 {
            ExtractUnloadChunkInputs: Ecs
        },
        OnRemovedChunkLoaderScalePetaMeter1 {
            SendRemovedChunkLoaderEvent: Ecs
        },
        LoadChunksScalePetaMeter1 {
            ValidateAndLoadAndWait: EcsWhile,
        },
        UnloadChunksScalePetaMeter1 {
            UnloadAndWait: EcsWhile,
        },

        CategorizeChunksScalePetaMeter10 {
            Categorize: Ecs,
        },
        OnRemoveChunkLoaderScalePetaMeter10 {
            ExtractUnloadChunkInputs: Ecs
        },
        OnRemovedChunkLoaderScalePetaMeter10 {
            SendRemovedChunkLoaderEvent: Ecs
        },
        LoadChunksScalePetaMeter10 {
            ValidateAndLoadAndWait: EcsWhile,
        },
        UnloadChunksScalePetaMeter10 {
            UnloadAndWait: EcsWhile,
        },

        CategorizeChunksScalePetaMeter100 {
            Categorize: Ecs,
        },
        OnRemoveChunkLoaderScalePetaMeter100 {
            ExtractUnloadChunkInputs: Ecs
        },
        OnRemovedChunkLoaderScalePetaMeter100 {
            SendRemovedChunkLoaderEvent: Ecs
        },
        LoadChunksScalePetaMeter100 {
            ValidateAndLoadAndWait: EcsWhile,
        },
        UnloadChunksScalePetaMeter100 {
            UnloadAndWait: EcsWhile,
        },

        CategorizeChunksScaleExaMeter1 {
            Categorize: Ecs,
        },
        OnRemoveChunkLoaderScaleExaMeter1 {
            ExtractUnloadChunkInputs: Ecs
        },
        OnRemovedChunkLoaderScaleExaMeter1 {
            SendRemovedChunkLoaderEvent: Ecs
        },
        LoadChunksScaleExaMeter1 {
            ValidateAndLoadAndWait: EcsWhile,
        },
        UnloadChunksScaleExaMeter1 {
            UnloadAndWait: EcsWhile,
        },

        CategorizeChunksScaleExaMeter10 {
            Categorize: Ecs,
        },
        OnRemoveChunkLoaderScaleExaMeter10 {
            ExtractUnloadChunkInputs: Ecs
        },
        OnRemovedChunkLoaderScaleExaMeter10 {
            SendRemovedChunkLoaderEvent: Ecs
        },
        LoadChunksScaleExaMeter10 {
            ValidateAndLoadAndWait: EcsWhile,
        },
        UnloadChunksScaleExaMeter10 {
            UnloadAndWait: EcsWhile,
        },

        CategorizeChunksScaleExaMeter100 {
            Categorize: Ecs,
        },
        OnRemoveChunkLoaderScaleExaMeter100 {
            ExtractUnloadChunkInputs: Ecs
        },
        OnRemovedChunkLoaderScaleExaMeter100 {
            SendRemovedChunkLoaderEvent: Ecs
        },
        LoadChunksScaleExaMeter100 {
            ValidateAndLoadAndWait: EcsWhile,
        },
        UnloadChunksScaleExaMeter100 {
            UnloadAndWait: EcsWhile,
        },

        CategorizeChunksScaleZettaMeter1 {
            Categorize: Ecs,
        },
        OnRemoveChunkLoaderScaleZettaMeter1 {
            ExtractUnloadChunkInputs: Ecs
        },
        OnRemovedChunkLoaderScaleZettaMeter1 {
            SendRemovedChunkLoaderEvent: Ecs
        },
        LoadChunksScaleZettaMeter1 {
            ValidateAndLoadAndWait: EcsWhile,
        },
        UnloadChunksScaleZettaMeter1 {
            UnloadAndWait: EcsWhile,
        },

        CategorizeChunksScaleZettaMeter10 {
            Categorize: Ecs,
        },
        OnRemoveChunkLoaderScaleZettaMeter10 {
            ExtractUnloadChunkInputs: Ecs
        },
        OnRemovedChunkLoaderScaleZettaMeter10 {
            SendRemovedChunkLoaderEvent: Ecs
        },
        LoadChunksScaleZettaMeter10 {
            ValidateAndLoadAndWait: EcsWhile,
        },
        UnloadChunksScaleZettaMeter10 {
            UnloadAndWait: EcsWhile,
        },

        CategorizeChunksScaleZettaMeter100 {
            Categorize: Ecs,
        },
        OnRemoveChunkLoaderScaleZettaMeter100 {
            ExtractUnloadChunkInputs: Ecs
        },
        OnRemovedChunkLoaderScaleZettaMeter100 {
            SendRemovedChunkLoaderEvent: Ecs
        },
        LoadChunksScaleZettaMeter100 {
            ValidateAndLoadAndWait: EcsWhile,
        },
        UnloadChunksScaleZettaMeter100 {
            UnloadAndWait: EcsWhile,
        },

        CategorizeChunksScaleYottaMeter1 {
            Categorize: Ecs,
        },
        OnRemoveChunkLoaderScaleYottaMeter1 {
            ExtractUnloadChunkInputs: Ecs
        },
        OnRemovedChunkLoaderScaleYottaMeter1 {
            SendRemovedChunkLoaderEvent: Ecs
        },
        LoadChunksScaleYottaMeter1 {
            ValidateAndLoadAndWait: EcsWhile,
        },
        UnloadChunksScaleYottaMeter1 {
            UnloadAndWait: EcsWhile,
        },

        CategorizeChunksScaleYottaMeter10 {
            Categorize: Ecs,
        },
        OnRemoveChunkLoaderScaleYottaMeter10 {
            ExtractUnloadChunkInputs: Ecs
        },
        OnRemovedChunkLoaderScaleYottaMeter10 {
            SendRemovedChunkLoaderEvent: Ecs
        },
        LoadChunksScaleYottaMeter10 {
            ValidateAndLoadAndWait: EcsWhile,
        },
        UnloadChunksScaleYottaMeter10 {
            UnloadAndWait: EcsWhile,
        },

        CategorizeChunksScaleYottaMeter100 {
            Categorize: Ecs,
        },
        OnRemoveChunkLoaderScaleYottaMeter100 {
            ExtractUnloadChunkInputs: Ecs
        },
        OnRemovedChunkLoaderScaleYottaMeter100 {
            SendRemovedChunkLoaderEvent: Ecs
        },
        LoadChunksScaleYottaMeter100 {
            ValidateAndLoadAndWait: EcsWhile,
        },
        UnloadChunksScaleYottaMeter100 {
            UnloadAndWait: EcsWhile,
        },

        CategorizeChunksScaleRonnaMeter1 {
            Categorize: Ecs,
        },
        OnRemoveChunkLoaderScaleRonnaMeter1 {
            ExtractUnloadChunkInputs: Ecs
        },
        OnRemovedChunkLoaderScaleRonnaMeter1 {
            SendRemovedChunkLoaderEvent: Ecs
        },
        LoadChunksScaleRonnaMeter1 {
            ValidateAndLoadAndWait: EcsWhile,
        },
        UnloadChunksScaleRonnaMeter1 {
            UnloadAndWait: EcsWhile,
        },

        CategorizeChunksScaleRonnaMeter10 {
            Categorize: Ecs,
        },
        OnRemoveChunkLoaderScaleRonnaMeter10 {
            ExtractUnloadChunkInputs: Ecs
        },
        OnRemovedChunkLoaderScaleRonnaMeter10 {
            SendRemovedChunkLoaderEvent: Ecs
        },
        LoadChunksScaleRonnaMeter10 {
            ValidateAndLoadAndWait: EcsWhile,
        },
        UnloadChunksScaleRonnaMeter10 {
            UnloadAndWait: EcsWhile,
        },

        CategorizeChunksScaleRonnaMeter100 {
            Categorize: Ecs,
        },
        OnRemoveChunkLoaderScaleRonnaMeter100 {
            ExtractUnloadChunkInputs: Ecs
        },
        OnRemovedChunkLoaderScaleRonnaMeter100 {
            SendRemovedChunkLoaderEvent: Ecs
        },
        LoadChunksScaleRonnaMeter100 {
            ValidateAndLoadAndWait: EcsWhile,
        },
        UnloadChunksScaleRonnaMeter100 {
            UnloadAndWait: EcsWhile,
        },

        CategorizeChunksScaleQuettaMeter1 {
            Categorize: Ecs,
        },
        OnRemoveChunkLoaderScaleQuettaMeter1 {
            ExtractUnloadChunkInputs: Ecs
        },
        OnRemovedChunkLoaderScaleQuettaMeter1 {
            SendRemovedChunkLoaderEvent: Ecs
        },
        LoadChunksScaleQuettaMeter1 {
            ValidateAndLoadAndWait: EcsWhile,
        },
        UnloadChunksScaleQuettaMeter1 {
            UnloadAndWait: EcsWhile,
        },

        CategorizeChunksScaleQuettaMeter10 {
            Categorize: Ecs,
        },
        OnRemoveChunkLoaderScaleQuettaMeter10 {
            ExtractUnloadChunkInputs: Ecs
        },
        OnRemovedChunkLoaderScaleQuettaMeter10 {
            SendRemovedChunkLoaderEvent: Ecs
        },
        LoadChunksScaleQuettaMeter10 {
            ValidateAndLoadAndWait: EcsWhile,
        },
        UnloadChunksScaleQuettaMeter10 {
            UnloadAndWait: EcsWhile,
        },

        CategorizeChunksScaleQuettaMeter100 {
            Categorize: Ecs,
        },
        OnRemoveChunkLoaderScaleQuettaMeter100 {
            ExtractUnloadChunkInputs: Ecs
        },
        OnRemovedChunkLoaderScaleQuettaMeter100 {
            SendRemovedChunkLoaderEvent: Ecs
        },
        LoadChunksScaleQuettaMeter100 {
            ValidateAndLoadAndWait: EcsWhile,
        },
        UnloadChunksScaleQuettaMeter100 {
            UnloadAndWait: EcsWhile,
        },

        CategorizeChunksScaleQuettaMeter1000 {
            Categorize: Ecs,
        },
        OnRemoveChunkLoaderScaleQuettaMeter1000 {
            ExtractUnloadChunkInputs: Ecs
        },
        OnRemovedChunkLoaderScaleQuettaMeter1000 {
            SendRemovedChunkLoaderEvent: Ecs
        },
        LoadChunksScaleQuettaMeter1000 {
            ValidateAndLoadAndWait: EcsWhile,
        },
        UnloadChunksScaleQuettaMeter1000 {
            UnloadAndWait: EcsWhile,
        },

        CategorizeChunksScaleQuettaMeter10000 {
            Categorize: Ecs,
        },
        OnRemoveChunkLoaderScaleQuettaMeter10000 {
            ExtractUnloadChunkInputs: Ecs
        },
        OnRemovedChunkLoaderScaleQuettaMeter10000 {
            SendRemovedChunkLoaderEvent: Ecs
        },
        LoadChunksScaleQuettaMeter10000 {
            ValidateAndLoadAndWait: EcsWhile,
        },
        UnloadChunksScaleQuettaMeter10000 {
            UnloadAndWait: EcsWhile,
        },

        CategorizeChunksScaleQuettaMeter100000 {
            Categorize: Ecs,
        },
        OnRemoveChunkLoaderScaleQuettaMeter100000 {
            ExtractUnloadChunkInputs: Ecs
        },
        OnRemovedChunkLoaderScaleQuettaMeter100000 {
            SendRemovedChunkLoaderEvent: Ecs
        },
        LoadChunksScaleQuettaMeter100000 {
            ValidateAndLoadAndWait: EcsWhile,
        },
        UnloadChunksScaleQuettaMeter100000 {
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
        GenerateRenderTextures {
            PrepareRenderExecutor: Ecs,
            GetTextureViews: RenderWhile,
            DispatchRenderTextures: Render,
            WaitForTexturesReady: EcsWhile,
        }
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
