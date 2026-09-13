//! Shared presentation policy for ordinary game UI and Developer UI.
//!
//! This module is intentionally small. It does not own application state or
//! layout structure; it centralizes only presentation decisions that are
//! genuinely shared: typography/font sources, panel colors and basic spacing.

mod theme;

pub use theme::{UiTextRole, UiTextStyle, UiTheme};

use bevy::prelude::*;

pub struct UiFoundationPlugin;

impl Plugin for UiFoundationPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<UiTheme>();
    }
}
