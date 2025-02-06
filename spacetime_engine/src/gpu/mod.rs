pub mod resources;

pub mod actions;

use bevy::prelude::*;
use resources::*;

pub(in crate) struct ActionPlugin;
impl Plugin for ActionPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(ShaderPipelineRegistry::default());
    }
}