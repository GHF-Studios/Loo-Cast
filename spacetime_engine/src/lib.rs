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
use statics::TOKIO_RUNTIME;
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
            .add_systems(PostStartup, post_startup_system);
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
    ASYNC_RUNTIME.spawn_task(|| async move {
        workflow!(gpu, setup_texture_generator, Input {
            shader_name: "texture_generators/example_compute_uv",
            shader_path: "assets/shaders/texture_generators/example_compute_uv.wgsl".to_string(),
        }).await?;

        let texture = workflow!(gpu, generate_texture, Input {
            shader_name: "texture_generators/example_compute_uv",
            texture_size: crate::config::statics::CONFIG.get::<f32>("chunk/size") as usize
        }).await?;

        workflow!(chunk, spawn, Input {
            chunk_coord: (0, 0),
            chunk_owner: None,
            metric_texture: texture,
        }).await?;

        Ok(())
    });
}

fn update_system() {
    use crossbeam_channel::Receiver;

    fn poll_workflow_ecs_ioe(request_rx: Receiver<WorkflowRequest>) {
        while let Ok(request) = request_rx.0.try_recv() {
            tokio::spawn(async move {
                match resolve_workflow!(request.workflow).await {
                    Ok(response) => {
                        request.response_tx.send(result).unwrap();
                    }
                    Err(e) => {
                        panic!("Failed to execute workflow: {:?}", e);
                    }
                };
            });
        }
    }
}
