pub mod stages;
pub mod imports {
    crate::workflow_imports_util!();

    use bevy::prelude::{Handle, Shader, Res, ResMut, Assets};
    use bevy::ecs::system::SystemState;
    use bevy::render::render_resource::{
        BindGroupLayout, CachedComputePipelineId,
        PipelineCache, BindGroupLayoutEntry, ShaderStages,
        BindingType, StorageTextureAccess, TextureFormat,
        TextureViewDimension, BufferBindingType, PushConstantRange,
        CachedPipelineState, Pipeline, ComputePipelineDescriptor
    };
    use bevy::render::render_asset::RenderAssets;
    use bevy::render::renderer::RenderDevice;

    use crate::gpu::resources::ShaderRegistry;
}
pub mod user_items {
    crate::workflow_user_items_util!();
}

use spacetime_engine_macros::define_workflow;

define_workflow!("SetupTextureGenerator", [
    stage!(Ecs, "SetupPhase1")
    stage!(RenderWhile, "SetupPhase2")
    stage!(Ecs, "SetupPhase3")
]);