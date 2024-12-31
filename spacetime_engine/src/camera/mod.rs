use bevy::prelude::*;

pub(in crate) struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(ClearColor(Color::srgb(0.0, 0.0, 0.0)));
    }
}