pub mod components;
pub mod events;
pub mod node_bundles;
mod systems;

use events::*;
use systems::*;

use bevy::prelude::*;

pub struct InputFieldPlugin;

impl Plugin for InputFieldPlugin {
    fn build(&self, app: &mut App) {
        app
            // Initialize Events
            .add_event::<ReceivedInput>()
            // Update Systems
            .add_systems(Update, (text_input_system, text_render_system));
    }
}
