//! Reusable in-game test playground.
//!
//! The playground deliberately uses ordinary Bevy resources, components,
//! messages and plugins as its extension surface. See `ARCHITECTURE.md` for the
//! mod-facing contract.

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
        app.add_message::<ErasePlaygroundObject>()
            .add_plugins((
                items::PlaygroundItemsPlugin,
                map::PlaygroundMapPlugin,
                ui::PlaygroundUiPlugin,
            ));

        input::configure(app);
        lifecycle::configure(app);
    }
}
