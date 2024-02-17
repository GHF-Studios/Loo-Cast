use bevy::prelude::*;
use std::collections::*;
use std::sync::*;

pub struct UICommandsPlugin;

impl Plugin for UICommandsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(UICommands::default());
    }
}

#[derive(Resource, Default)]
pub struct UICommands {
}