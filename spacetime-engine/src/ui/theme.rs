use bevy::prelude::*;

/// Small set of semantic text roles shared by game and developer UI.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UiTextRole {
    Heading,
    Body,
    Data,
    Secondary,
    Tiny,
}

#[derive(Debug, Clone, Copy)]
pub struct UiTextStyle {
    pub font_size_px: f32,
    pub color: Color,
}

/// Shared visual policy, not shared UI state.
///
/// Explicit font handles are intentionally deferred until the project chooses a
/// Unicode-capable font asset and license. Until then consumers keep using Bevy's
/// configured/default font while sharing sizing, spacing and colors.
#[derive(Resource, Debug, Clone)]
pub struct UiTheme {
    pub panel_background: Color,
    pub panel_border: Color,
    pub text_primary: Color,
    pub text_secondary: Color,
    pub text_accent: Color,
    pub spacing_px: f32,
    pub panel_padding_px: f32,
}

impl Default for UiTheme {
    fn default() -> Self {
        Self {
            panel_background: Color::srgba(0.018, 0.022, 0.034, 0.965),
            panel_border: Color::srgba(0.32, 0.38, 0.50, 0.55),
            text_primary: Color::WHITE,
            text_secondary: Color::srgb(0.55, 0.62, 0.72),
            text_accent: Color::srgb(0.78, 0.86, 1.0),
            spacing_px: 6.0,
            panel_padding_px: 12.0,
        }
    }
}

impl UiTheme {
    pub fn text(&self, role: UiTextRole) -> UiTextStyle {
        match role {
            UiTextRole::Heading => UiTextStyle {
                font_size_px: 14.0,
                color: self.text_primary,
            },
            UiTextRole::Body => UiTextStyle {
                font_size_px: 10.5,
                color: self.text_primary,
            },
            UiTextRole::Data => UiTextStyle {
                font_size_px: 10.5,
                color: self.text_accent,
            },
            UiTextRole::Secondary => UiTextStyle {
                font_size_px: 9.5,
                color: self.text_secondary,
            },
            UiTextRole::Tiny => UiTextStyle {
                font_size_px: 8.5,
                color: self.text_secondary,
            },
        }
    }
}
