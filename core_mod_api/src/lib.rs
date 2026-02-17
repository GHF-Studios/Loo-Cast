#![allow(clippy::type_complexity)]
#![allow(clippy::too_many_arguments)]
#![warn(clippy::large_stack_frames)]

#![feature(generic_const_exprs)]

//! core_mod_api
//!
//! The `core_mod_api` crate defines the internal API surface and the core plugin set used by the
//! engine and first-party mods. Its goal is to be a concise, repository-focused contract for
//! engine authors and mod developers working inside this codebase — not a public, stable SDK for
//! external projects (yet).
//!
//! Purpose
//! - Provide the primary plugin composition (`CoreApiPluginGroup`) and well-typed interfaces that
//!   the engine and bundled mods use to interoperate.
//! - Re-export commonly-used dependencies and provide shared utilities and types to reduce
//!   friction across workspace crates.
//!
//! Intended audience & stability
//! - This crate is primarily for internal use by contributors to the engine and official mods.
//! - Treat public items as "internal-first": document intended stability and avoid promising
//!   external backward-compat guarantees unless explicitly marked.
//!
//! Structure, scope & assets
//! - Modules are organized around runtime subsystems: `chunk`, `config`, `core`, `debug`, `gpu`,
//!   `input`, `logging`, `picking`, `player`, `reflection`, `render`, `script`, `time`, `usf`,
//!   `utils`, `window`, and `workflow`.
//! - The crate registers workflows and plugin groups; see `CoreApiPluginGroup` and the
//!   `register_workflow_mods!` invocation for the canonical composition approach used by the
//!   engine.
//! - This crate is *code-only*. Canonical assets (configuration files, scripts-as-assets, models,
//!   shaders, and other data) live in the companion `core_mod` crate — `core_mod_api` should not
//!   contain asset files.
//!
//! Documentation style guidance
//! - Keep the crate-level doc focused and short, with links to per-module docs for detail.
//! - Prefer architectural notes and design discussions in `documents/` (longform) and `docs/`
//!   (curated reference). Avoid long usage examples here until the API is intentionally
//!   stabilized for external consumption.
//! - When adding public APIs, include a concise module-level doc explaining intent, lifecycle
//!   guarantees, and any invariants callers must respect.


pub use bevy_consumable_message;
pub use core_mod_macros;

pub use anymap;
#[cfg(all(debug_assertions, not(target_os = "windows")))]
#[allow(unused_imports)]
pub use bevy_dylib;
pub use bevy;
pub use bevy_egui;
pub use bevy_inspector_egui;
// pub use bevy_rapier2d; // Stuck on bevy 0.17.3
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
// pub use iyes_perf_ui; // Stuck on bevy 0.16.0
pub use lazy_static;
pub use log;
pub use noise;
pub use num_bigint;
pub use num_traits;
pub use once_cell;
pub use parking_lot;
pub use paste;
pub use pin_project_lite;
pub use queues;
pub use radsort;
pub use rand;
pub use serde;
pub use serde_json;
pub use thiserror;
pub use tokio;
pub use toml;
pub use tracing;
pub use tracing_appender;
pub use tracing_subscriber;
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
pub mod access;
pub mod chunk;
pub mod config;
pub mod core;
pub mod debug;
pub mod follower;
pub mod gpu;
pub mod input;
pub mod logging;
pub mod picking;
pub mod player;
pub mod reflection;
pub mod render;
pub mod script;
pub mod time;
pub mod usf;
pub mod utils;
pub mod window;
pub mod workflow;

use crate::bevy::{app::PluginGroupBuilder, prelude::*};
use core_mod_macros::register_workflow_mods;

use chunk::ChunkPlugin;
use config::ConfigPlugin;
use core::CorePlugin;
use debug::DebugPlugin;
use follower::FollowerPlugin;
use gpu::GpuPlugin;
use input::InputPlugin;
use logging::LogPlugin;
use picking::PickingPlugin;
use player::PlayerPlugin;
use reflection::internals::ReflectionPlugin;
use render::RenderPlugin;
use time::TimePlugin;
use usf::UsfPlugin;
use utils::UtilsPlugin;
use window::WindowPlugin;
use workflow::WorkflowPlugin;

pub struct CoreApiPluginGroup;
impl PluginGroup for CoreApiPluginGroup {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add_group(WorkflowPlugins)
            .add(CorePlugin)
            .add(ChunkPlugin)
            .add(ConfigPlugin)
            .add(DebugPlugin)
            .add(FollowerPlugin)
            .add(GpuPlugin)
            .add(InputPlugin)
            .add(LogPlugin)
            .add(PickingPlugin)
            .add(PlayerPlugin)
            .add(ReflectionPlugin)
            .add(RenderPlugin)
            .add(TimePlugin)
            .add(UsfPlugin)
            .add(UtilsPlugin)
            .add(WindowPlugin)
            .add(WorkflowPlugin)
    }
}

register_workflow_mods!(
    Chunk {
        SpawnChunks {
            ValidateAndSpawnAndWait: EcsWhile,
        },
        DespawnChunks {
            FindAndDespawnAndWait: EcsWhile,
        },
    },
    Core {
        FinishStartup {
            InsertResource: Ecs,
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
        GenerateChunkTextures {
            PrepareRenderExecutor: Ecs,
            GetTextureViews: RenderWhile,
            DispatchChunkTextures: Render,
            WaitForTexturesReady: EcsWhile,
            ReadbackTextureData: Ecs,
        }
    },
    Render {
        SpawnCameras {
            SpawnAndWait: EcsWhile,
        },
    },
);
