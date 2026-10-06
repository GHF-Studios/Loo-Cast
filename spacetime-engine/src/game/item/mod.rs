//! Generic item identity, metadata, actions and presentation.
//!
//! ## Module map
//!
//! - `action`: Device-agnostic semantic item actions.
//! - `catalog`: Registered semantic item definitions.
//! - `presentation`: Generic UI presentation of item definitions.
//!
//! The plugin or configure entrypoint here wires the child systems and resources.
//!

mod action;
mod catalog;
pub mod presentation;

pub use action::{AimRay, ItemAction, ItemAim, ItemAimContext, UseItemRequest};
pub use catalog::{ItemActionHint, ItemCatalog, ItemDefinition, ItemId};

use bevy::prelude::*;

pub struct ItemPlugin;

impl Plugin for ItemPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ItemCatalog>()
            .init_resource::<ItemAim>()
            .add_message::<UseItemRequest>();

        presentation::configure(app);
    }
}
