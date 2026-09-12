//! Reusable in-game test playground.
//!
//! The playground deliberately uses ordinary Bevy resources, components,
//! messages and plugins as its extension surface. See `ARCHITECTURE.md` for the
//! mod-facing contract.

mod action;
mod catalog;
mod input;
mod inventory;
mod items;
mod lifecycle;
mod map;
mod object;
mod picking;
mod ui;

pub use action::{
    AimRay,
    ErasePlaygroundObject,
    PlaygroundAim,
    PlaygroundAimContext,
    PlaygroundItemAction,
    UsePlaygroundItem,
};
pub use catalog::{
    PlaygroundCatalog,
    PlaygroundItem,
    PlaygroundItemId,
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
            .init_resource::<PlaygroundAim>()
            .add_message::<UsePlaygroundItem>()
            .add_message::<ErasePlaygroundObject>()
            .add_plugins((
                items::PlaygroundItemsPlugin,
                map::PlaygroundMapPlugin,
                ui::PlaygroundUiPlugin,
            ));

        input::configure(app);
        lifecycle::configure(app);
    }
}
