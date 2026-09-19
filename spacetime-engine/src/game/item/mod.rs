//! Generic item identity, metadata, actions and presentation.

mod action;
mod catalog;
pub mod presentation;

pub use action::{AimRay, ItemAction, ItemAim, ItemAimContext, UseItem};
pub use catalog::{ItemCatalog, ItemDefinition, ItemId};

use bevy::prelude::*;

pub struct ItemPlugin;

impl Plugin for ItemPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ItemCatalog>()
            .init_resource::<ItemAim>()
            .add_message::<UseItem>();

        presentation::configure(app);
    }
}
