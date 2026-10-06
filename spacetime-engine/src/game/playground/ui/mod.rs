//! Install the playground creative menu, hotbar, and HUD.
//!
//! ## Module map
//!
//! - `creative_menu`: Compose creative catalog, selection, cursor, layout, and hotbar state.
//! - `hotbar`: Spawn and synchronize the playground hotbar presentation.
//! - `hud`: Install playground HUD and context-action presentation.
//!
//! The plugin or configure entrypoint here wires the child systems and resources.
//!

pub mod creative_menu;
mod hotbar;
mod hud;

use bevy::prelude::*;

pub(super) struct PlaygroundUiPlugin;

impl Plugin for PlaygroundUiPlugin {
    fn build(&self, app: &mut App) {
        creative_menu::configure(app);
        hud::configure(app);
    }
}
