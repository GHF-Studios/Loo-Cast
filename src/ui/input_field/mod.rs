pub mod components;
pub mod component_bundles;
pub mod events;
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
            .add_systems(Update, (text_input_system, text_render_system, interact_with_input_field, handle_gained_focus_event, handle_lost_focus_event));
    }
}
