use bevy::prelude::*;

#[derive(Resource)]
pub struct GameViewRenderTarget {
    pub image_handle: Handle<Image>,
}