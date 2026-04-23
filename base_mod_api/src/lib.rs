//! base_mod_api
//!
//! `base_mod_api` defines gameplay-facing capability surfaces and plugin composition.
//! Platform/runtime authority remains in `core_mod_api`.

pub use base_mod_macros;
pub use core_mod_api;
pub use core_mod_api::core_mod_macros;

pub mod backend;
pub use backend::{follower, gpu, input, picking, player, render};
pub use core_mod_api::{config, core, debug, logging, reflection, rhai_binding, time, usf, utils, window, workflow};

use core_mod_api::bevy::{app::PluginGroupBuilder, prelude::*};
use core_mod_api::core_mod_macros::register_workflow_mods;
use backend::follower::FollowerPlugin;
use backend::gpu::GpuPlugin;
use backend::input::InputPlugin;
use backend::picking::PickingPlugin;
use backend::player::PlayerPlugin;
use backend::render::RenderPlugin;

pub struct BaseApiPluginGroup;
impl PluginGroup for BaseApiPluginGroup {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(FollowerPlugin)
            .add(GpuPlugin)
            .add(InputPlugin)
            .add(PickingPlugin)
            .add(PlayerPlugin)
            .add(RenderPlugin)
    }
}

register_workflow_mods!(
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
