//! Shared presentation policy for ordinary game UI and developer UI.
//!
//! This module is intentionally small. It is not an application-state framework;
//! it centralizes only presentation decisions that are genuinely shared.

mod theme;

pub use theme::{UiTextRole, UiTextStyle, UiTheme};

use bevy::prelude::*;

pub struct UiFoundationPlugin;

impl Plugin for UiFoundationPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<UiTheme>();
    }
}
