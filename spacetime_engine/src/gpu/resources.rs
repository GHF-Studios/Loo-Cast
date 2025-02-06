use std::collections::HashMap;
use bevy::prelude::*;
use bevy::render::render_resource::{BindGroupLayout, CachedComputePipelineId};

#[derive(Resource, Default)]
pub struct ShaderPipelineRegistry {
    pub shaders: HashMap<String, Handle<Shader>>,
    pub pipelines: HashMap<String, CachedComputePipelineId>,
    pub bind_group_layouts: HashMap<String, BindGroupLayout>,
}