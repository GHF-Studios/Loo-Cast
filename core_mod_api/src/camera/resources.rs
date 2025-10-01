use bevy::prelude::*;
use egui::TextureId;

use crate::config::statics::CONFIG;

/// The current scale of the camera (0 = base, +1 = one scale up, -1 = one down, etc.)
#[derive(Resource, Default, Reflect)]
#[reflect(Resource)]
pub struct ViewScale {
    pub discrete: i32,  // Current scale
    pub offset: f32,    // Fractional offset between this and next (for blending)
}

#[derive(Resource)]
pub struct GameViewRenderTarget {
    pub handle: Handle<Image>,
    pub size: UVec2,
    pub id: TextureId,
}

#[derive(Resource, Reflect)]
#[reflect(Resource)]
pub(crate) struct ZoomFactor(pub f32);
impl Default for ZoomFactor {
    fn default() -> Self {
        Self(CONFIG().get::<f32>("camera/default_zoom"))
    }
}