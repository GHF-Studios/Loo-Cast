//! Reusable in-game test playground.
//!
//! See `ARCHITECTURE.md` for the deliberately small contract between modding,
//! actions, items, inventory state and UI presentation.

mod catalog;
mod input;
mod inventory;
mod items;
mod lifecycle;
mod object;
mod picking;
mod ui;

pub use catalog::{
    AimRay,
    ErasePlaygroundObject,
    PlaygroundCatalog,
    PlaygroundItem,
    PlaygroundItemId,
    UsePlaygroundItem,
};

pub use inventory::{
    CreativeMenuState,
    CursorItem,
    Hotbar,
};

pub use object::{
    PlaygroundPickable,
    PlaygroundRoot,
    ShowHealthInPlaygroundHud,
};

use bevy::prelude::*;

pub struct PlaygroundPlugin;

impl Plugin for PlaygroundPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlaygroundCatalog>()
            .init_resource::<Hotbar>()
            .init_resource::<CreativeMenuState>()
            .init_resource::<CursorItem>()
            .add_message::<UsePlaygroundItem>()
            .add_message::<ErasePlaygroundObject>()
            .add_plugins((
                items::PlaygroundItemsPlugin,
                ui::PlaygroundUiPlugin,
            ));

        input::configure(app);
        lifecycle::configure(app);
    }
}
