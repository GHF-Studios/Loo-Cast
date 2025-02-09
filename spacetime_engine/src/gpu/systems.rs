
use bevy::prelude::*;
use bevy::ecs::system::SystemState;
use bevy::render::render_resource::PipelineCache;
use bevy::render::renderer::{RenderAdapter, RenderDevice};

pub(in super) fn gpu_startup_system(world: &mut World) {
    let mut system_state: SystemState<(
        Res<RenderDevice>,
        Res<RenderAdapter>,
    )> = SystemState::new(world);
    let (render_device, render_adapter) = system_state.get_mut(world);

    let render_device = render_device.clone();
    let render_adapter = render_adapter.clone();

    world.insert_resource(PipelineCache::new(
        render_device,
        render_adapter,
        false,
    ));
}