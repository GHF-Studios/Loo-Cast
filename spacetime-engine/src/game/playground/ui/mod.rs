pub mod creative_menu;
mod hotbar;
mod hud;
mod item_view;

use bevy::prelude::*;

pub struct PlaygroundUiPlugin;

impl Plugin for PlaygroundUiPlugin {
    fn build(&self, app: &mut App) {
        creative_menu::configure(app);
        hud::configure(app);

        app.add_systems(PostUpdate, item_view::sync_item_views);
    }
}
