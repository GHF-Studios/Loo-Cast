use bevy::prelude::*;
use egui::TextureId;

#[derive(Resource)]
pub struct GameViewRenderTarget {
    pub handle: Handle<Image>,
    pub size: UVec2,
    pub id: TextureId,
}
