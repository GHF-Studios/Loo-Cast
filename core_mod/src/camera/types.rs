use bevy::prelude::Reflect;

use crate::config::statics::CONFIG;

#[derive(Reflect)]
pub(crate) struct ZoomFactor(pub f32);
impl Default for ZoomFactor {
    fn default() -> Self {
        Self(CONFIG().get::<f32>("camera/default_zoom"))
    }
}
