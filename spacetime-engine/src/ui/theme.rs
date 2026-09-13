use bevy::{
    prelude::*,
    text::{FontSize, FontSource},
};

/// Semantic text roles shared by game UI and Developer UI.
///
/// Roles describe presentation intent, not the surface that happens to use them.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UiTextRole {
    Title,
    Heading,
    Body,
    Compact,
    Data,
    Secondary,
    Tiny,
}

#[derive(Debug, Clone)]
pub struct UiTextStyle {
    pub font: FontSource,
    pub font_size_px: f32,
    pub color: Color,
}

impl UiTextStyle {
    pub fn with_size(mut self, font_size_px: f32) -> Self {
        self.font_size_px = font_size_px;
        self
    }

    pub fn font(&self) -> TextFont {
        TextFont {
            font: self.font.clone(),
            font_size: FontSize::Px(self.font_size_px),
            ..default()
        }
    }

    pub fn color(&self) -> TextColor {
        TextColor(self.color)
    }
}

/// Shared visual policy, not shared UI state.
///
/// Font selection is centralized here so replacing the temporary Bevy default
/// font with a bundled Unicode-capable project font is a single policy change.
/// Both font roles intentionally retain Bevy's default source until such an
/// asset is actually shipped by the project; Stage 6 does not introduce a
/// platform/system-font dependency merely to improve development UI glyphs.
#[derive(Resource, Debug, Clone)]
pub struct UiTheme {
    pub regular_font: FontSource,
    pub data_font: FontSource,
    pub panel_background: Color,
    pub panel_border: Color,
    pub overlay_scrim: Color,
    pub text_primary: Color,
    pub text_secondary: Color,
    pub text_accent: Color,
    pub spacing_px: f32,
    pub panel_padding_px: f32,
}

impl Default for UiTheme {
    fn default() -> Self {
        Self {
            regular_font: FontSource::default(),
            data_font: FontSource::default(),
            panel_background: Color::srgba(0.018, 0.022, 0.034, 0.965),
            panel_border: Color::srgba(0.32, 0.38, 0.50, 0.55),
            overlay_scrim: Color::srgba(0.0, 0.0, 0.0, 0.35),
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
        let (font, font_size_px, color) = match role {
            UiTextRole::Title => (self.regular_font.clone(), 24.0, self.text_primary),
            UiTextRole::Heading => (self.regular_font.clone(), 14.0, self.text_primary),
            UiTextRole::Body => (self.regular_font.clone(), 13.0, self.text_primary),
            UiTextRole::Compact => (self.regular_font.clone(), 10.5, self.text_primary),
            UiTextRole::Data => (self.data_font.clone(), 10.5, self.text_accent),
            UiTextRole::Secondary => (self.regular_font.clone(), 11.0, self.text_secondary),
            UiTextRole::Tiny => (self.regular_font.clone(), 8.5, self.text_secondary),
        };

        UiTextStyle {
            font,
            font_size_px,
            color,
        }
    }
}
