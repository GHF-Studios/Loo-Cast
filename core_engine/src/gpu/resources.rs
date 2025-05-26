use bevy::prelude::*;
use bevy::render::render_resource::{BindGroupLayout, CachedComputePipelineId};
use std::collections::HashMap;

#[derive(Resource, Default)]
pub struct ShaderRegistry {
    pub shaders: HashMap<String, Handle<Shader>>,
    pub pipelines: HashMap<String, CachedComputePipelineId>,
    pub bind_group_layouts: HashMap<String, BindGroupLayout>,
}
