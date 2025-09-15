use bevy::prelude::Reflect;

use crate::config::statics::config;

#[derive(Reflect)]
pub(crate) struct ZoomFactor(pub f32);
impl Default for ZoomFactor {
    fn default() -> Self {
        Self(config().get::<f32>("camera/default_zoom"))
    }
}
