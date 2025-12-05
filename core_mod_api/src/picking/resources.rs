use bevy::prelude::*;

/// How should the [`SpritePickingPlugin`] handle picking and how should it handle transparent pixels
#[derive(Debug, Clone, Copy, Reflect)]
#[reflect(Debug, Clone)]
pub enum SpritePickingMode {
    /// Even if a sprite is picked on a transparent pixel, it should still count within the backend.
    /// Only consider the rect of a given sprite.
    BoundingBox,
    /// Ignore any part of a sprite which has a lower alpha value than the threshold (inclusive)
    /// Threshold is given as an f32 representing the alpha value in a Bevy Color Value
    AlphaThreshold(f32),
}

#[derive(Resource, Reflect)]
#[reflect(Resource, Default)]
pub struct SpritePickingSettings {
    /// Should the backend count transparent pixels as part of the sprite for picking purposes or should it use the bounding box of the sprite alone.
    ///
    /// Defaults to an inclusive alpha threshold of 0.1
    pub picking_mode: SpritePickingMode,
}

impl Default for SpritePickingSettings {
    fn default() -> Self {
        Self {
            picking_mode: SpritePickingMode::AlphaThreshold(0.1),
        }
    }
}
