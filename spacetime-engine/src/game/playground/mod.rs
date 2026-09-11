//! Reusable in-game test playground.
//!
//! New mechanics should not edit the menu. They register a catalog entry and
//! react to `UsePlaygroundItem`.

mod catalog;
mod input;
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
    PlaygroundMenuState,
    PlaygroundSelection,
    UsePlaygroundItem,
};

pub use object::{
    PlaygroundPickable,
    PlaygroundRoot,
    ShowHealthInPlaygroundHud,
};

use bevy::prelude::*;

pub struct PlaygroundPlugin;

impl Plugin for PlaygroundPlugin {
    fn build(
        &self,
        app: &mut App,
    ) {
        app.init_resource::<
            PlaygroundCatalog,
        >()
        .init_resource::<
            PlaygroundSelection,
        >()
        .init_resource::<
            PlaygroundMenuState,
        >()
        .add_message::<
            UsePlaygroundItem,
        >()
        .add_message::<
            ErasePlaygroundObject,
        >()
        .add_plugins((
            items::
                PlaygroundItemsPlugin,
            ui::
                PlaygroundUiPlugin,
        ));

        input::configure(
            app,
        );

        lifecycle::configure(
            app,
        );
    }
}
