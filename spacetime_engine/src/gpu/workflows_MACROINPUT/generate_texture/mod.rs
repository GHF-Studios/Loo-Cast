pub mod stages;
pub mod imports {
    crate::workflow_imports_util!();

    use bevy::prelude::{Handle, Res, ResMut, Assets, Image};
    use bevy::render::render_resource::{
        CachedComputePipelineId, BindGroupLayout,
        Buffer, TextureView, TextureDescriptor, Extent3d,
        TextureDimension, TextureFormat, TextureUsages,
        BufferInitDescriptor, BufferUsages, CommandEncoderDescriptor,
        ComputePassDescriptor
    };
    use bevy::ecs::system::SystemState;
    use bevy::render::render_asset::RenderAssets;
    use bevy::render::texture::GpuImage;
    use bevy::render::renderer::{RenderDevice, RenderQueue};
    use bevy::render::render_resource::{PipelineCache, BindGroupEntry, BindingResource};
    use crossbeam_channel::Receiver;

    use crate::gpu::resources::ShaderRegistry;
}
pub mod user_items {
    crate::workflow_user_items_util!();
}

use spacetime_engine_macros::define_workflow;

define_workflow!("GenerateTexture", [
    stage!(Ecs, "PrepareRequest")
    stage!(RenderWhile, "GetTextureView")
    stage!(Render, "DispatchCompute")
    stage!(EcsWhile, "WaitForCompute")
]);