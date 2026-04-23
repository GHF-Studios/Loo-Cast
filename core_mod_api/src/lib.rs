#![allow(clippy::type_complexity)]
#![allow(clippy::too_many_arguments)]
#![warn(clippy::large_stack_frames)]
#![feature(const_trait_impl)]
#![feature(const_destruct)]
#![feature(fn_traits)]
#![feature(unboxed_closures)]

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
//! - Public API modules are exposed under `facade::*`.
//! - Internal implementation modules live under `backend::*`.
//! - The crate registers workflows and plugin groups; see `CoreApiPluginGroup` and the
//!   `register_workflow_mods!` invocation for the canonical composition approach used by the
//!   engine.
//! - This crate is *code-only*. Canonical assets (configuration files, scripts-as-assets, models,
//!   shaders, and other data) live in the companion `core_mod` crate — `core_mod_api` should not
//!   contain asset files.
//!
//! Documentation style guidance
//! - Keep the crate-level doc focused and short, with links to per-module docs for detail.
//! - Prefer architectural notes and design discussions in `documents/intention_records/` (diagram
//!   atlases) and `documents/markdown_summary/` (focused implementation notes). Avoid long usage examples here until the API is intentionally
//!   stabilized for external consumption.
//! - When adding public APIs, include a concise module-level doc explaining intent, lifecycle
//!   guarantees, and any invariants callers must respect.

pub use bevy_consumable_message;
pub use core_engine_macros;

pub use anymap;
pub use bevy;
#[cfg(all(debug_assertions, not(target_os = "windows")))]
#[allow(unused_imports)]
pub use bevy_dylib;
pub use bevy_egui;
pub use bevy_inspector_egui;
pub use bevy_rapier3d;
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
pub use inventory;
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
pub use rhai;
pub use serde;
pub use serde_json;
pub use thiserror;
pub use tokio;
pub use toml;
pub use tracing;
pub use tracing_appender;
pub use tracing_subscriber;
pub use uuid;
pub use wgpu;

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

pub mod backend;
pub use backend::{chunk, config, core, debug, input, logging, picking, player, reflection, render, rhai_binding, time, usf, utils, window, workflow};

use crate::bevy::{app::PluginGroupBuilder, prelude::*};
use core_engine_macros::register_workflow_mods;

use backend::config::ConfigPlugin;
use backend::core::CorePlugin;
use backend::debug::DebugPlugin;
use backend::logging::LogPlugin;
use backend::rhai_binding::engine::RhaiEnginePlugin;
use backend::time::TimePlugin;
use backend::usf::UsfPlugin;
use backend::utils::UtilsPlugin;
use backend::window::WindowPlugin;
use backend::workflow::WorkflowPlugin;

pub struct CoreApiPluginGroup;
impl PluginGroup for CoreApiPluginGroup {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add_group(WorkflowPlugins)
            .add(CorePlugin)
            .add(ConfigPlugin)
            .add(DebugPlugin)
            .add(LogPlugin)
            .add(RhaiEnginePlugin)
            .add(TimePlugin)
            .add(UsfPlugin)
            .add(UtilsPlugin)
            .add(WindowPlugin)
            .add(WorkflowPlugin)
    }
}

register_workflow_mods!(
    UsfChunk @ chunk {
        SpawnChunks {
            ValidateAndSpawnAndWait: EcsWhile,
        },
        DespawnChunks {
            FindAndDespawnAndWait: EcsWhile,
        },
        ReconcileChunkRealizationArtifacts {
            ResolveIntents: Async,
            ApplyOutputs: EcsWhile,
        },
    },
    Core {
        FinishStartup {
            InsertResource: Ecs,
        },
    },
);
