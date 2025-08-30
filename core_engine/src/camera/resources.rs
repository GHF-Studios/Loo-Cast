use bevy::prelude::*;
use egui::TextureId;

#[derive(Resource)]
pub struct GameViewRenderTarget {
    pub image_handle: Handle<Image>,
    pub texture_id: TextureId,
}