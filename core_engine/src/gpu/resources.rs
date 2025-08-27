use bevy::prelude::*;
use bevy::render::render_resource::{BindGroupLayout, CachedComputePipelineId};
use std::collections::HashMap;

#[derive(Resource, Reflect, Default)]
#[reflect(Resource)]
pub struct ShaderRegistry {
    pub shaders: HashMap<String, Handle<Shader>>,
    #[reflect(ignore)]
    pub pipelines: HashMap<String, CachedComputePipelineId>,
    #[reflect(ignore)]
    pub bind_group_layouts: HashMap<String, BindGroupLayout>,
}
