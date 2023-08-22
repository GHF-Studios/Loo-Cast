mod main_menu;
mod pause_menu;

use main_menu::MainMenuPlugin;
use pause_menu::PauseMenuPlugin;

use bevy::prelude::*;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app
            // Plugins
            .add_plugins((MainMenuPlugin, PauseMenuPlugin));
    }
}
