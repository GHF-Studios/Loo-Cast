use crate::bevy::prelude::*;
use crate::bevy::render::render_resource::{BindGroupLayoutDescriptor, CachedComputePipelineId};
use std::collections::HashMap;

#[derive(Resource, Reflect, Default)]
#[reflect(Resource)]
pub struct ShaderRegistry {
    pub shaders: HashMap<String, Handle<Shader>>,
    #[reflect(ignore)]
    pub pipelines: HashMap<String, CachedComputePipelineId>,
    #[reflect(ignore)]
    pub bind_group_layout_descriptors: HashMap<String, BindGroupLayoutDescriptor>,
}
