use std::collections::HashMap;
use bevy::prelude::*;
use bevy::render::render_resource::CachedComputePipelineId;

#[derive(Resource, Default)]
pub struct ShaderPipelineRegistry {
    pub shaders: HashMap<String, Handle<Shader>>,
    pub pipelines: HashMap<String, CachedComputePipelineId>,
}