pub mod creative_menu;
mod hud;

use bevy::prelude::*;

pub struct PlaygroundUiPlugin;

impl Plugin for PlaygroundUiPlugin {
    fn build(
        &self,
        app: &mut App,
    ) {
        creative_menu::configure(
            app,
        );

        hud::configure(
            app,
        );
    }
}
