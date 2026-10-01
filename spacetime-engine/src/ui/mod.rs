//! Shared presentation policy for ordinary game UI and Developer UI.
//!
//! This module is intentionally small. It does not own application state or
//! layout structure; it centralizes only presentation decisions that are
//! genuinely shared: typography/font sources, panel colors and basic spacing.

mod theme;

pub use theme::{UiTextRole, UiTextStyle, UiTheme};

use bevy::prelude::*;

/// Coarse UI attention/occlusion layers.
///
/// Ordinary HUD remains underneath modal/full-attention surfaces. Temporary
/// drag/cursor previews may sit above their owning modal without teaching
/// individual HUD widgets about inventories or other specific interfaces.
pub struct UiLayer;

impl UiLayer {
    pub const HUD: i32 = 0;
    pub const FULL_ATTENTION: i32 = 100;
    pub const DRAG_PREVIEW: i32 = 110;
}

pub struct UiFoundationPlugin;

impl Plugin for UiFoundationPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<UiTheme>();
    }
}
