//! Reusable in-game test playground.
//!
//! The playground is a game-facing proving adapter over ordinary engine/game
//! resources, components, messages and plugin extension surfaces.
//!
//! ## Module map
//!
//! - `input`: Built-in local input adapter for playground actions.
//! - `items`: Built-in playground item plugins.
//! - `lifecycle`: Manage the lifetime of playground objects and their cleanup.
//! - `map`: Bootstrap for the authored physics playground map.
//! - `object`: Tag playground objects and describe pick/erase operations.
//! - `ui`: Install the playground creative menu, hotbar, and HUD.
//!
//! The plugin or configure entrypoint here wires the child systems and resources.
//!

mod input;
mod items;
mod lifecycle;
mod map;
mod object;
mod ui;

pub use object::{ErasePlaygroundObject, PlaygroundPickable, PlaygroundRoot};

use bevy::prelude::*;

pub struct PlaygroundPlugin;

impl Plugin for PlaygroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<ErasePlaygroundObject>().add_plugins((
            items::PlaygroundItemsPlugin,
            map::PlaygroundMapPlugin,
            ui::PlaygroundUiPlugin,
        ));

        input::configure(app);
        lifecycle::configure(app);
    }
}
