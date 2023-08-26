pub mod components;
pub mod events;
pub mod resources;
mod systems;

use events::*;

use bevy::prelude::*;

pub struct UniversePlugin;

impl Plugin for UniversePlugin {
    fn build(&self, app: &mut App) {
        app
            // Events
            .add_event::<LoadUniverse>()
            .add_event::<ConfirmLoadedUniverse>();
    }
}
